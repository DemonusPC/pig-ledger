use crate::datastruct::{Currency, SqlResult};

use rusqlite::{params, Result, NO_PARAMS};

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

    let result = if query[0].value == 0 { true } else { false };

    Ok(result)
}

pub fn current_balance(
    conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    account: i32,
) -> Result<SqlResult> {
    let mut stmt = conn.prepare("SELECT (SELECT ifnull(SUM(balance),0) as \"Debits\" FROM Debits WHERE account = ?1) - (SELECT ifnull(SUM(balance),0) as \"Credits\" FROM Credits WHERE account = ?1)")?;

    stmt.query_row(params![account], |row| {
        Ok(SqlResult {
            value: row.get(0).unwrap(),
        })
    })
}

pub fn list_currencies(
    conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
) -> Result<Vec<Currency>> {
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
