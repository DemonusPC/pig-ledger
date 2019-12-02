use crate::account::data::{AccountType, DetailedAccount};
use rusqlite::{params, Connection, Result, NO_PARAMS};

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
