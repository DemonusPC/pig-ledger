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

mod account;
mod api;
mod budget;
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
    // std::env::set_var("RUST_LOG", "actix_web=info");
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let manager = SqliteConnectionManager::file("ledger.db");
    let pool = r2d2::Pool::new(manager).unwrap();

    let app = move || {
        debug!("Constructing the App");

        App::new()
            .wrap(Logger::default())
            .wrap(Cors::new().send_wildcard().max_age(3600))
            .data(pool.clone())
            .service(web::resource("/").route(web::get().to_async(account::list_accounts)))
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
                    .service(
                        web::resource("/").route(web::post().to_async(account::create_account)),
                    )
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to_async(account::get_account))
                            .route(web::delete().to_async(account::delete_account)),
                    )
                    .service(
                        web::resource("/{id}/balance")
                            .route(web::get().to_async(api::get_account_balance)),
                    ),
            )
            .service(
                web::scope("/accounts")
                    .service(web::resource("").route(web::get().to_async(account::list_accounts)))
                    .service(
                        web::resource("/asset")
                            .route(web::get().to_async(account::list_asset_accounts)),
                    )
                    .service(
                        web::resource("/expense")
                            .route(web::get().to_async(account::list_expense_accounts)),
                    ),
            )
            .service(
                web::scope("/budget")
                    .service(
                        web::resource("")
                            .route(web::get().to_async(budget::get_current_budget))
                            .route(web::post().to_async(budget::create_budget)),
                    )
                    .service(
                        web::resource("/generate")
                            .route(web::post().to_async(budget::generate_budget)),
                    )
                    .service(
                        web::scope("/{id}")
                            .service(
                                web::resource("")
                                    .route(web::get().to_async(budget::get_budget))
                                    .route(web::delete().to_async(budget::delete_budget)),
                            )
                            .service(
                                web::resource("/entry")
                                    .route(web::post().to_async(budget::add_entry_to_budget))
                                    .route(web::put().to_async(budget::update_entry_in_budget))
                                    .route(web::delete().to_async(budget::delete_entry_in_budget)),
                            ),
                    ),
            )
    };

    debug!("Starting server");
    HttpServer::new(app).bind("localhost:8088")?.run()
}
