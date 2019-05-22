use crate::errors::ServiceError;
use crate::models::DbExecutor;
use crate::svc::auth::model::{AuthUser, Info};
use crate::svc::product::model::{Get, InpNew, InpUpdate, New};
use crate::utils::jwt::{create_token, decode_token};
use crate::utils::validator::Validate;
use actix::Addr;
use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json, Path, Query},
    Error, HttpRequest, HttpResponse, Responder, ResponseError,
};
use futures::{future::result, Future};
use uuid::Uuid;

pub fn put(
    json: Json<InpNew>,
    auth_user: AuthUser,
    shop_id: Path<String>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    result(json.validate())
        .from_err()
        .and_then(move |_| {
            let j = json.into_inner();
            db.send(j.new(shop_id.into_inner(), j.option_group.clone()))
                .from_err()
        })
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}
use crate::svc::auth::model::Ceo;
pub fn post(
    json: Json<InpUpdate>,
    auth_user: AuthUser,
    path_info: Path<Info>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    println!("path_info:{:?}", path_info);
    // result(json.validate())
    //     .from_err()
    db.send(path_info.into_inner())
           .from_err()
        /*
        .and_then(move |res| {
            // println!("ceo.user ==>{:?}", ceo.user);
            // println!("ceo.shop ==>{:?}", ceo.shop);
            // println!("ceo.product ==>{:?}", ceo.product);
            match res {
                Err(e) => print!("errr"),
                Ok(ceo) => {
                    println!("ceo.user ==>{:?}", ceo.user);
                    println!("ceo.shop ==>{:?}", ceo.shop);
                    println!("ceo.product ==>{:?}", ceo.product);
                    Ok(ceo)
                    //   let j = json.into_inner();
                    //   db.send(j.new(j.option_group.clone())).from_err()
                }
            }
        })
        */
        //.and_then(move |res| res)
        // .from_err()
        .and_then(move |res| {
            match res {
                Ok(ceo)=>{
                    println!("ceo.user ==>{:?}", ceo.user);
                    println!("ceo.shop ==>{:?}", ceo.shop);
                    println!("ceo.product ==>{:?}", ceo.product);
                }
                Err(e)=>{println!("ceo.eeeeeeeeee ==>{:?}", e);}
            };
           

            let j = json.into_inner();
            db.send(j.new(j.option_group.clone())).from_err()
        })
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
    /*

     db.send(path_info.into_inner())
        .from_err()
        .and_then(move |res| {
            let j = json.into_inner();
            db.send(j.new(j.option_group.clone())).from_err()
        })
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })

    */
}
pub fn get(
    json: Json<Get>,
    auth_user: AuthUser,
    path_info: Path<Info>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    println!("path_info:{:?}", path_info);

    db.send(json.into_inner())
        .from_err()
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}

fn delete() -> &'static str {
    "Hello world! post\r\n"
}
