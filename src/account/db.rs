use crate::account::data::{Account, AccountType, DetailedAccount};
use rusqlite::{params, Result, NO_PARAMS};
use std::ops::DerefMut;

// Single Account Operations

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

// List Operations

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
) -> Result<(Vec<DetailedAccount>)> {
    let mut stmt = conn.prepare("SELECT Accounts.id, Accounts.type, Accounts.name, Accounts.currency, 
    (SELECT ifnull(SUM(balance),0) as \"Debits\" FROM Debits WHERE Debits.account = Accounts.id) - 
    (SELECT ifnull(SUM(balance),0) as \"Credits\" FROM Credits WHERE Credits.account = Accounts.id) as \"balance\"
    from Accounts WHERE type = ?1
    ")?;

    let accounts = stmt
        .query_map(params![account_type as i32], |row| {
            Ok(DetailedAccount {
                id: row.get(0).unwrap(),
                acc_type: AccountType::from_i32(row.get(1).unwrap()),
                name: row.get(2).unwrap(),
                currency: row.get(3).unwrap(),
                balance: row.get(4).unwrap(),
            })
        })
        .and_then(|mapped_rows| {
            Ok(mapped_rows
                .map(|row| row.unwrap())
                .collect::<Vec<DetailedAccount>>())
        })?;

    Ok(accounts)
}
