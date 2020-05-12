
use actix_web::{web, Error, HttpResponse};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde_json::json;

use crate::account;
use crate::account::data::Account;
use crate::datastruct;

use crate::transaction::db;

pub async fn get_transaction_v2(
    params: web::Path<datastruct::IdRequest>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let transaction = db::get_transaction_v2(pool.get().unwrap(), params.id);

    if transaction.is_err(){
        return Ok(HttpResponse::InternalServerError().finish());
    }

    Ok(HttpResponse::Ok().json(transaction.unwrap()))
}