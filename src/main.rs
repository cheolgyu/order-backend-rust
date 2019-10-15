// #![deny(warnings)]
// #![allow(warnings)]

#[macro_use]
extern crate diesel;
extern crate actix_derive;
#[macro_use]
extern crate diesel_derives;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate lazy_static;
use actix::prelude::*;

mod api;
mod batch;
mod config;
mod errors;
mod fcm;
mod middleware;
mod models;
mod schema;
mod utils;

use crate::models::{AppStateWithTxt, DbExecutor, WebPush, WebSocket};

use actix_cors::Cors;
use actix_web::{
    get,
    http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    middleware as actix_middleware, web, App, HttpRequest, HttpServer,
};
use diesel::{r2d2::ConnectionManager, PgConnection};
use dotenv::dotenv;
use std::env;

#[get("/resource1/{name}/index.html")]
fn index(req: HttpRequest, name: web::Path<String>) -> String {
    format!("Hello: {}!\r\n", name)
}

#[get("/")]
fn no_params() -> &'static str {
    "Hello world!\r\n"
}

fn main() -> std::io::Result<()> {
    std::env::set_var(
        "RUST_LOG",
        "order-back-rust=debug,actix_web=debug,actix_server=debug",
    );

    dotenv().ok();
    let domain: String = env::var("DOMAIN").expect("DOMAIN must be set");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let url_frontend_ceo: String =
        env::var("URL_FRONTEND_CEO").expect("URL_FRONTEND_CEO must be set");
    let url_frontend_user: String =
        env::var("URL_FRONTEND_USER").expect("URL_FRONTEND_USER must be set");
    let valid_email: String = env::var("VALID_EMAIL").expect("VALID_EMAIL must be set");

    let store = AppStateWithTxt {
        websocket: WebSocket {
            send: env::var("WEBSOCKET_URL").expect("WEBSOCKET_URL must be set"),
        },
        webpush: WebPush {
            send: env::var("WEBPUSH_URL_SEND").expect("webpush_url_send must be set"),
            reg: env::var("WEBPUSH_URL_REG").expect("webpush_url_reg must be set"),
            send_id: env::var("WEBPUSH_SEND_ID").expect("webpush_send_id must be set"),
            key: format!(
                "key={}",
                env::var("WEBPUSH_KEY").expect("WEBPUSH_KEY must be set")
            ),
        },
        valid_email: env::var("VALID_EMAIL").expect("VALID_EMAIL must be set"),
    };

    env_logger::init();

    let sys = actix_rt::System::new("mybackend");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url.clone());
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let address: Addr<DbExecutor> = SyncArbiter::start(4, move || DbExecutor(pool.clone()));
    let manager2 = ConnectionManager::<PgConnection>::new(database_url.clone());
    let pool2 = r2d2::Pool::builder()
        .build(manager2)
        .expect("Failed to create pool.");

    let bat = batch::Batch::new(
        web::Data::new(address.clone()),
        web::Data::new(store.clone()),
    );
    bat.start();

    //let addr_batch: Addr<batch::Batch> = SyncArbiter::start(1, move || batch::Batch( web::Data::new(address2.clone()),web::Data::new(store2.clone()) ) );

    HttpServer::new(move || {
        App::new()
            .data(address.clone())
            .data(pool2.clone())
            .data(store.clone())
            .data(valid_email.clone())
            .wrap(actix_middleware::Logger::default())
            .wrap(
                Cors::new()
                    .allowed_origin(&url_frontend_ceo)
                    .allowed_origin(&url_frontend_user)
                    .allowed_methods(vec!["GET", "POST", "PUT", "OPTIONS", "DELETE"])
                    .allowed_headers(vec![AUTHORIZATION, ACCEPT])
                    .allowed_header(CONTENT_TYPE)
                    .max_age(3600),
            )
            .service(web::scope("/api/v1/ceo/auth").configure(config::ceo::v1::public))
            .service(
                web::scope("/api/v1/ceo")
                    .wrap(middleware::auth::CheckToken)
                    .configure(config::ceo::v1::config),
            )
            .service(web::scope("/api/v1/user").configure(config::user::v1::config))
            .service(index)
            .service(no_params)
    })
    .bind(domain)?
    .workers(1)
    .start();

    sys.run()
}
