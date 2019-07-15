use crate::api::v1::ceo::auth::model::{AuthUser, Info};
use crate::api::v1::ceo::option::model::{Get, GetList, InpDelete, InpNew, InpUpdate};

use crate::api::v1::ceo::option::service;
use crate::model::DbExecutor;
use crate::utils::validator::Validate;
use actix::Addr;
use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json, Path, Query},
    Error, HttpRequest, HttpResponse, Responder, ResponseError,
};
use futures::{future::result, Future};
use uuid::Uuid;

use crate::model::db;

pub fn put(
    json: Json<InpNew>,
    auth_user: AuthUser,
    path_info: Path<Info>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let mut info = path_info.into_inner();
    info.auth_user = Some(auth_user);
    let info2 = info.clone();
    let j = json.into_inner();
    let db2 = db.clone();
    let shop_id = info2.shop_id.unwrap();

    result(j.validate())
        .from_err()
        .and_then(move |_| db.send(info).from_err())
        .and_then(move |_| db2.send(j.new(shop_id)).from_err())
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
    let info2 = info.clone();
    let j = json.into_inner();
    let db2 = db.clone();
    let shop_id = info2.shop_id.unwrap();

    result(j.validate())
        .and_then(move |_| db.send(info).from_err())
        .and_then(move |_| db2.send(j.new(shop_id)).from_err())
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
    pool: web::Data<db::PgPool>,
    auth_user: AuthUser,
    path_info: Path<Info>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let mut info = path_info.into_inner();
    let info2 = info.clone();
    info.auth_user = Some(auth_user);
    let db2 = db.clone();
    let shop_id = info2.shop_id.unwrap();

    let ss = Uuid::parse_str(&shop_id).unwrap();

    web::block(move || service::get_all(&ss, &pool))
        .from_err()
        .then(move |res| match res {
            Ok(tasks) => Ok(HttpResponse::Ok().json(tasks)),
            Err(e) => Err(e),
        })
}

pub fn delete(
    auth_user: AuthUser,
    path_info: Path<Info>,
    pinfo: Path<(String, String, i32)>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let j = InpDelete { id: pinfo.2 };
    let mut info = path_info.into_inner();
    info.auth_user = Some(auth_user);
    let info2 = info.clone();
    let db2 = db.clone();
    let shop_id = info2.shop_id.unwrap();

    result(j.validate())
        .and_then(move |_| db.send(info).from_err())
        .and_then(move |_| db2.send(j.new(shop_id)).from_err())
        .from_err()
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}
