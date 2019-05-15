#![allow(unused_imports)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate actix_derive;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
use actix::prelude::*;
mod models;
mod schema;
mod svc;
use dotenv::dotenv;
use futures::IntoFuture;
use std::env;

use crate::models::DbExecutor;
use actix_web::{get, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use diesel::{r2d2::ConnectionManager, PgConnection};
#[get("/resource1/{name}/index.html")]
fn index(req: HttpRequest, name: web::Path<String>) -> String {
    println!("REQ: {:?}", req);
    format!("Hello: {}!\r\n", name)
}

fn index_async(req: HttpRequest) -> impl IntoFuture<Item = &'static str, Error = Error> {
    println!("REQ: {:?}", req);
    Ok("Hello world!\r\n")
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
    let addr = env::var("ADDR").expect("ADDR must be set");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    env_logger::init();

    let sys = actix_rt::System::new("mybackend");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let address: Addr<DbExecutor> = SyncArbiter::start(4, move || DbExecutor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .data(address.clone())
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(svc::auth::router::signup)
            .service(svc::auth::router::signin)
            //.service(web::resource("/signup").route(web::put().to_async(svc::auth::router::signup)))
            /*
            .service(
                web::scope("/api/v1").service(
                    web::resource("/auth")
                        .route(web::post().to_async(auth_routes::login))
                        .route(web::delete().to(auth_routes::logout))
                        .route(web::get().to_async(auth_routes::get_me)),
                ),
            )
            */
            .service(index)
            .service(no_params)
            .service(
                web::resource("/resource2/index.html")
                    .wrap(middleware::DefaultHeaders::new().header("X-Version-R2", "0.3"))
                    .default_service(web::route().to(|| HttpResponse::MethodNotAllowed()))
                    .route(web::get().to_async(index_async)),
            )
            .service(web::resource("/test1.html").to(|| "Test\r\n"))
    })
    .bind(addr)?
    .workers(1)
    .start();

    sys.run()
}
