use crate::errors::ServiceError;
use crate::models::DbExecutor;
use crate::svc::auth::model::{AuthUser, Info};
use crate::svc::valid::model::New;
use crate::utils::jwt::{create_token, decode_token};
use crate::utils::validator::Validate;
use actix::Addr;
use actix_web::{
    get, post, put,
    web::{self, Data, Json, Path},
    Error, HttpRequest, HttpResponse, Responder, ResponseError,
};
use futures::{future::result, Future};
use uuid::Uuid;

pub fn valid_email(
    auth_user: AuthUser,
    path_info: Path<Info>,
    req: HttpRequest,
    json: Json<New>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let mut info = path_info.into_inner();
    info.auth_user = Some(auth_user);
    println!("1111111111{:?}", info);
    println!("1111111111{:?}", json);
    let j = json.into_inner();
    db.send(info)
        .from_err()
        .and_then(move |_| {
            println!("999999999999999");
            let res = db.send(j).from_err();
            println!("888888888888");
            res
        })
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn valid_phone(
    auth_user: AuthUser,
    path_info: Path<Info>,
    req: HttpRequest,
    json: Json<New>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let mut info = path_info.into_inner();
    info.auth_user = Some(auth_user);
    db.send(info)
        .from_err()
        .and_then(move |_| db.send(json.into_inner()).from_err())
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}
