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
mod errors;
mod middleware;
mod models;
mod schema;
mod utils;
use crate::models::DbExecutor;
use actix_cors::Cors;
use actix_web::{
    client::Client,
    get,
    http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    middleware as actix_middleware, web, App, HttpRequest, HttpServer,
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
    let url_frontend_user: String =
        env::var("URL_FRONTEND_USER").expect("URL_FRONTEND_USER must be set");
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
                Cors::new()
              .allowed_origin(&url_frontend_ceo)
              .allowed_origin(&url_frontend_user)
              .allowed_methods(vec!["GET", "POST", "PUT", "OPTIONS","DELETE"])
              .allowed_headers(vec![AUTHORIZATION, ACCEPT])
              .allowed_header(CONTENT_TYPE)
              .max_age(3600)
            )
            .service(web::resource("/ws/").route(web::get().to(models::ws::ws_index)))
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("ceo")
                        .service(
                            web::resource("/auth")
                            .route(web::put().to_async(api::v1::ceo::auth::router::signup))
                            .route(web::post().to_async(api::v1::ceo::auth::router::signin))
                            .route(web::get().to_async(api::v1::ceo::auth::router::getme))
                        )
                        .service(
                            web::scope("/{ceo_id}")
                                .service(
                                    web::resource("")
                                        .route(web::get().to_async(api::v1::ceo::auth::router::getme)),
                                )
                                .service(
                                    web::resource("/valid_email")
                                        .route(web::put().to_async(api::v1::ceo::valid::router::valid_email))
                                        .route(web::post().to_async(api::v1::ceo::valid::router::chk_valid_email)),
                                ) .service(
                                    web::resource("/valid_phone")
                                        .route(web::put().to_async(api::v1::ceo::valid::router::valid_phone)),
                                )
                                .service(
                                    web::scope("/shops")
                                        .service(
                                            web::resource("")
                                                .route(web::put().to_async(api::v1::ceo::shop::router::put))
                                                .route(web::post().to_async(api::v1::ceo::shop::router::post)),
                                        )
                                        .service(
                                            web::scope("/{shop_id}")
                                                .service(web::resource("").route(web::get().to_async(api::v1::ceo::shop::router::get), ))
                                                .service(
                                                    web::scope("/products")
                                                        .service(
                                                            web::resource("")
                                                                .route( web::put().to_async( api::v1::ceo::product::router::put) )
                                                                .route(web::get().to_async(api::v1::ceo::product::router::get_list)))
                                                        .service(
                                                            web::scope("/{product_id}").service(
                                                                web::resource("")
                                                                    .route(web::post().to_async( api::v1::ceo::product::router::post ))
                                                                    .route(web::get().to_async( api::v1::ceo::product::router::get))
                                                                    .route( web::delete().to_async( api::v1::ceo::product::router::delete) ),
                                                            ),
                                                        ),
                                                ).service(
                                                        web::scope("/option_group")
                                                            .service(web::resource("")
                                                            .route( web::put().to_async( api::v1::ceo::option_group::router::put)  )
                                                            .route(web::get().to_async(api::v1::ceo::option_group::router::get_list))
                                                            .route(web::post().to_async( api::v1::ceo::option_group::router::post ))
                                                            )
                                                            .service(
                                                                web::scope("/{option_group_id}").service(
                                                                    web::resource("")
                                                                        .route(web::get().to_async( api::v1::ceo::option_group::router::get))
                                                                        .route( web::delete().to_async( api::v1::ceo::option_group::router::delete)  )
                                                                ),
                                                            ),
                                                )
                                                .service(
                                                    web::scope("/option")
                                                        .service(web::resource("")
                                                        .route( web::put().to_async( api::v1::ceo::option::router::put)  )
                                                        .route(web::get().to_async(api::v1::ceo::option::router::get_list))
                                                        .route(web::post().to_async( api::v1::ceo::option::router::post ))
                                                        )
                                                        .service(
                                                            web::scope("/{option_id}").service(
                                                                web::resource("")
                                                                    .route(web::get().to_async( api::v1::ceo::option::router::get))
                                                                    .route( web::delete().to_async( api::v1::ceo::option::router::delete)  )
                                                                    ,
                                                            ),
                                                        ),
                                                ),
                                        ),
                                ),
                        )
                    )
                     .service(
                        web::scope("user") 
                            // 코드 위치에 따른 우선순위 탐. 주의 
                            // 1. user/shops 
                            .service(
                                web::scope("/shops")
                                    .service(
                                        web::resource("")
                                            .route(web::get().to_async(api::v1::user::shop::router::get_list)),
                                    )
                            )
                            // 2. user/{shop_id} 
                            .service(
                                web::scope("/{shop_id}")
                                .service(
                                    web::resource("")
                                        .route(web::get().to_async(api::v1::user::shop::router::get)),
                                )
                            )
                    )
                       
            )
            .service(index)
            .service(no_params)
    })
    .bind(domain)?
    .workers(1)
    .start();

    sys.run()
}
