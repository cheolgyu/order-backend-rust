use crate::api;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
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
                    ).service(
                        web::resource("/valid_phone")
                            .route(web::put().to_async(api::v1::ceo::valid::router::valid_phone)),
                    ).service(
                        web::resource("/device")
                            .route(web::put().to_async(api::v1::ceo::device::router::put))
                            .route(web::post().to_async(api::v1::ceo::device::router::check)),
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
                                    )
                                    .service(
                                        web::scope("/order")
                                            .service(web::resource("")
                                            .route(web::get().to_async(api::v1::ceo::order::router::get_list))
                                            )
                                            .service(web::resource("/now")
                                            .route(web::get().to_async(api::v1::ceo::order::router::now_list))
                                            )
                                            .service(
                                                web::scope("/{order_id}").service(
                                                    web::resource("")
                                                        .route(web::post().to_async( api::v1::ceo::order::router::post ))
                                                        ,
                                                ),
                                            ),
                                    ).service(
                                        web::scope("/order_detail")
                                            .service(web::resource("")
                                            .route( web::put().to_async( api::v1::ceo::order_detail::router::put)  )
                                            )
                                            ,
                                    ),
                            ),
                    ),
            )
    );
}
