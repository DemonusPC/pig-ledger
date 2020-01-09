use actix_web::{web, Error, HttpResponse};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde_json::json;

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
