use crate::datastruct;
use actix_web::{web, Error, HttpResponse};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde_json::json;

use crate::account;
use crate::transaction::data;
use crate::transaction::db;

// Get a single transaction
pub async fn get_transaction_v2(
    params: web::Path<datastruct::IdRequest>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let transaction = db::get_transaction_v2(pool.get().unwrap(), params.id);

    match transaction {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(err) => match err {
            rusqlite::Error::QueryReturnedNoRows => Ok(HttpResponse::NotFound().finish()),
            _ => Ok(HttpResponse::InternalServerError().finish()),
        },
    }
}

// Create a new transaction
pub async fn create_transaction_v2(
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

    if !from_account.currency_compatible(&to_account) {
        return Ok(HttpResponse::BadRequest().finish());
    }

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
                "status": "CREATED",
                "id": v,
            });

            Ok(HttpResponse::Ok().json(result))
        }
        Err(_e) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

// Update Transaction name or balance
pub async fn update_transaction_v2(
    params: web::Path<datastruct::IdRequest>,
    transaction: web::Json<data::UpdateTransaction>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let result = db::update_transaction(
        pool.get().unwrap(),
        params.id,
        transaction.balance,
        &transaction.name,
    );

    match result {
        Ok(()) => {
            let result = json!({
                "status": "UPDATED",
                "id": params.id,
            });

            Ok(HttpResponse::Ok().json(result))
        }
        Err(_e) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

// Delete a single transaction
pub async fn delete_transaction(
    params: web::Path<datastruct::IdRequest>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let result = db::remove_transaction(pool.get().unwrap(), params.id);

    match result {
        Ok(_v) => {
            let result = json!({
                "status": "DELETED",
                "id": params.id,
            });

            Ok(HttpResponse::Ok().json(result))
        }
        Err(_e) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

// List operations
// Get all transactions limited to 32

pub async fn list_transactions(
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let result = db::list_transactions(pool.get().unwrap());

    match result {
        Ok(v) => Ok(HttpResponse::Ok().json(v)),
        Err(_e) => Ok(HttpResponse::InternalServerError().finish()),
    }
}
