use crate::api::v1::ceo::auth::model::{AuthUser, Info};
use crate::api::v1::ceo::shop::model::{InpNew, InpUpdate, ShopID};

use crate::models::DbExecutor;

use crate::utils::validator::Validate;
use actix::Addr;
use actix_web::{
    delete,
    web::{Data, Json, Path},
    Error, HttpResponse, ResponseError,
};
use futures::{future::result, Future};
use uuid::Uuid;

pub fn put(
    json: Json<InpNew>,
    auth_user: AuthUser,
    _path_info: Path<String>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    result(json.validate())
        .from_err()
        .and_then(move |_| db.send(json.into_inner().new_shop(auth_user)).from_err())
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn get(
    path_info: Path<Info>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let info = path_info.into_inner();
    let sid = info.shop_id.unwrap();
    let uuid_shop_id = sid;
    db.send(ShopID { id: uuid_shop_id })
        .from_err()
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(err) => Ok(err.error_response()),
        })
}

pub fn post(
    json: Json<InpUpdate>,
    auth_user: AuthUser,
    _path_info: Path<String>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    result(json.validate())
        .from_err()
        .and_then(move |_| db.send(json.into_inner().update_shop(auth_user)).from_err())
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}

#[delete("/shops/{shop_id}")]
fn delete() -> &'static str {
    "Hello world! post\r\n"
}
