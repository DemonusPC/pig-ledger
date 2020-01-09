use crate::transaction::data::{Entry, EntryType, Transaction};
use rusqlite::{params, Result, NO_PARAMS};

pub fn list_transactions(
    conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
) -> Result<Vec<Transaction>> {
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

pub fn get_entries_for_transaction(
    conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    id: i32,
) -> Result<Vec<Entry>> {
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

pub fn list_transactions_date(
    conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    month: u8,
    year: i32,
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
