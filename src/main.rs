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

extern crate chrono;
extern crate rusqlite;

mod api;
mod datastruct;
mod db;

#[macro_use]
extern crate log;

use std::io;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

use r2d2_sqlite::SqliteConnectionManager;

use env_logger;

fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let manager = SqliteConnectionManager::file("ledger.db");
    let pool = r2d2::Pool::new(manager).unwrap();

    let app = move || {
        debug!("Constructing the App");

        App::new()
            .wrap(Logger::default())
            .wrap(Cors::new().send_wildcard().max_age(3600))
            .data(pool.clone())
            .service(web::resource("/").route(web::get().to_async(api::index)))
            .service(web::resource("/currencies").route(web::get().to_async(api::list_currencies)))
            .service(
                web::resource("/integrity").route(web::get().to_async(api::check_ledger_integrity)),
            )
            .service(
                web::scope("/transactions")
                    .service(web::resource("").route(web::get().to_async(api::list_transactions)))
                    .service(
                        web::resource("/detail")
                            .route(web::get().to_async(api::list_transactions_with_details)),
                    )
                    .service(
                        web::resource("/{year}/{month}")
                            .route(web::get().to_async(api::get_transactions_date_scoped)),
                    ),
            )
            .service(
                web::scope("/transaction")
                    .service(web::resource("").route(web::post().to_async(api::create_transaction)))
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to_async(api::get_transaction))
                            .route(web::delete().to_async(api::delete_transaction)),
                    )
                    .service(
                        web::resource("/{id}/detail")
                            .route(web::get().to_async(api::get_transaction_detail)),
                    ),
            )
            .service(
                web::scope("/account")
                    .service(web::resource("/").route(web::post().to_async(api::create_account)))
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to_async(api::get_account))
                            .route(web::delete().to_async(api::delete_account)),
                    )
                    .service(
                        web::resource("/{id}/balance")
                            .route(web::get().to_async(api::get_account_balance)),
                    ),
            )
    };

    debug!("Starting server");
    HttpServer::new(app).bind("localhost:8088")?.run()
}
