use actix_web::{web};
use crate::api;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("user") 
            // 코드 위치에 따른 우선순위 탐. 주의 
            // 1. user/shops 
            .service(
                web::scope("/shops")
                    .service(
                        web::resource("")
                            .route(web::get().to_async(api::v1::user::shop::router::get_list)),
                    )
            ).service(
                web::scope("/order")
                    .service(
                        web::resource("")
                            .route(web::put().to_async(api::v1::user::order::router::put)),
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
    );
}