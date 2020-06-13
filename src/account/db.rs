use crate::account::data::{Account, AccountType, DetailedAccount};
use crate::account::AccountHierarchyStorage;
use rusqlite::{params, Result, NO_PARAMS};
use std::ops::DerefMut;

// Account Hierarchies

// Expense Hierarchies

fn leaf_to_bool(leaf: i32) -> bool {
    let result = match leaf {
        0 => false,
        1 => true,
        _ => false,
    };

    result
}

pub fn list_account_hierarchies(
    conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>
) -> Result<Vec<AccountHierarchyStorage>> {
    let mut stmt = conn.prepare("SELECT id as h_id, parent, name, child as account_id, (SELECT CASE WHEN child IS NOT NULL THEN (SELECT type from AccountsV2 WHERE id = child) ELSE type END) as acc_type, (SELECT CASE WHEN child IS NOT NULL THEN (SELECT name from AccountsV2 WHERE id = child) ELSE NULL END) as acc_name, (SELECT CASE WHEN child IS NOT NULL THEN (SELECT balance from AccountsV2 WHERE id = child) ELSE NULL END) as balance, (SELECT CASE WHEN child IS NOT NULL THEN (SELECT currency from AccountsV2 WHERE id = child) ELSE NULL END) as currency, leaf FROM AccountHierarchies ORDER BY name DESC")?;

    let accounts = stmt
        .query_map(NO_PARAMS, |row| {
            let account_id: Option<i32> = row.get(3).unwrap();

            let acc_type: i32 = row.get(4).unwrap();

            let acc_name: Option<String> = row.get(5).unwrap();
            let balance: Option<i32> = row.get(6).unwrap();
            let currency: Option<String> = row.get(7).unwrap();

            Ok(AccountHierarchyStorage::new(
                row.get(0).unwrap(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                account_id,
                AccountType::from(acc_type),
                acc_name,
                balance,
                currency,
                leaf_to_bool(row.get(8).unwrap()),
            ))
        })
        .and_then(|mapped_rows| {
            Ok(mapped_rows
                .map(|row| row.unwrap())
                .collect::<Vec<AccountHierarchyStorage>>())
        })?;

    Ok(accounts)
}

// Single Account Operations

pub fn get_account(
    conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    id: i32,
) -> Result<Account> {
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
) -> Result<Vec<Account>> {
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
) -> Result<Vec<DetailedAccount>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use r2d2_sqlite::SqliteConnectionManager;
    use rusqlite::params;

    #[test]
    fn is_able_to_add_and_select_an_account() {
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

        let add_result = add_account(conn, AccountType::Assets, "Dank", "GBP");

        assert!(add_result.is_ok(), true);

        let account = get_account(pool.get().unwrap(), 1).unwrap();

        assert_eq!(account.name, "Dank");
        assert_eq!(account.acc_type, AccountType::Assets);
        assert_eq!(account.currency, "GBP")
    }

    #[test]
    fn can_get_all_accounts() {
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

        let _ = add_account(conn, AccountType::Assets, "Dank", "GBP");
        let _ = add_account(pool.get().unwrap(), AccountType::Expenses, "Food", "GBP");
        let _ = add_account(pool.get().unwrap(), AccountType::Revenue, "Dab", "GBP");

        let accounts: Vec<Account> = list_accounts(pool.get().unwrap()).unwrap();

        assert_eq!(accounts.len(), 3)
    }

    #[test]
    fn can_filter_account_list_by_type() {
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
        let _num = conn.execute(
            "INSERT INTO Currency (code, numeric_code, minor_unit, name) VALUES ('GBP', '826', '2', 'Pound Sterling');",
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

        let _ = add_account(conn, AccountType::Assets, "Dank", "GBP");
        let _ = add_account(pool.get().unwrap(), AccountType::Expenses, "Food", "GBP");
        let _ = add_account(pool.get().unwrap(), AccountType::Revenue, "Dab", "GBP");

        let accounts: Vec<DetailedAccount> =
            list_accounts_filter_type(pool.get().unwrap(), AccountType::Assets).unwrap();

        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].name, "Dank");
    }
}
