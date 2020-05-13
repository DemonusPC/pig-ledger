use crate::transaction::data::{Entry, EntryType, EntryV2, Transaction, TransactionV2};
use rusqlite::{params, Result, NO_PARAMS};

use chrono::{DateTime, Utc};
use std::ops::DerefMut;

// V2
pub fn get_transaction_v2(
    conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    id: i32,
) -> Result<TransactionV2> {
    let mut stmt = conn.prepare("SELECT id, date, name from Transactions WHERE id = ?1")?;

    let metadata = stmt.query_row(params![id], |row| {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?))
    })?;

    let mut entry_stmt = conn.prepare(
        "
        SELECT c.id, c.account, a.name, c.transaction_id, c.balance, 0 as entry_type FROM Credits as c INNER JOIN Accounts as a ON c.account = a.id WHERE c.transaction_id = ?1
        UNION ALL
        SELECT d.id, d.account, a.name, d.transaction_id, d.balance, 1 as entry_type FROM Debits as d INNER JOIN Accounts as a ON d.account = a.id WHERE d.transaction_id = ?1",
    )?;

    let entries = entry_stmt
        .query_map(params![id], |row| {
            Ok(EntryV2::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                EntryType::from_i32(row.get(5)?),
            ))
        })
        .and_then(|mapped_rows| {
            Ok(mapped_rows
                .map(|row| row.unwrap())
                .collect::<Vec<EntryV2>>())
        })?;

    if entries.len() % 2 != 0 {
        panic!("Uneven number of entries. Integrity damaged");
    }

    Ok(TransactionV2::new(
        metadata.0, metadata.1, metadata.2, entries,
    ))
}

// List database functions

pub fn list_transactions(
    conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
) -> Result<Vec<Transaction>> {
    let mut stmt = conn.prepare("SELECT id, date, name from Transactions ORDER BY date DESC LIMIT 32")?;

    let transactions = stmt
        .query_map(NO_PARAMS, |row| {
            Ok(Transaction {
                id: row.get(0).unwrap(),
                date: row.get(1).unwrap(),
                name: row.get(2).unwrap(),
            })
        })
        .and_then(|mapped_rows| {
            Ok(mapped_rows
                .map(|row| row.unwrap())
                .collect::<Vec<Transaction>>())
        })?;

    Ok(transactions)
}

pub fn get_entries_for_transaction(
    conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    id: i32,
) -> Result<Vec<Entry>> {
    let mut stmt = conn.prepare(
        "
        SELECT c.id, c.account, a.name, c.transaction_id, c.balance, 0 as entry_type FROM Credits as c INNER JOIN Accounts as a ON c.account = a.id WHERE c.transaction_id = ?1
        UNION ALL
        SELECT d.id, d.account, a.name, d.transaction_id, d.balance, 1 as entry_type FROM Debits as d INNER JOIN Accounts as a ON d.account = a.id WHERE d.transaction_id = ?1",
    )?;

    let result = stmt
        .query_map(params![id], |row| {
            Ok(Entry {
                id: row.get(0).unwrap(),
                account: row.get(1).unwrap(),
                account_name: row.get(2).unwrap(),
                transaction_id: row.get(3).unwrap(),
                balance: row.get(4).unwrap(),
                entry_type: EntryType::from_i32(row.get(5).unwrap()),
            })
        })
        .and_then(|mapped_rows| Ok(mapped_rows.map(|row| row.unwrap()).collect::<Vec<Entry>>()))?;

    if result.len() % 2 != 0 {
        panic!("Uneven number of entries. Integrity damaged");
    }

    Ok(result)
}

pub fn list_transactions_date(
    conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    year: i32,
    month: u8,
) -> Result<Vec<Transaction>> {
    let mut stmt = conn.prepare(
        "SELECT id, date, name from Transactions
        WHERE CAST(strftime('%m', date) as integer) = ?1 
        AND CAST(strftime('%Y', date) as integer) = ?2 
        ORDER BY date DESC",
    )?;

    let transactions = stmt
        .query_map(params![month, year], |row| {
            Ok(Transaction {
                id: row.get(0).unwrap(),
                date: row.get(1).unwrap(),
                name: row.get(2).unwrap(),
            })
        })
        .and_then(|mapped_rows| {
            Ok(mapped_rows
                .map(|row| row.unwrap())
                .collect::<Vec<Transaction>>())
        })?;

    Ok(transactions)
}

pub fn list_transactions_year(
    conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    year: i32,
) -> Result<Vec<Transaction>> {
    let mut stmt = conn.prepare(
        "SELECT id, date, name from Transactions
        WHERE CAST(strftime('%Y', date) as integer) = ?1 
        ORDER BY date DESC",
    )?;

    let transactions = stmt
        .query_map(params![year], |row| {
            Ok(Transaction {
                id: row.get(0).unwrap(),
                date: row.get(1).unwrap(),
                name: row.get(2).unwrap(),
            })
        })
        .and_then(|mapped_rows| {
            Ok(mapped_rows
                .map(|row| row.unwrap())
                .collect::<Vec<Transaction>>())
        })?;

    Ok(transactions)
}

// Single transactions functions

pub fn get_transaction(
    conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    id: i32,
) -> Result<Transaction> {
    let mut stmt = conn.prepare("SELECT id, date, name from Transactions WHERE id = ?1")?;

    stmt.query_row(params![id], |row| {
        Ok(Transaction {
            id: row.get(0).unwrap(),
            date: row.get(1).unwrap(),
            name: row.get(2).unwrap(),
        })
    })
}

pub fn create_transaction(
    mut conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    debit_account: i32,
    credit_account: i32,
    balance: i32,
    name: &str,
) -> Result<i64> {
    let con = conn.deref_mut();
    let tx = con.transaction()?;
    let date: DateTime<Utc> = Utc::now();

    tx.execute(
        "INSERT INTO Transactions (date, name) VALUES (?1, ?2)",
        params![date, name],
    )?;

    let transaction_id = tx.last_insert_rowid();

    tx.execute(
        "INSERT INTO Debits (account, transaction_id, balance) VALUES (?1, ?2, ?3)",
        params![debit_account, transaction_id, balance],
    )?;
    tx.execute(
        "INSERT INTO Credits (account, transaction_id, balance) VALUES (?1, ?2, ?3)",
        params![credit_account, transaction_id, balance],
    )?;

    let transaction_result = tx.commit();

    match transaction_result {
        Ok(_) => Ok(transaction_id),
        Err(_) => panic!("Transaction has failed"),
    }
}

pub fn remove_transaction(
    mut conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    id: i32,
) -> Result<()> {
    let con = conn.deref_mut();
    let tx = con.transaction()?;

    tx.execute("DELETE FROM Transactions WHERE id = ?1", params![id])?;

    tx.commit()
}

pub fn update_transaction(
    mut conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    transaction_id: i32,
    balance: i32,
    name: &str,
) -> Result<()> {
    let con = conn.deref_mut();
    let tx = con.transaction()?;

    tx.execute(
        "
        UPDATE Transactions SET name = ?1 WHERE id = ?2;
        ",
        params![name, transaction_id],
    )?;

    tx.execute(
        "
        UPDATE Credits SET balance = ?1 WHERE transaction_id = ?2;
        ",
        params![balance, transaction_id],
    )?;

    tx.execute(
        "
        UPDATE Debits SET balance = ?1  WHERE transaction_id = ?2;
        ",
        params![balance, transaction_id],
    )?;

    tx.commit()
}

#[cfg(test)]
mod tests {
    use super::*;
    use r2d2_sqlite::SqliteConnectionManager;
    use rusqlite::params;

    fn create_base(conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>) {
        let _ = conn.execute(
            "CREATE TABLE \"Currency\" (
            \"code\"	TEXT NOT NULL UNIQUE,
            \"numeric_code\"	INTEGER NOT NULL UNIQUE,
            \"minor_unit\"	INTEGER NOT NULL DEFAULT 2,
            \"name\"	TEXT NOT NULL UNIQUE,
            PRIMARY KEY(\"code\")
            )",
            params![],
        );
        let _num = conn.execute(
            "INSERT INTO Currency (code, numeric_code, minor_unit, name) VALUES ('GBP', '826', '2', 'Pound Sterling');",
            params![],
        );

        let _ = conn.execute(
            "CREATE TABLE \"Accounts\" (
	        \"id\"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	        \"type\"	INTEGER NOT NULL,
	        \"name\"	TEXT NOT NULL,
	        \"currency\"	TEXT NOT NULL,
	        FOREIGN KEY(\"currency\") REFERENCES \"Currency\"(\"code\")
            )",
            params![],
        );

        let _num = conn.execute(
            "INSERT INTO Accounts (type, name, currency) VALUES (0, \"Current\", \"GBP\")",
            params![],
        );

        let _num = conn.execute(
            "INSERT INTO Accounts (type, name, currency) VALUES (1, \"Expenses\", \"GBP\")",
            params![],
        );

        let _ = conn.execute(
            "CREATE TABLE \"Transactions\" (
	        \"id\"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	        \"date\"	TEXT NOT NULL,
	        \"name\"	TEXT
            )",
            params![],
        );

        let _ = conn.execute(
            "CREATE TABLE \"Credits\" (
	        \"id\"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	        \"account\"	INTEGER NOT NULL,
	        \"transaction_id\"	INTEGER NOT NULL,
	        \"balance\"	INTEGER NOT NULL DEFAULT 0 CHECK (typeof(\"balance\") = 'integer'),
	        FOREIGN KEY(\"account\") REFERENCES \"Accounts\"(\"id\"),
	        FOREIGN KEY(\"transaction_id\") REFERENCES \"Transactions\"(\"id\") ON DELETE CASCADE
            )",
            params![],
        );

        let _ = conn.execute(
            "CREATE TABLE \"Debits\" (
	        \"id\"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	        \"account\"	INTEGER NOT NULL,
	        \"transaction_id\"	INTEGER NOT NULL,
	        \"balance\"	INTEGER NOT NULL DEFAULT 0 CHECK (typeof(\"balance\") = 'integer'),
	        FOREIGN KEY(\"account\") REFERENCES \"Accounts\"(\"id\"),
	        FOREIGN KEY(\"transaction_id\") REFERENCES \"Transactions\"(\"id\") ON DELETE CASCADE
            )",
            params![],
        );
    }

    #[test]
    fn create_transaction_test() {
        let manager = SqliteConnectionManager::memory();
        let pool = r2d2::Pool::new(manager).unwrap();
        create_base(pool.get().unwrap());

        let id = create_transaction(pool.get().unwrap(), 1, 2, 50, "Super Payment");

        assert_eq!(id.unwrap(), 1);
    }
}
