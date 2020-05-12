
use actix_web::{web, Error, HttpResponse};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde_json::json;
use crate::datastruct;

use crate::transaction::db;

// Get a single transaction
pub async fn get_transaction_v2(
    params: web::Path<datastruct::IdRequest>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let transaction = db::get_transaction_v2(pool.get().unwrap(), params.id);

    match transaction {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(err) => {
            match err {
                rusqlite::Error::QueryReturnedNoRows => Ok(HttpResponse::NotFound().finish()),
                _ => Ok(HttpResponse::InternalServerError().finish())
            }      
        }
    }
}

// Create a new transaction

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

