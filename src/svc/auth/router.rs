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
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    db.send(json.into_inner())
        .map_err(|_| ServiceError::InternalServerError)
        .and_then(move |res| match res {
            Ok(_user) => Ok(HttpResponse::Ok().into()),
            Err(err) => Ok(err.error_response()),
        })

    /*
        println!("REQ: {:?}", req);
        println!("REQ: {:?}", json.login.validate());
        println!("REQ: {:?}", json.validate());
        format!("Hello signup: {:?}!\r\n", json)
        && json.validate()
    */
    /*
        result(json.login.validate())
            .from_err()
            .and_then(move |_| db.send(json.into_inner()).from_err())
            .and_then(|res| match res {
                Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                Err(e) => Ok(e.error_response()),
            })
    */
    /*
    Box::new(
        db.send(json.into_inner())
            .from_err()
            .and_then(move |res| match res {
                Ok(user) => Ok(HttpResponse::Ok().into()),
                Err(err) => Ok(err.error_response()),
            }),
    )
    */
}

#[post("/signin")]
fn signin(req: HttpRequest) -> String {
    println!("REQ: {:?}", req);
    format!("Hello signin: {:?}!\r\n", req)
}
