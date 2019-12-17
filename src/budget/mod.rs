use crate::chrono::Datelike;
use actix_web::{web, Error, HttpResponse};
use chrono::{DateTime, Duration, TimeZone, Utc};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde_json::json;

pub mod data;
mod db;

use crate::datastruct;

pub async fn get_budget(
    params: web::Path<datastruct::IdRequest>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    let budget = db::get_budget(conn, params.id);
    let budget_entries = db::list_budget_entries(pool.get().unwrap(), params.id);

    if budget.is_err() || budget_entries.is_err() {
        error!("Get Budget failed");
        return Ok(HttpResponse::InternalServerError().finish());
    }

    let result = json!({
        "budget": budget.unwrap(),
        "entries": budget_entries.unwrap(),
    });

    Ok(HttpResponse::Ok().json(result))
}

pub async fn delete_budget(
    params: web::Path<datastruct::IdRequest>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    let result = db::remove_budget(conn, params.id);

    match result {
        Ok(_v) => Ok(HttpResponse::Ok().finish()),
        Err(_e) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

pub async fn create_budget(
    budget_request: web::Json<data::NewBudget>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let open_time = DateTime::parse_from_rfc3339(&budget_request.open);
    let close_time = DateTime::parse_from_rfc3339(&budget_request.close);

    if open_time.is_err() || close_time.is_err() {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let open_utc = open_time.unwrap().with_timezone(&Utc);
    let close_utc = close_time.unwrap().with_timezone(&Utc);

    if db::check_if_budget_exists(pool.get().unwrap(), open_utc, close_utc + Duration::days(1))
        .unwrap()
        == true
    {
        error!(
            "Budget already exists for {open} - {close}",
            open = close_utc,
            close = close_utc
        );
        return Ok(HttpResponse::Conflict().finish());
    }

    if close_utc < open_utc {
        error!(
            "Wrong Request. {close} is smaller than {open}",
            close = close_utc,
            open = open_utc
        );
        return Ok(HttpResponse::BadRequest().finish());
    }

    let parsed_budget = data::Budget::new(-1, &budget_request.name, open_utc, close_utc);

    let result = db::create_budget(pool.get().unwrap(), &parsed_budget);

    match result {
        Ok(v) => Ok(HttpResponse::Created().json(v)),
        Err(e) => {
            error!("Create current budget failed with {error}.", error = e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

pub async fn get_current_budget(
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();

    let now = chrono::offset::Utc::now();
    let start_month = Utc.ymd(now.year(), now.month(), 1).and_hms(0, 0, 0);
    let end_month = now + (Duration::weeks(4) - Duration::days(1));

    let result = db::get_budget_by_date(conn, start_month, end_month);

    match result {
        Ok(v) => Ok(HttpResponse::Ok().json(v)),
        Err(e) => {
            if e == rusqlite::Error::QueryReturnedNoRows {
                warn!("No monthly budget specified");
                return Ok(HttpResponse::NotFound().finish());
            }
            error!("Get current budget failed with {error}", error = e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

pub async fn add_entry_to_budget(
    pool: web::Data<Pool<SqliteConnectionManager>>,
    params: web::Path<datastruct::IdRequest>,
    entry: web::Json<data::NewBudgetEntry>,
) -> Result<HttpResponse, Error> {
    let parsed_entry = entry.into_inner();
    let result = db::add_budget_entry(pool.get().unwrap(), params.id, parsed_entry);
    match result {
        Ok(v) => Ok(HttpResponse::Ok().json(v)),
        Err(_e) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

pub async fn update_entry_in_budget(
    pool: web::Data<Pool<SqliteConnectionManager>>,
    params: web::Path<datastruct::IdRequest>,
    entry: web::Json<data::NewBudgetEntry>,
) -> Result<HttpResponse, Error> {
    let parsed_entry = entry.into_inner();
    let result = db::update_budget_entry(pool.get().unwrap(), params.id, parsed_entry);
    match result {
        Ok(_v) => Ok(HttpResponse::Ok().json(true)),
        Err(_e) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

pub async fn delete_entry_in_budget(
    pool: web::Data<Pool<SqliteConnectionManager>>,
    params: web::Path<datastruct::IdRequest>,
    entry: web::Json<data::NewBudgetEntry>,
) -> Result<HttpResponse, Error> {
    let parsed_entry = entry.into_inner();
    let result = db::delete_budget_entry(pool.get().unwrap(), params.id, parsed_entry);
    match result {
        Ok(_v) => Ok(HttpResponse::Ok().json(true)),
        Err(_e) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

pub async fn generate_budget(
    budget_request: web::Json<data::NewBudget>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let open_time = DateTime::parse_from_rfc3339(&budget_request.open);
    let close_time = DateTime::parse_from_rfc3339(&budget_request.close);

    if open_time.is_err() || close_time.is_err() {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let open_utc = open_time.unwrap().with_timezone(&Utc);
    let close_utc = close_time.unwrap().with_timezone(&Utc);

    if db::check_if_budget_exists(pool.get().unwrap(), open_utc, close_utc + Duration::days(1))
        .unwrap()
        == true
    {
        error!(
            "Budget already exists for {open} - {close}",
            open = close_utc,
            close = close_utc
        );
        return Ok(HttpResponse::Conflict().finish());
    }

    if close_utc < open_utc {
        error!(
            "Wrong Request. {close} is smaller than {open}",
            close = close_utc,
            open = open_utc
        );
        return Ok(HttpResponse::BadRequest().finish());
    }

    let parsed_budget = data::Budget::new(-1, &budget_request.name, open_utc, close_utc);

    let result = db::generate_budget(pool.get().unwrap(), &parsed_budget);

    match result {
        Ok(v) => Ok(HttpResponse::Created().json(v)),
        Err(e) => {
            error!("Create current budget failed with {error}.", error = e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}
