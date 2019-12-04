use actix_web::{web, Error, HttpResponse};
use futures::future::Future;

use futures::future::ok;

use serde_json::json;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::account::data::Account;
use crate::datastruct;
use crate::db;

pub fn list_transactions() -> impl Future<Item = HttpResponse, Error = Error> {
    let result = db::list_transactions();

    match result {
        Ok(v) => ok(HttpResponse::Ok().json(v)),
        Err(_e) => ok(HttpResponse::InternalServerError().finish()),
    }
}

pub fn list_transactions_with_details(
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let transactions = db::list_transactions();

    if transactions.is_err() {
        return ok(HttpResponse::InternalServerError().finish());
    }

    let mut vec: Vec<serde_json::value::Value> = Vec::new();

    for t in transactions.unwrap() {
        let entries = db::get_entries(pool.get().unwrap(), t.id);
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
    ok(HttpResponse::Ok().json(result))
}

pub fn get_transactions_date_scoped(
    params: web::Path<datastruct::DateRequest>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    if params.month < 1 || params.month > 12 || params.year < 1970 {
        return ok(HttpResponse::BadRequest().finish());
    }

    let result = db::list_transactions_date(pool.get().unwrap(), params.month, params.year);

    match result {
        Ok(v) => {
            let date_transactions = json!({
                "transactions": v,
            });

            ok(HttpResponse::Ok().json(date_transactions))
        }
        Err(_e) => ok(HttpResponse::InternalServerError().finish()),
    }
}

fn are_accounts_compatible(from: &Account, to: &Account) -> bool {
    if &from.id == &to.id || &from.currency != &to.currency {
        return false;
    }
    return true;
}

pub fn create_transaction(
    transaction: web::Json<datastruct::NewTransaction>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // First we get the two accounts
    let from_acc_query = db::get_account(pool.get().unwrap(), transaction.from);
    let to_acc_query = db::get_account(pool.get().unwrap(), transaction.to);

    if from_acc_query.is_err() || to_acc_query.is_err() {
        return ok(HttpResponse::BadRequest().finish());
    }

    let from_account = from_acc_query.unwrap();
    let to_account = to_acc_query.unwrap();

    if are_accounts_compatible(&from_account, &to_account) == false {
        ok(HttpResponse::BadRequest().finish())
    } else {
        let result = db::transaction(
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

                ok(HttpResponse::Ok().json(result))
            }
            Err(_e) => ok(HttpResponse::InternalServerError().finish()),
        }
    }
}

pub fn delete_transaction(
    params: web::Path<datastruct::IdRequest>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let result = db::remove_transaction(pool.get().unwrap(), params.id);

    match result {
        Ok(_v) => {
            let result = json!({
                "id": params.id,
            });

            ok(HttpResponse::Ok().json(result))
        }
        Err(_e) => ok(HttpResponse::InternalServerError().finish()),
    }
}

pub fn get_transaction(
    params: web::Path<datastruct::IdRequest>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let result = db::get_transaction(pool.get().unwrap(), params.id);

    match result {
        Ok(v) => ok(HttpResponse::Ok().json(v)),
        Err(_e) => ok(HttpResponse::InternalServerError().finish()),
    }
}

pub fn get_transaction_detail(
    params: web::Path<datastruct::IdRequest>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let transaction = db::get_transaction(pool.get().unwrap(), params.id);
    let entries = db::get_entries(pool.get().unwrap(), params.id);

    if transaction.is_err() || entries.is_err() {
        return ok(HttpResponse::InternalServerError().finish());
    }

    let result = json!({
        "transaction": transaction.unwrap(),
        "entries": entries.unwrap(),
    });

    ok(HttpResponse::Ok().json(result))
}

pub fn get_account_balance(
    params: web::Path<datastruct::IdRequest>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let result = db::current_balance(pool.get().unwrap(), params.id);

    match result {
        Ok(v) => ok(HttpResponse::Ok().json(v)),
        Err(_e) => ok(HttpResponse::InternalServerError().finish()),
    }
}

pub fn list_currencies(
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let conn = pool.get().unwrap();
    let result = db::list_currencies(conn);

    match result {
        Ok(v) => ok(HttpResponse::Ok().json(v)),
        Err(_e) => ok(HttpResponse::InternalServerError().finish()),
    }
}

pub fn check_ledger_integrity(
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let conn = pool.get().unwrap();
    let result = db::check_integrity(conn);

    match result {
        Ok(v) => {
            let result = json!({
                "integrity": v,
            });
            ok(HttpResponse::Ok().json(result))
        }
        Err(_e) => ok(HttpResponse::InternalServerError().finish()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account::data::Account;
    use crate::account::data::AccountType;

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
}
