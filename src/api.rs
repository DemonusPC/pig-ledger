use actix_web::{web, Error, HttpResponse};
use futures::future::Future;

use futures::future::ok;

use serde_json::json;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::datastruct;
use crate::db;

pub fn index(
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let conn = pool.get().unwrap();
    let result = db::list_accounts(conn);

    match result {
        Ok(v) => ok(HttpResponse::Ok().json(v)),
        Err(_e) => ok(HttpResponse::InternalServerError().finish()),
    }
}

pub fn list_transactions() -> impl Future<Item = HttpResponse, Error = Error> {
    let result = db::list_transactions();

    match result {
        Ok(v) => ok(HttpResponse::Ok().json(v)),
        Err(_e) => ok(HttpResponse::InternalServerError().finish()),
    }
}

pub fn create_transaction(
    transaction: web::Json<datastruct::NewTransaction>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let from_account = db::get_account(pool.get().unwrap(), &transaction.from).unwrap();
    let to_account = db::get_account(pool.get().unwrap(), &transaction.to).unwrap();

    let result = db::transaction(
        pool.get().unwrap(),
        to_account.id,
        from_account.id,
        transaction.balance,
        &transaction.name,
    );

    match result {
        Ok(_v) => ok(HttpResponse::Ok().finish()),
        Err(_e) => ok(HttpResponse::InternalServerError().finish()),
    }
}

pub fn delete_transaction(
    params: web::Path<datastruct::IdRequest>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let result = db::remove_transaction(pool.get().unwrap(), params.id);

    match result {
        Ok(_v) => ok(HttpResponse::Ok().finish()),
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
    let debit = db::get_debit(pool.get().unwrap(), params.id);
    let credit = db::get_credit(pool.get().unwrap(), params.id);

    if transaction.is_err() || debit.is_err() || credit.is_err() {
        return ok(HttpResponse::InternalServerError().finish());
    }

    let result = json!({
        "transaction": transaction.unwrap(),
        "debit": debit.unwrap(),
        "credit": credit.unwrap()
    });

    ok(HttpResponse::Ok().json(result))
}

pub fn get_account(
    params: web::Path<datastruct::IdRequest>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let conn = pool.get().unwrap();
    let result = db::get_account_by_id(conn, params.id);

    match result {
        Ok(v) => ok(HttpResponse::Ok().json(v)),
        Err(_e) => ok(HttpResponse::InternalServerError().finish()),
    }
}

pub fn create_account(
    account: web::Json<datastruct::NewAccount>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let account_type = datastruct::AccountType::from_i32(account.acc_type);
    let result = db::add_account(
        pool.get().unwrap(),
        account_type,
        &account.name,
        &account.currency,
    );

    match result {
        Ok(v) => ok(HttpResponse::Ok().json(v)),
        Err(_e) => ok(HttpResponse::InternalServerError().finish()),
    }
}

pub fn delete_account(
    params: web::Path<datastruct::IdRequest>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let result = db::remove_account_by_id(pool.get().unwrap(), params.id);

    match result {
        Ok(_v) => ok(HttpResponse::Ok().finish()),
        Err(_e) => ok(HttpResponse::InternalServerError().finish()),
    }
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
