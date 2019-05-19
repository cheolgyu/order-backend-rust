use crate::errors::ServiceError;
use crate::models::DbExecutor;
use crate::svc::auth::model::AuthUser;
use crate::svc::product::model::{Get, InpNew, InpUpdate, New};
use crate::utils::jwt::{create_token, decode_token};
use crate::utils::validator::Validate;
use actix::Addr;
use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json, Path},
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

pub fn post(
    json: Json<InpUpdate>,
    auth_user: AuthUser,
    path_info: Path<(String, i32)>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    result(json.validate())
        .from_err()
        .and_then(move |_| {
            let j = json.into_inner();
            db.send(j.new(j.option_group.clone())).from_err()
        })
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}
pub fn get(
    json: Json<Get>,
    auth_user: AuthUser,
    shop_id: Path<(String, i32)>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
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
