use actix_web::{web, Error, HttpResponse};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

mod account;
mod account_hierarchy;
pub mod data;
pub mod db;
mod traits;

use crate::datastruct;

pub async fn get_account(
    params: web::Path<datastruct::IdRequest>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    let result = db::get_account(conn, params.id);

    match result {
        Ok(v) => Ok(HttpResponse::Ok().json(v)),
        Err(_e) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

pub async fn create_account(
    account: web::Json<data::NewAccount>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let account_type = data::AccountType::from_i32(account.acc_type);
    let result = db::add_account(
        pool.get().unwrap(),
        account_type,
        &account.name,
        &account.currency,
    );

    match result {
        Ok(v) => Ok(HttpResponse::Ok().json(v)),
        Err(_e) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

pub async fn delete_account(
    params: web::Path<datastruct::IdRequest>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let result = db::remove_account(pool.get().unwrap(), params.id);

    match result {
        Ok(_v) => Ok(HttpResponse::Ok().finish()),
        Err(_e) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

pub async fn list_accounts(
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    let result = db::list_accounts(conn);

    match result {
        Ok(v) => Ok(HttpResponse::Ok().json(v)),
        Err(_e) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

pub async fn list_asset_accounts(
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let result = db::list_accounts_filter_type(pool.get().unwrap(), data::AccountType::Assets);

    match result {
        Ok(v) => Ok(HttpResponse::Ok().json(v)),
        Err(_e) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

pub async fn list_expense_accounts(
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let result = db::list_accounts_filter_type(pool.get().unwrap(), data::AccountType::Expenses);

    match result {
        Ok(v) => Ok(HttpResponse::Ok().json(v)),
        Err(_e) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

pub use self::account::AccountV2;
pub use self::data::AccountType;
pub use self::traits::AccountAble;
