use crate::budget::data::{Budget, BudgetEntry, NewBudgetEntry};
use crate::datastruct::SqlResult;
use chrono::Utc;
use rusqlite::{params, Result};
use std::ops::DerefMut;

pub fn get_budget(
    conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    id: i32,
) -> Result<Budget> {
    let mut stmt = conn.prepare("SELECT id, name, open, close FROM Budgets WHERE id = ?1")?;

    stmt.query_row(params![id], |row| {
        Ok(Budget::new(
            row.get(0).unwrap(),
            &row.get(1).unwrap(),
            row.get(2).unwrap(),
            row.get(3).unwrap(),
        ))
    })
}

pub fn remove_budget(
    mut conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    id: i32,
) -> Result<()> {
    let con = conn.deref_mut();
    let tx = con.transaction()?;

    tx.execute("DELETE FROM Budgets WHERE id = ?1", params![id])?;

    tx.commit()
}

pub fn create_budget(
    mut conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    budget: &Budget,
) -> Result<i64> {
    let con = conn.deref_mut();
    let tx = con.transaction()?;

    tx.execute(
        "INSERT INTO Budgets (name, open, close) VALUES (?1, ?2, ?3)",
        params![budget.name, budget.open, budget.close],
    )?;

    let budget_id = tx.last_insert_rowid();

    let transaction_result = tx.commit();

    match transaction_result {
        Ok(_) => Ok(budget_id),
        Err(_) => panic!("Budget creation has failed"),
    }
}

pub fn get_budget_by_date(
    conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    start: chrono::DateTime<Utc>,
    end: chrono::DateTime<Utc>,
) -> Result<Budget> {
    let mut stmt = conn.prepare("SELECT * from Budgets WHERE open >= ?1 AND close < ?2;")?;

    stmt.query_row(params![start, end], |row| {
        Ok(Budget::new(
            row.get(0).unwrap(),
            &row.get(1).unwrap(),
            row.get(2).unwrap(),
            row.get(3).unwrap(),
        ))
    })
}

pub fn check_if_budget_exists(
    conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    start: chrono::DateTime<Utc>,
    end: chrono::DateTime<Utc>,
) -> Result<bool> {
    let mut stmt =
        conn.prepare("SELECT EXISTS(SELECT * from Budgets WHERE open >= ?1 AND close < ?2);")?;

    let result = stmt
        .query_row(params![start, end], |row| {
            Ok(SqlResult {
                value: row.get(0).unwrap(),
            })
        })
        .unwrap();

    if result.value == 0 {
        Ok(false)
    } else {
        Ok(true)
    }
}

pub fn add_budget_entry(
    mut conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    budget_id: i32,
    entry: NewBudgetEntry,
) -> Result<()> {
    let con = conn.deref_mut();
    let tx = con.transaction()?;

    tx.execute(
        "INSERT INTO BudgetEntries (account, budget, balance) VALUES (?1, ?2, ?3)",
        params![entry.account, budget_id, entry.balance],
    )?;

    tx.commit()
}

pub fn update_budget_entry(
    mut conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    budget_id: i32,
    entry: NewBudgetEntry,
) -> Result<()> {
    let con = conn.deref_mut();
    let tx = con.transaction()?;

    tx.execute(
        "UPDATE BudgetEntries SET balance = ?1 WHERE account = ?2 AND budget = ?3",
        params![entry.balance, entry.account, budget_id],
    )?;

    tx.commit()
}

pub fn delete_budget_entry(
    mut conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    budget_id: i32,
    entry: NewBudgetEntry,
) -> Result<()> {
    let con = conn.deref_mut();
    let tx = con.transaction()?;

    tx.execute(
        "DELETE FROM BudgetEntries WHERE account = ?1 AND budget = ?2",
        params![entry.account, budget_id],
    )?;

    tx.commit()
}

pub fn list_budget_entries(
    conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    budget: i32,
) -> Result<Vec<BudgetEntry>> {
    let mut stmt =
        conn.prepare("SELECT id, account, budget, balance FROM BudgetEntries WHERE budget = ?1;")?;

    let result = stmt
        .query_map(params![budget], |row| {
            Ok(BudgetEntry {
                id: row.get(0).unwrap(),
                account: row.get(1).unwrap(),
                budget: row.get(2).unwrap(),
                balance: row.get(3).unwrap(),
            })
        })
        .and_then(|mapped_rows| {
            Ok(mapped_rows
                .map(|row| row.unwrap())
                .collect::<Vec<BudgetEntry>>())
        })?;

    Ok(result)
}

pub fn generate_budget(
    mut conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    budget: &Budget,
) -> Result<i64> {
    let con = conn.deref_mut();
    let tx = con.transaction()?;

    tx.execute(
        "INSERT INTO Budgets (name, open, close) VALUES (?1, ?2, ?3)",
        params![budget.name, budget.open, budget.close],
    )?;

    let budget_id = tx.last_insert_rowid();

    tx.execute(
        "INSERT INTO BudgetEntries (account, budget, balance)
        SELECT id, ?1, 0 FROM Accounts WHERE Accounts.type = 4;",
        params![budget_id],
    )?;

    let transaction_result = tx.commit();

    match transaction_result {
        Ok(_) => Ok(budget_id),
        Err(_) => panic!("Budget creation has failed"),
    }
}
