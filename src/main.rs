#![allow(unused_imports)]

#[macro_use]
extern crate diesel;
extern crate actix_derive;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
use actix::prelude::*;
mod errors;
mod models;
mod schema;
mod svc;
mod utils;
use crate::models::DbExecutor;
use actix_web::{get, middleware, web, App, HttpRequest, HttpServer};
use diesel::{r2d2::ConnectionManager, PgConnection};
use dotenv::dotenv;
use std::env;
#[get("/resource1/{name}/index.html")]
fn index(req: HttpRequest, name: web::Path<String>) -> String {
    println!("REQ: {:?}", req);
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
      
            .service(
                web::scope("/api/v1")
                    .service(
                        web::resource("/auth")
                            .route(web::put().to_async(svc::auth::router::signup))
                            .route(web::post().to_async(svc::auth::router::signin))
                            .route(web::get().to_async(svc::auth::router::getme)),
                    )
                    .service(
                        web::resource("/users/{id}")
                            .route(web::get().to_async(svc::auth::router::getme)),
                    )
                    .service(
                        web::resource("/shops")
                            .route(web::put().to_async(svc::shop::router::put))
                            .route(web::post().to(svc::shop::router::post)),
                    )
                    .service(
                        web::resource("/options")
                            .route(web::put().to_async(svc::option::router::put)),
                    )
                    .service(
                        web::resource("/option_groups")
                            .route(web::put().to_async(svc::option_group::router::put))
                            .route(web::post().to_async(svc::option_group::router::post)),
                    ),
            )
            .service(index)
            .service(no_params)
      
    })
    .bind(domain)?
    .workers(1)
    .start();

    sys.run()
}
