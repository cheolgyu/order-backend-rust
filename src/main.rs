#![allow(unused_imports)]

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
mod errors;
mod middleware;
mod models;
mod schema;
mod svc;
mod utils;
use crate::models::DbExecutor;
use actix_web::{
    client::Client, get, http::header, middleware as actix_middleware, web, App, HttpRequest,
    HttpServer,
};
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
    let url_frontend_ceo: String =
        env::var("URL_FRONTEND_CEO").expect("URL_FRONTEND_CEO must be set");
    let valid_email: String = env::var("VALID_EMAIL").expect("VALID_EMAIL must be set");
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

    HttpServer::new(move || {
        App::new()
            .data(address.clone())
            .data(pool2.clone())
            .data(Client::default())
            .data(valid_email.clone())
            .wrap(actix_middleware::Logger::default())
            .wrap(
                actix_middleware::cors::Cors::new()
                    .allowed_origin(&url_frontend_ceo)
                    .allowed_methods(vec!["GET", "POST", "PUT", "OPTIONS"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .service(web::resource("/ws/").route(web::get().to(models::ws::ws_index)))
            .service(
                web::scope("/api/v1")
                    .service(
                        web::resource("/auth")
                            .route(web::put().to_async(svc::auth::router::signup))
                            .route(web::post().to_async(svc::auth::router::signin))
                            .route(web::get().to_async(svc::auth::router::getme)),
                    )
                    .service(
                        web::scope("/users").service(
                            web::scope("/{user_id}")
                                .service(
                                    web::resource("")
                                        .route(web::get().to_async(svc::auth::router::getme)),
                                )
                                .service(
                                    web::resource("/valid_email")
                                        .route(web::put().to_async(svc::valid::router::valid_email)),
                                ) .service(
                                    web::resource("/valid_phone")
                                        .route(web::put().to_async(svc::valid::router::valid_phone)),
                                )
                                .service(
                                    web::scope("/shops")
                                        .service(
                                            web::resource("")
                                                .route(web::put().to_async(svc::shop::router::put))
                                                .route(web::post().to(svc::shop::router::post)),
                                        )
                                        .service(
                                            web::scope("/{shop_id}")
                                                .service(web::resource("").route(
                                                    web::get().to_async(svc::shop::router::get),
                                                ))
                                                .service(
                                                    web::scope("/products")
                                                        .service(web::resource("").route(
                                                            web::put().to_async(
                                                                svc::product::router::put,
                                                            ),
                                                        ))
                                                        .service(
                                                            web::scope("/{product_id}").service(
                                                                web::resource("")
                                                                    .route(web::post().to_async(
                                                                        svc::product::router::post,
                                                                    ))
                                                                    .route(web::get().to_async(
                                                                        svc::product::router::get,
                                                                    )),
                                                            ),
                                                        ),
                                                ),
                                        ),
                                ),
                        )// .wrap(middleware::auth::Auth),
                    ), /*
                       .service(
                           web::resource("/users/{user_id}/shops")
                               .route(web::put().to_async(svc::shop::router::put))
                               .route(web::post().to(svc::shop::router::post)),
                       )
                       .service(
                           web::resource("/users/{user_id}/shops/{shop_id}")
                               .route(web::get().to_async(svc::shop::router::get)),
                       )
                       .service(
                           web::resource("/users/{user_id}/shops/{shop_id}/products")
                               .route(web::put().to_async(svc::product::router::put)),
                       )
                       .service(
                           web::resource("/users/{user_id}/shops/{shop_id}/products/{product_id}")
                               .route(web::post().to_async(svc::product::router::post))
                               .route(web::get().to_async(svc::product::router::get)),
                       ),
                       */
                       /*
                          .service(
                              web::resource("/options")
                                  .route(web::put().to_async(svc::option::router::put)),
                          )
                          .service(
                              web::resource("/option_groups")
                                  .route(web::put().to_async(svc::option_group::router::put))
                                  .route(web::post().to_async(svc::option_group::router::post)),
                          ),
                          */
            )
            .service(index)
            .service(no_params)
    })
    .bind(domain)?
    .workers(1)
    .start();

    sys.run()
}
