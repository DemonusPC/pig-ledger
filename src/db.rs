use rusqlite::{Connection, Result, params, NO_PARAMS};
use crate::datastruct::{Account, AccountType, Transaction, SqlResult, Entry};

use chrono::{DateTime, Utc};

pub fn list_accounts(conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>) -> Result<(Vec<Account>)> {
    let mut stmt = conn.prepare("SELECT id, type, name from Accounts")?;

    let accounts = stmt.query_map(NO_PARAMS, |row|
        Ok(Account {
            id: row.get(0).unwrap(),
            acc_type: AccountType::from_i32(row.get(1).unwrap()),
            name: row.get(2).unwrap(),
        }))
        .and_then(|mapped_rows| { 
            Ok(mapped_rows
                .map(|row| row.unwrap())
                .collect::<Vec<Account>>())
        })?;

    Ok(accounts)
}

pub fn list_transactions() -> Result<(Vec<Transaction>)> {
    let conn = Connection::open("ledger.db")?;
    let mut stmt = conn.prepare("SELECT id, date, name from Transactions")?;

    let transactions = stmt.query_map(NO_PARAMS, |row|
        Ok(Transaction {
            id: row.get(0).unwrap(),
            date: row.get(1).unwrap(),
            name: row.get(2).unwrap(),
        }))
        .and_then(|mapped_rows| { 
            Ok(mapped_rows
                .map(|row| row.unwrap())
                .collect::<Vec<Transaction>>())
        })?;

    Ok(transactions)
}

pub fn get_account(account : &str) -> Result<(Account)> {
    let conn = Connection::open("ledger.db")?;
    let mut stmt = conn.prepare("SELECT id, type, name FROM Accounts WHERE name = ?1")?;


    stmt.query_row(params![account], |row|
       Ok(Account {
            id: row.get(0).unwrap(),
            acc_type: AccountType::from_i32(row.get(1).unwrap()),
            name: row.get(2).unwrap(),
        }))
}

pub fn get_account_by_id(id : i32) -> Result<(Account)> {
    let conn = Connection::open("ledger.db")?;
    let mut stmt = conn.prepare("SELECT id, type, name FROM Accounts WHERE id = ?1")?;


    stmt.query_row(params![id], |row|
       Ok(Account {
            id: row.get(0).unwrap(),
            acc_type: AccountType::from_i32(row.get(1).unwrap()),
            name: row.get(2).unwrap(),
        }))
}

pub fn get_transaction(id : i32) -> Result<(Transaction)> {
    let conn = Connection::open("ledger.db")?;
    let mut stmt = conn.prepare("SELECT id, date, name from Transactions WHERE id = ?1")?;


    stmt.query_row(params![id], |row|
       Ok(Transaction {
           id: row.get(0).unwrap(),
           date: row.get(1).unwrap(),
           name: row.get(2).unwrap(),
       }))
}

pub fn add_account(acc_type : AccountType, name : &str) -> Result<()> {
    let mut conn = Connection::open("ledger.db")?;

    let tx = conn.transaction()?;

    tx.execute("INSERT INTO Accounts (Type, Name) VALUES (?1, ?2)", params![acc_type as i32, name])?;

    tx.commit()
}

pub fn remove_account(name : &str) -> Result<()> {
    let mut conn = Connection::open("ledger.db")?;

    let tx = conn.transaction()?;

    tx.execute("DELETE FROM Accounts WHERE Name = ?1", params![name])?;

    tx.commit()
}

pub fn remove_account_by_id(id : i32) -> Result<()> {
    let mut conn = Connection::open("ledger.db")?;

    let tx = conn.transaction()?;

    tx.execute("DELETE FROM Accounts WHERE id = ?1", params![id])?;

    tx.commit()
}

pub fn transaction(debit_account : i32, credit_account : i32, balance : f64, name: &str) -> Result<()> {
    let mut conn = Connection::open("ledger.db")?;
    let tx = conn.transaction()?;

    let date : DateTime<Utc> = Utc::now(); 

    tx.execute("INSERT INTO Transactions (date, name) VALUES (?1, ?2)", params![date, name])?;

    let transaction_id = tx.last_insert_rowid();

    tx.execute("INSERT INTO Debits (account, transaction_id, balance) VALUES (?1, ?2, ?3)", params![debit_account, transaction_id, balance])?;
    tx.execute("INSERT INTO Credits (account, transaction_id, balance) VALUES (?1, ?2, ?3)", params![credit_account, transaction_id, balance])?;
    
    tx.commit()
}

pub fn remove_transaction(id : i32) -> Result<()> {
    let mut conn = Connection::open("ledger.db")?;
    let tx = conn.transaction()?;

    tx.execute("DELETE FROM Transactions WHERE id = ?1", params![id])?;
    
    tx.commit()
}

// SELECT SUM(c.balance) - SUM(d.balance) FROM Credits as c, Debits as d;
pub fn check_integrity() -> Result<bool> {
    let conn = Connection::open("ledger.db")?;
    let mut stmt = conn.prepare("SELECT SUM(c.balance) - SUM(d.balance) FROM Credits as c, Debits as d")?;

    let query = stmt.query_map(NO_PARAMS, |row|
        Ok(SqlResult {
            value: row.get(0).unwrap(),
        }))
        .and_then(|mapped_rows| { 
            Ok(mapped_rows
                .map(|row| row.unwrap())
                .collect::<Vec<SqlResult>>())
        })?;

    let result = if query[0].value == 0. { true } else { false };
    

    Ok(result)
}

pub fn current_balance(account : i32 ) -> Result<(SqlResult)> {
    let conn = Connection::open("ledger.db")?;
    
    let mut stmt = conn.prepare("SELECT (SELECT ifnull(SUM(balance),0) as \"Debits\" FROM Debits WHERE account = ?1) - (SELECT ifnull(SUM(balance),0) as \"Credits\" FROM Credits WHERE account = ?1)")?;
    
    stmt.query_row(params![account], |row|
       Ok(SqlResult {
            value: row.get(0).unwrap()
    }))

}

pub fn get_debit(id : i32) -> Result<(Entry)> {
    let conn = Connection::open("ledger.db")?;
    let mut stmt = conn.prepare("SELECT id, account, transaction_id, balance from Debits WHERE transaction_id = ?1;")?;


    stmt.query_row(params![id], |row|
       Ok(Entry {
           id: row.get(0).unwrap(),
           account: row.get(1).unwrap(),
           transaction_id: row.get(2).unwrap(),
           balance: row.get(2).unwrap(),
       }))
}

pub fn get_credit(id : i32) -> Result<(Entry)> {
    let conn = Connection::open("ledger.db")?;
    let mut stmt = conn.prepare("SELECT id, account, transaction_id, balance from Credits WHERE transaction_id = ?1;")?;


    stmt.query_row(params![id], |row|
       Ok(Entry {
           id: row.get(0).unwrap(),
           account: row.get(1).unwrap(),
           transaction_id: row.get(2).unwrap(),
           balance: row.get(2).unwrap(),
       }))
}


// SELECT t.date, t.name,  c.account as "from", c.balance as "Credit", d.account as "to",  d.balance as "Debit" FROM Transactions as t LEFT JOIN Debits as d ON d.transaction_id = t.id LEFT JOIN Credits as c ON c.transaction_id = t.id WHERE t.id = 8;