use actix_web::{web, Error, HttpResponse};
use futures::future::{Future};

use futures::{future::ok};



use crate::db;
use crate::datastruct;

pub fn index() -> impl Future<Item = HttpResponse, Error = Error> {
    let result = db::list_accounts();

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

pub fn create_transaction(transaction: web::Json<datastruct::NewTransaction>) -> impl Future<Item = HttpResponse, Error = Error> {
    let from_account = db::get_account(&transaction.from).unwrap();
    let to_account = db::get_account(&transaction.to).unwrap();

    let result = db::transaction(to_account.id, from_account.id, transaction.balance, &transaction.name);

    match result {
        Ok(_v) => ok(HttpResponse::Ok().finish()),
        Err(_e) => ok(HttpResponse::InternalServerError().finish()),
    } 
}

pub fn delete_transaction(params: web::Path<datastruct::IdRequest>) -> impl Future<Item = HttpResponse, Error = Error> {
    let result = db::remove_transaction(params.id);

    match result {
        Ok(_v) => ok(HttpResponse::Ok().finish()),
        Err(_e) => ok(HttpResponse::InternalServerError().finish()),
    } 
}

pub fn get_transaction(params: web::Path<datastruct::IdRequest>) -> impl Future<Item = HttpResponse, Error = Error> {
    let result = db::get_transaction(params.id);

    match result {
        Ok(v) => ok(HttpResponse::Ok().json(v)),
        Err(_e) => ok(HttpResponse::InternalServerError().finish()),
    } 
}

pub fn get_account(params: web::Path<datastruct::IdRequest>) -> impl Future<Item = HttpResponse, Error = Error> {
    let result = db::get_account_by_id(params.id);

    match result {
        Ok(v) => ok(HttpResponse::Ok().json(v)),
        Err(_e) => ok(HttpResponse::InternalServerError().finish()),
    } 
}

pub fn create_account(account: web::Json<datastruct::NewAccount>) -> impl Future<Item = HttpResponse, Error = Error> {
    let account_type = datastruct::AccountType::from_i32(account.acc_type);
    let result = db::add_account(account_type, &account.name);

    match result {
        Ok(v) => ok(HttpResponse::Ok().json(v)),
        Err(_e) => ok(HttpResponse::InternalServerError().finish()),
    } 
}

pub fn delete_account(params: web::Path<datastruct::IdRequest>) -> impl Future<Item = HttpResponse, Error = Error> {
    let result = db::remove_account_by_id(params.id);

    match result {
        Ok(_v) => ok(HttpResponse::Ok().finish()),
        Err(_e) => ok(HttpResponse::InternalServerError().finish()),
    } 
}

pub fn get_account_balance(params: web::Path<datastruct::IdRequest>) -> impl Future<Item = HttpResponse, Error = Error> {
    let result = db::current_balance(params.id);

    match result {
        Ok(v) => ok(HttpResponse::Ok().json(v)),
        Err(_e) => ok(HttpResponse::InternalServerError().finish()),
    } 
}