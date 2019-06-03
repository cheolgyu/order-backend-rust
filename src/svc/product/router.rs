use crate::errors::ServiceError;
use crate::models::DbExecutor;
use crate::svc::auth::model::{AuthUser, Info};
use crate::svc::product::model::{Get, InpNew, InpUpdate, New,GetList};
use crate::utils::jwt::{create_token, decode_token};
use crate::utils::validator::Validate;
use actix::Addr;
use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json, Path, Query},
    Error, HttpRequest, HttpResponse, Responder, ResponseError,
};
use futures::{future::result, Future};
use uuid::Uuid;
use crate::svc::auth::model::Ceo;

pub fn put(
    json: Json<InpNew>,
    auth_user: AuthUser,
    info: Path<Info>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    result(json.validate())
        .from_err()
        .and_then(move |_| {
            let j = json.into_inner();
            db.send(j.new(info.into_inner().shop_id.unwrap(), j.option_group.clone()))
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
        .and_then(move |_| db2.send(j.new(j.option_group.clone())).from_err())
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
    let  info = path_info.into_inner();
    info.auth_user = Some(auth_user);
    let db2 = db.clone();
 
    db.send(info).from_err()
    /*
    .and_then(|msg| match msg  {
        Ok(m)=>m,
        Err(e)=>{
            println!("{:?}",e);
        }
    })
    */
    .and_then(|msg| match msg {
        Ok(m)=>{
             db2.send(GetList{shop_id: m.get_shop_id()})
        },Err(e) => println!("errrr"), 
    } )
    .from_err()
    .and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(e) => Ok(e.error_response()),
    })
}

fn delete() -> &'static str {
    "Hello world! post\r\n"
}
