use crate::errors::ServiceError;
use crate::models::DbExecutor;
use crate::svc::auth::model::{AuthUser, Info};
use crate::svc::shop::model::{InpNew, NewShop, ShopID};
use crate::utils::jwt::{create_token, decode_token};
use crate::utils::validator::Validate;
use actix::Addr;
use actix_web::{
    delete, error, get, post, put,
    web::{self, Data, Json, Path},
    Either, Error, HttpRequest, HttpResponse, Responder, ResponseError,
};
use futures::{future::result, Future};
use uuid::Uuid;

pub fn put(
    json: Json<InpNew>,
    auth_user: AuthUser,
    path_info: Path<String>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    result(json.validate())
        .from_err()
        //.and_then(|_| -> bool { auth_user.check_role(path_info.into_inner()) })
        .and_then(move |_| db.send(json.into_inner().new_shop(auth_user)).from_err())
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })

    /*
    result(json.validate())
        .from_err()
        .and_then(move |_| db.send(json.into_inner().new_shop(auth_user)).from_err())
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
        */
}

pub fn get(
    path_info: Path<Info>,
    auth_user: AuthUser,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    println!("shop ---g et ");
    let mut info = path_info.into_inner();
    let sid = info.shop_id.unwrap();
    let uuid_shop_id = Uuid::parse_str(&sid).unwrap();
    db.send(ShopID { id: uuid_shop_id })
        .from_err()
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(err) => Ok(err.error_response()),
        })
}

pub fn post() -> impl Responder {
    format!("Hello {}! id:{}", 1, 0)
}

#[delete("/shops/{shop_id}")]
fn delete() -> &'static str {
    "Hello world! post\r\n"
}
