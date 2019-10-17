use crate::api::v1::ceo::auth::model::{AuthUser, LoginUser, Target};
use crate::api::v1::ceo::shop::model::{InpNew, InpUpdate, NewShop, ShopID};

use crate::models::DbExecutor;

use crate::utils::validator::Validate;
use actix::Addr;
use actix_web::{
    delete,
    web::{Data, Json, Path},
    Error, HttpResponse, ResponseError,
};
use futures::{future::result, Future};

pub fn post(
    json: Json<InpNew>,
    auth_user: LoginUser,
    _path_info: Path<String>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    result(json.validate())
        .from_err()
        //.and_then(move |_| db.send(NewShop::from(json.into_inner(),tg.u_id)).from_err())
        .and_then(move |_| db.send(json.into_inner().new_shop(auth_user)).from_err())
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn get(
    tg: Path<Target>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    db.send(ShopID { id: tg.s_id() })
        .from_err()
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(err) => Ok(err.error_response()),
        })
}

pub fn put(
    json: Json<InpUpdate>,
    login_user: LoginUser,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    result(json.validate())
        .from_err()
        .and_then(move |_| {
            db.send(json.into_inner().update_shop(login_user))
                .from_err()
        })
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}

#[delete("/shops/{shop_id}")]
fn delete() -> &'static str {
    "Hello world! post\r\n"
}
