use crate::api::v1::ceo::auth::model::{AuthUser, Info};
use crate::api::v1::ceo::order::model::{Get, GetList,InpUpdate};

use crate::models::DbExecutor;

use crate::utils::validator::Validate;
use actix::Addr;
use actix_web::{
    web::{Data, Json, Path},
    Error, HttpResponse, ResponseError,
};
use futures::{future::result, Future};
use uuid::Uuid;

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
    let shop_id = info2.shop_id.unwrap();

    db.send(info)
        .from_err()
        .and_then(move |_| {
            db2.send(GetList {
                shop_id: Uuid::parse_str(&shop_id).unwrap(),
            })
        })
        .from_err()
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}
