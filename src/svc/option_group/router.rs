use crate::errors::ServiceError;
use crate::models::DbExecutor;
use crate::svc::auth::model::Ceo;
use crate::svc::auth::model::{AuthUser, Info};
use crate::svc::option_group::model::{Get, GetList, InpNew, InpUpdate, New};
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
    path_info: Path<Info>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let mut info = path_info.into_inner();
    info.auth_user = Some(auth_user);
    let j = json.into_inner();
    let db2 = db.clone();

    result(j.validate())
        .from_err()
        .and_then(move |_| db.send(info).from_err())
        .and_then(move |_| db2.send(j.new()).from_err())
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn post(
    json: Json<InpUpdate>,
    auth_user: AuthUser,
    path_info: Path<Info>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let mut info = path_info.into_inner();
    info.auth_user = Some(auth_user);
    let j = json.into_inner();
    let db2 = db.clone();

    result(j.validate())
        .and_then(move |_| db.send(info).from_err())
        .and_then(move |_| db2.send(j.new()).from_err())
        .from_err()
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}
pub fn get(
    json: Json<Get>,
    auth_user: AuthUser,
    path_info: Path<Info>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    println!("path_info:{:?}", path_info);
    let mut info = path_info.into_inner();
    info.auth_user = Some(auth_user);
    let j = json.into_inner();
    let db2 = db.clone();

    db.send(info)
        .and_then(move |_| db2.send(j).from_err())
        .from_err()
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn get_list(
    auth_user: AuthUser,
    path_info: Path<Info>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    println!("path_info:{:?}", path_info);
    let mut info = path_info.into_inner();
    let info2 = info.clone();
    info.auth_user = Some(auth_user);
    let db2 = db.clone();

    db.send(info)
        .from_err()
        .and_then(move |_| db2.send(GetList {}))
        .from_err()
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}
