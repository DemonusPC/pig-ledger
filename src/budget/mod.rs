use crate::chrono::Datelike;
use actix_web::{web, Error, HttpResponse};
use chrono::{DateTime, Duration, TimeZone, Utc};
use futures::future::ok;
use futures::future::Future;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use uuid::Uuid;

pub mod data;
mod db;

use crate::datastruct;

pub fn get_budget(
    params: web::Path<datastruct::IdRequest>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let conn = pool.get().unwrap();
    let result = db::get_budget(conn, params.id);

    match result {
        Ok(v) => ok(HttpResponse::Ok().json(v)),
        Err(_e) => ok(HttpResponse::InternalServerError().finish()),
    }
}

pub fn delete_budget(
    params: web::Path<datastruct::IdRequest>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let conn = pool.get().unwrap();
    let result = db::remove_budget(conn, params.id);

    match result {
        Ok(_v) => ok(HttpResponse::Ok().finish()),
        Err(_e) => ok(HttpResponse::InternalServerError().finish()),
    }
}

pub fn create_budget(
    budget_request: web::Json<data::NewBudget>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let target = Uuid::new_v4().to_simple().to_string();
    let open_time = DateTime::parse_from_rfc3339(&budget_request.open);
    let close_time = DateTime::parse_from_rfc3339(&budget_request.close);

    if open_time.is_err() || close_time.is_err() {
        return ok(HttpResponse::BadRequest().finish());
    }

    let open_utc = open_time.unwrap().with_timezone(&Utc);
    let close_utc = close_time.unwrap().with_timezone(&Utc);

    if close_utc < open_utc {
        error!(
            "Wrong Request. {close} is smaller than {open}",
            close = close_utc,
            open = open_utc
        );
        return ok(HttpResponse::BadRequest().finish());
    }

    let parsed_budget = data::Budget::new(-1, &budget_request.name, open_utc, close_utc, &target);

    let result = db::create_budget(pool.get().unwrap(), &parsed_budget);

    match result {
        Ok(v) => ok(HttpResponse::Ok().json(v)),
        Err(_e) => ok(HttpResponse::InternalServerError().finish()),
    }
}

pub fn get_current_budget(
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let conn = pool.get().unwrap();

    let now = chrono::offset::Utc::now();
    let start_month = Utc.ymd(now.year(), now.month(), 1).and_hms(0, 0, 0);
    let end_month = now + (Duration::weeks(4) - Duration::days(1));

    let result = db::get_budget_by_date(conn, start_month, end_month);

    match result {
        Ok(v) => ok(HttpResponse::Ok().json(v)),
        Err(e) => {
            error!("Get current budget failed with {error}.", error = e);
            ok(HttpResponse::InternalServerError().finish())
        }
    }
}
