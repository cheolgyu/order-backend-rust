use crate::errors::ServiceError;
use crate::models::DbExecutor;
use crate::svc::auth::model::AuthUser;
use crate::svc::product::model::{InpNew, New};
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
    println!("00000000000000000000");
    result(json.validate())
        .from_err()
        .and_then(move |_| {
            db.send(json.into_inner().new(shop_id.into_inner()))
                .from_err()
        })
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}

fn get() -> &'static str {
    "Hello world! get \r\n"
}

pub fn post() -> impl Responder {
    format!("Hello {}! id:{}", 1, 0)
}

fn delete() -> &'static str {
    "Hello world! post\r\n"
}
