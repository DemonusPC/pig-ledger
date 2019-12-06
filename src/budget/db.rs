use crate::budget::data::Budget;
use rusqlite::{params, Result};
use std::ops::DerefMut;

pub fn get_budget(
    conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    id: i32,
) -> Result<(Budget)> {
    let mut stmt =
        conn.prepare("SELECT id, name, open, close, target FROM Budgets WHERE id = ?1")?;

    stmt.query_row(params![id], |row| {
        Ok(Budget::new(
            row.get(0).unwrap(),
            &row.get(1).unwrap(),
            row.get(2).unwrap(),
            row.get(3).unwrap(),
            &row.get(4).unwrap(),
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
) -> Result<(i64)> {
    let con = conn.deref_mut();
    let tx = con.transaction()?;

    tx.execute(
        "INSERT INTO Budgets (name, open, close, target) VALUES (?1, ?2, ?3, ?4)",
        params![budget.name, budget.open, budget.close, budget.get_target()],
    )?;

    let budget_id = tx.last_insert_rowid();

    let transaction_result = tx.commit();

    match transaction_result {
        Ok(_) => Ok(budget_id),
        Err(_) => panic!("Budget creation has failed"),
    }
}