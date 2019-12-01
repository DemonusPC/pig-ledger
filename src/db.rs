use crate::datastruct::{Account, AccountType, Currency, Entry, EntryType, SqlResult, Transaction};
use rusqlite::{params, Connection, Result, NO_PARAMS};

use chrono::{DateTime, Utc};
use std::ops::DerefMut;

pub fn list_accounts(
    conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
) -> Result<(Vec<Account>)> {
    let mut stmt = conn.prepare("SELECT id, type, name, currency from Accounts")?;

    let accounts = stmt
        .query_map(NO_PARAMS, |row| {
            Ok(Account {
                id: row.get(0).unwrap(),
                acc_type: AccountType::from_i32(row.get(1).unwrap()),
                name: row.get(2).unwrap(),
                currency: row.get(3).unwrap(),
            })
        })
        .and_then(|mapped_rows| {
            Ok(mapped_rows
                .map(|row| row.unwrap())
                .collect::<Vec<Account>>())
        })?;

    Ok(accounts)
}

pub fn list_accounts_filter_type(
    conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    account_type: AccountType,
) -> Result<(Vec<Account>)> {
    let mut stmt = conn.prepare("SELECT id, type, name, currency from Accounts WHERE type = ?1")?;

    let accounts = stmt
        .query_map(params![account_type as i32], |row| {
            Ok(Account {
                id: row.get(0).unwrap(),
                acc_type: AccountType::from_i32(row.get(1).unwrap()),
                name: row.get(2).unwrap(),
                currency: row.get(3).unwrap(),
            })
        })
        .and_then(|mapped_rows| {
            Ok(mapped_rows
                .map(|row| row.unwrap())
                .collect::<Vec<Account>>())
        })?;

    Ok(accounts)
}

pub fn list_transactions() -> Result<(Vec<Transaction>)> {
    let conn = Connection::open("ledger.db")?;
    let mut stmt = conn.prepare("SELECT id, date, name from Transactions ORDER BY date DESC")?;

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

pub fn list_transactions_date(
    conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    month: u8,
    year: i32,
) -> Result<(Vec<Transaction>)> {
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

pub fn get_account(
    conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    id: i32,
) -> Result<(Account)> {
    let mut stmt = conn.prepare("SELECT id, type, name, currency FROM Accounts WHERE id = ?1")?;

    stmt.query_row(params![id], |row| {
        Ok(Account {
            id: row.get(0).unwrap(),
            acc_type: AccountType::from_i32(row.get(1).unwrap()),
            name: row.get(2).unwrap(),
            currency: row.get(3).unwrap(),
        })
    })
}

pub fn get_transaction(
    conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    id: i32,
) -> Result<(Transaction)> {
    let mut stmt = conn.prepare("SELECT id, date, name from Transactions WHERE id = ?1")?;

    stmt.query_row(params![id], |row| {
        Ok(Transaction {
            id: row.get(0).unwrap(),
            date: row.get(1).unwrap(),
            name: row.get(2).unwrap(),
        })
    })
}

pub fn add_account(
    mut conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    acc_type: AccountType,
    name: &str,
    currency: &str,
) -> Result<()> {
    let con = conn.deref_mut();
    let tx = con.transaction()?;

    tx.execute(
        "INSERT INTO Accounts (type, name, currency) VALUES (?1, ?2, ?3)",
        params![acc_type as i32, name, currency],
    )?;

    tx.commit()
}

pub fn remove_account(
    mut conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    id: i32,
) -> Result<()> {
    let con = conn.deref_mut();
    let tx = con.transaction()?;

    tx.execute("DELETE FROM Accounts WHERE id = ?1", params![id])?;

    tx.commit()
}

pub fn transaction(
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

// SELECT SUM(c.balance) - SUM(d.balance) FROM Credits as c, Debits as d;
pub fn check_integrity(
    conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
) -> Result<bool> {
    let mut stmt =
        conn.prepare("SELECT SUM(c.balance) - SUM(d.balance) FROM Credits as c, Debits as d")?;

    let query = stmt
        .query_map(NO_PARAMS, |row| {
            Ok(SqlResult {
                value: row.get(0).unwrap(),
            })
        })
        .and_then(|mapped_rows| {
            Ok(mapped_rows
                .map(|row| row.unwrap())
                .collect::<Vec<SqlResult>>())
        })?;

    let result = if query[0].value == 0. { true } else { false };

    Ok(result)
}

pub fn current_balance(
    conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    account: i32,
) -> Result<(SqlResult)> {
    let mut stmt = conn.prepare("SELECT (SELECT ifnull(SUM(balance),0) as \"Debits\" FROM Debits WHERE account = ?1) - (SELECT ifnull(SUM(balance),0) as \"Credits\" FROM Credits WHERE account = ?1)")?;

    stmt.query_row(params![account], |row| {
        Ok(SqlResult {
            value: row.get(0).unwrap(),
        })
    })
}

pub fn get_entries(
    conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    id: i32,
) -> Result<(Vec<Entry>)> {
    let mut stmt = conn.prepare(
        "SELECT id, account, transaction_id, balance, 0 as entry_type FROM Credits WHERE transaction_id = ?1
        UNION ALL
        SELECT id, account, transaction_id, balance, 1 as entry_type FROM Debits WHERE transaction_id = ?1;",
    )?;

    let result = stmt
        .query_map(params![id], |row| {
            Ok(Entry {
                id: row.get(0).unwrap(),
                account: row.get(1).unwrap(),
                transaction_id: row.get(2).unwrap(),
                balance: row.get(3).unwrap(),
                entry_type: EntryType::from_i32(row.get(4).unwrap()),
            })
        })
        .and_then(|mapped_rows| Ok(mapped_rows.map(|row| row.unwrap()).collect::<Vec<Entry>>()))?;

    if result.len() % 2 != 0 {
        panic!("Uneven number of entries. Integrity damaged");
    }

    Ok(result)
}

// SELECT t.date, t.name,  c.account as "from", c.balance as "Credit", d.account as "to",  d.balance as "Debit" FROM Transactions as t LEFT JOIN Debits as d ON d.transaction_id = t.id LEFT JOIN Credits as c ON c.transaction_id = t.id WHERE t.id = 8;

pub fn list_currencies(
    conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
) -> Result<(Vec<Currency>)> {
    let mut stmt = conn.prepare("SELECT code, numeric_code, minor_unit, name FROM Currency")?;

    let result = stmt
        .query_map(NO_PARAMS, |row| {
            Ok(Currency {
                code: row.get(0).unwrap(),
                numeric_code: row.get(1).unwrap(),
                minor_unit: row.get(2).unwrap(),
                name: row.get(3).unwrap(),
            })
        })
        .and_then(|mapped_rows| {
            Ok(mapped_rows
                .map(|row| row.unwrap())
                .collect::<Vec<Currency>>())
        })?;

    Ok(result)
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

        let id = transaction(pool.get().unwrap(), 1, 2, 50, "Super Payment");

        assert_eq!(id.unwrap(), 1);
    }

    #[test]
    fn lists_currencies_returns_struct() {
        let manager = SqliteConnectionManager::memory();
        let pool = r2d2::Pool::new(manager).unwrap();
        let conn = pool.get().unwrap();

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

        let num = conn.execute(
            "INSERT INTO Currency (code, numeric_code, minor_unit, name) VALUES ('GBP', '826', '2', 'Pound Sterling');",
            params![],
        );

        assert_eq!(num.unwrap(), 1);

        let expected = Currency {
            code: String::from("GBP"),
            numeric_code: 826,
            minor_unit: 2,
            name: String::from("Pound Sterling"),
        };
        let result = list_currencies(conn).unwrap();

        assert_eq!(expected, result[0]);
    }

    #[test]
    fn list_currencies_can_return_multiple_currencies() {
        let manager = SqliteConnectionManager::memory();
        let pool = r2d2::Pool::new(manager).unwrap();
        let conn = pool.get().unwrap();

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

        let num = conn.execute(
            "INSERT INTO Currency (code, numeric_code, minor_unit, name) VALUES 
            ('GBP', '826', '2', 'Pound Sterling'),
            ('EUR', '978', '2', 'Euro'),
            ('PLN', '985', '2', 'Zloty');",
            params![],
        );
        assert_eq!(num.unwrap(), 3);

        let result = list_currencies(conn).unwrap();

        assert_eq!(result.len(), 3)
    }
}
