use crate::api::v1::ceo::auth::model::{AuthUser, Info};
use crate::api::v1::ceo::product::model::{Get, GetList, InpDelete, InpNew, InpUpdate};

use crate::models::DbExecutor;

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
    _auth_user: AuthUser,
    info: Path<Info>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    result(json.validate())
        .from_err()
        .and_then(move |_| {
            let j = json.into_inner();
            db.send(j.new(info.into_inner().shop_id.unwrap()))
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
    let sid = info2.shop_id.unwrap();
    let uusid = Uuid::parse_str(&sid).unwrap();
    info.auth_user = Some(auth_user);
    let db2 = db.clone();

    db.send(info)
        .from_err()
        .and_then(move |_| db2.send(GetList { shop_id: uusid }))
        .from_err()
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
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
