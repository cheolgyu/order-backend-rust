use crate::models::DbExecutor;
use crate::svc::auth::model::RegUser;
use crate::svc::errors::ServiceError;
use crate::svc::validator::Validate;
use actix::Addr;
use actix_web::{
    post, put,
    web::{self, Data, Json, Path},
    Error, HttpRequest, HttpResponse, Responder, ResponseError,
};
use futures::{future::result, Future};

pub fn signup(
    req: HttpRequest,
    json: Json<RegUser>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    result(json.validate())
        .from_err()
        .and_then(move |_| db.send(json.into_inner()).from_err())
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}

#[post("/signin")]
fn signin(req: HttpRequest) -> String {
    println!("REQ: {:?}", req);
    format!("Hello signin: {:?}!\r\n", req)
}
