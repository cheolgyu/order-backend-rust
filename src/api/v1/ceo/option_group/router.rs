use crate::api::v1::ceo::auth::model::ReqInfo;
use crate::api::v1::ceo::option_group::model::{Get, GetList, InpDelete, InpNew, InpUpdate};

use crate::models::DbExecutor;

use crate::utils::validator::Validate;
use actix::Addr;
use actix_web::{
    web::{Data, Json},
    Error, HttpResponse, ResponseError,
};
use futures::{future::result, Future};

pub fn put(
    json: Json<InpNew>,
    req_info: ReqInfo,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let j = json.into_inner();

    result(j.validate())
        .from_err()
        .and_then(move |_| db.send(j.new(req_info.req_s_id())).from_err())
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn post(
    req_info: ReqInfo,
    json: Json<InpUpdate>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let j = json.into_inner();

    result(j.validate())
        .and_then(move |_| db.send(j.new(req_info.req_s_id())).from_err())
        .from_err()
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn delete(
    req_info: ReqInfo,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let j = InpDelete {
        id: req_info.req_target_id(),
    };

    result(j.validate())
        .and_then(move |_| db.send(j.new(req_info.req_s_id())).from_err())
        .from_err()
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}
pub fn get(
    json: Json<Get>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let j = json.into_inner();

    db.send(j).from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(e) => Ok(e.error_response()),
    })
}

pub fn get_list(
    req_info: ReqInfo,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    db.send(GetList {
        shop_id: req_info.req_s_id(),
    })
    .from_err()
    .and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(e) => Ok(e.error_response()),
    })
}
