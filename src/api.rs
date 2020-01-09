use actix_web::{web, Error, HttpResponse};
use serde_json::json;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::datastruct;
use crate::db;

pub async fn get_account_balance(
    params: web::Path<datastruct::IdRequest>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let result = db::current_balance(pool.get().unwrap(), params.id);

    match result {
        Ok(v) => Ok(HttpResponse::Ok().json(v)),
        Err(_e) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

pub async fn list_currencies(
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    let result = db::list_currencies(conn);

    match result {
        Ok(v) => Ok(HttpResponse::Ok().json(v)),
        Err(_e) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

pub async fn check_ledger_integrity(
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    let result = db::check_integrity(conn);

    match result {
        Ok(v) => {
            let result = json!({
                "integrity": v,
            });
            Ok(HttpResponse::Ok().json(result))
        }
        Err(_e) => Ok(HttpResponse::InternalServerError().finish()),
    }
}
