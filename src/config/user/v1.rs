use crate::api;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    // 코드 위치에 따른 우선순위 탐. 주의
    // 1. user/shops
    cfg.service(web::scope("/shops").service(
        web::resource("").route(web::get().to_async(api::v1::user::shop::router::get_list)),
    ));
    cfg.service(
        web::scope("/order").service(
            web::resource("").route(web::put().to_async(api::v1::user::order::router::put)),
        ),
    );
    // 2. user/{shop_id}
    cfg.service(
        web::scope("/{shop_id}").service(
            web::resource("").route(web::get().to_async(api::v1::user::shop::router::get)),
        ),
    );
}
