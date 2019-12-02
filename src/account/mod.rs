use actix_web::{web, Error, HttpResponse};
use futures::future::ok;
use futures::future::Future;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

pub mod data;
mod db;

pub fn list_asset_accounts(
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let result = db::list_accounts_filter_type(pool.get().unwrap(), data::AccountType::Assets);

    match result {
        Ok(v) => ok(HttpResponse::Ok().json(v)),
        Err(_e) => ok(HttpResponse::InternalServerError().finish()),
    }
}

pub fn list_expense_accounts(
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let result = db::list_accounts_filter_type(pool.get().unwrap(), data::AccountType::Expenses);

    match result {
        Ok(v) => ok(HttpResponse::Ok().json(v)),
        Err(_e) => ok(HttpResponse::InternalServerError().finish()),
    }
}
