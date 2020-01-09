use actix_web::{web, Error, HttpResponse};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde_json::json;

use crate::account;
use crate::account::data::Account;
use crate::datastruct;

pub mod data;
mod db;

pub async fn list_transactions(
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let result = db::list_transactions(pool.get().unwrap());

    match result {
        Ok(v) => Ok(HttpResponse::Ok().json(v)),
        Err(_e) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

pub async fn list_transactions_with_details(
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let transactions = db::list_transactions(pool.get().unwrap());

    if transactions.is_err() {
        return Ok(HttpResponse::InternalServerError().finish());
    }

    let mut vec: Vec<serde_json::value::Value> = Vec::new();

    for t in transactions.unwrap() {
        let entries = db::get_entries_for_transaction(pool.get().unwrap(), t.id);
        match entries {
            Ok(v) => {
                let result = json!({
                    "transaction": t,
                    "entries": v
                });
                vec.push(result);
            }
            Err(_e) => continue,
        }
    }

    let result = json!({ "transactions": vec });
    Ok(HttpResponse::Ok().json(result))
}

pub async fn list_transactions_date_scoped(
    params: web::Path<datastruct::DateRequest>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    if params.month < 1 || params.month > 12 || params.year < 1970 {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let result = db::list_transactions_date(pool.get().unwrap(), params.month, params.year);

    match result {
        Ok(v) => {
            let date_transactions = json!({
                "transactions": v,
            });

            Ok(HttpResponse::Ok().json(date_transactions))
        }
        Err(_e) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

pub async fn get_transaction(
    params: web::Path<datastruct::IdRequest>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let result = db::get_transaction(pool.get().unwrap(), params.id);

    match result {
        Ok(v) => Ok(HttpResponse::Ok().json(v)),
        Err(_e) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

pub async fn get_transaction_detail(
    params: web::Path<datastruct::IdRequest>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let transaction = db::get_transaction(pool.get().unwrap(), params.id);
    let entries = db::get_entries_for_transaction(pool.get().unwrap(), params.id);

    if transaction.is_err() || entries.is_err() {
        return Ok(HttpResponse::InternalServerError().finish());
    }

    let result = json!({
        "transaction": transaction.unwrap(),
        "entries": entries.unwrap(),
    });

    Ok(HttpResponse::Ok().json(result))
}

fn are_accounts_compatible(from: &Account, to: &Account) -> bool {
    if &from.id == &to.id || &from.currency != &to.currency {
        return false;
    }
    return true;
}

pub async fn create_transaction(
    transaction: web::Json<data::NewTransaction>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    // First we get the two accounts
    let from_acc_query = account::db::get_account(pool.get().unwrap(), transaction.from);
    let to_acc_query = account::db::get_account(pool.get().unwrap(), transaction.to);

    if from_acc_query.is_err() || to_acc_query.is_err() {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let from_account = from_acc_query.unwrap();
    let to_account = to_acc_query.unwrap();

    if are_accounts_compatible(&from_account, &to_account) == false {
        Ok(HttpResponse::BadRequest().finish())
    } else {
        let result = db::create_transaction(
            pool.get().unwrap(),
            to_account.id,
            from_account.id,
            transaction.balance,
            &transaction.name,
        );

        match result {
            Ok(v) => {
                let result = json!({
                    "id": v,
                });

                Ok(HttpResponse::Ok().json(result))
            }
            Err(_e) => Ok(HttpResponse::InternalServerError().finish()),
        }
    }
}

pub async fn delete_transaction(
    params: web::Path<datastruct::IdRequest>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let result = db::remove_transaction(pool.get().unwrap(), params.id);

    match result {
        Ok(_v) => {
            let result = json!({
                "id": params.id,
            });

            Ok(HttpResponse::Ok().json(result))
        }
        Err(_e) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account::data::Account;
    use crate::account::data::AccountType;
    use crate::transaction::data::EntryType;

    #[test]
    fn accounts_compatible() {
        let first = Account {
            id: 0,
            acc_type: AccountType::Assets,
            name: String::from("Current"),
            currency: String::from("GBP"),
        };
        let second = Account {
            id: 1,
            acc_type: AccountType::Expenses,
            name: String::from("Groceries"),
            currency: String::from("GBP"),
        };

        let result = are_accounts_compatible(&first, &second);

        assert_eq!(result, true)
    }

    #[test]
    fn accounts_not_compatible_if_id_equal() {
        let first = Account {
            id: 0,
            acc_type: AccountType::Assets,
            name: String::from("Current"),
            currency: String::from("GBP"),
        };
        let second = Account {
            id: 0,
            acc_type: AccountType::Assets,
            name: String::from("Current"),
            currency: String::from("GBP"),
        };

        let result = are_accounts_compatible(&first, &second);

        assert_eq!(result, false)
    }

    #[test]
    fn accounts_not_compatible_if_different_currencies() {
        let first = Account {
            id: 0,
            acc_type: AccountType::Assets,
            name: String::from("Current"),
            currency: String::from("GBP"),
        };
        let second = Account {
            id: 1,
            acc_type: AccountType::Expenses,
            name: String::from("Groceries"),
            currency: String::from("EUR"),
        };

        let result = are_accounts_compatible(&first, &second);

        assert_eq!(result, false)
    }

    #[test]
    fn entry_correctly_types() {
        let debit = EntryType::from_i32(1);
        let credit = EntryType::from_i32(0);

        assert_eq!(debit, EntryType::Debit);
        assert_eq!(credit, EntryType::Credit)
    }
}
