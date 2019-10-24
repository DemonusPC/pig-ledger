// Accounts
    // Assets = 0
    // Liabilities = 1
    // Equities = 2
    // Revenue = 3
    // Expenses = 4
    // Gains = 5 
    // Losses = 6

// Assets = Liabilities + Equity

// Assets DB + CR -
// Liabilities DB - CR +

extern crate rusqlite;
extern crate chrono;


mod db;
mod datastruct;
mod api;

#[macro_use]
extern crate log;

use std::{io};


use actix_web::middleware::{Logger};
use actix_web::{web, middleware, App, HttpServer};


use env_logger;


fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let app = move || {
        debug!("Constructing the App");

        App::new()
            .wrap(Logger::default())
            .service(web::resource("/").route(web::get().to_async(api::index)))
            .service(web::resource("/transactions").route(web::get().to_async(api::list_transactions)))
            .service(web::scope("/transaction")
                .service(web::resource("/").route(web::post().to_async(api::create_transaction)))
                .service(
                    web::resource("/{id}")
                        .route(web::get().to_async(api::get_transaction))
                        .route(web::delete().to_async(api::delete_transaction)),
                )
            )
            .service(web::scope("/account")
                .service(web::resource("/").route(web::post().to_async(api::create_account)))
                .service(
                    web::resource("/{id}")
                        .route(web::get().to_async(api::get_account))
                        .route(web::delete().to_async(api::delete_account)),
                )
                .service(web::resource("/{id}/balance").route(web::get().to_async(api::get_account_balance)))
            )
            
    };

    debug!("Starting server");
    HttpServer::new(app).bind("localhost:8088")?.run()
}

