use crate::errors::ServiceError;
use crate::models::DbExecutor;
use crate::svc::auth::model::{AuthUser, Info};
use crate::svc::valid::model::{InpNew, New};
use crate::utils::jwt::{create_token, decode_token};
use crate::utils::validator::Validate;
use actix::Addr;
use actix_web::client::Client;
use actix_web::web::BytesMut;
use actix_web::{
    get, post, put,
    web::{self, Data, Json, Path},
    Error, HttpRequest, HttpResponse, Responder, ResponseError,
};
use chrono::format::strftime::StrftimeItems;
use chrono::{Duration, Local};
use futures::{future::result, Future, Stream};
use std::collections::HashMap;
use std::io;
use uuid::Uuid;
#[derive(Debug, Deserialize)]
struct HttpBinResponse {
    status: bool,
}

pub fn valid_email(
    auth_user: AuthUser,
    path_info: Path<Info>,
    client: Data<Client>,
    url_valid_email: Data<String>,
    json: Json<InpNew>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let mut info = path_info.into_inner();
    info.auth_user = Some(auth_user);
    let code = format!("{}", uuid::Uuid::new_v4());
    let valid_at = Local::now().naive_local() + Duration::hours(24);
    let j = json.into_inner();
    let api = url_valid_email.to_string();

    let n = New {
        user_id: j.user_id,
        kind: j.kind,
        kind_value: j.kind_value,
        code: code,
        valid_at: valid_at,
        req: "false".to_string(),
    };
    let mut nn = n.clone();

    db.send(info)
        .from_err()
        .and_then(move |_| {
            let fmt = StrftimeItems::new("%Y-%m-%d_%H:%M:%S");

            let url = format!(
                "{}?mail_to={}&token={}&valid_at={}",
                api,
                n.kind_value,
                n.code,
                n.valid_at.format_with_items(fmt.clone()).to_string()
            );

            client
                .get(url)
                .header("User-Agent", "Actix-web")
                .send()
                .map_err(Error::from)
        })
        .and_then(|resp| {
            resp.from_err()
                .fold(BytesMut::new(), |mut acc, chunk| {
                    acc.extend_from_slice(&chunk);
                    Ok::<_, Error>(acc)
                })
                .map(|body| {
                    let body: HttpBinResponse = serde_json::from_slice(&body).unwrap();
                    body.status
                })
        })
        .and_then(move |status| {
            nn.req = status.to_string();
            let res = db.send(nn).from_err();
            res
        })
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn chk_valid_email(
    auth_user: AuthUser,
    path_info: Path<Info>,
    client: Data<Client>,
    url_valid_email: Data<String>,
    json: Json<New>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let mut info = path_info.into_inner();
    info.auth_user = Some(auth_user);
    println!("1111111111{:?}", info);
    println!("1111111111{:?}", json);
    //인증코드 만들기
    let code = format!("{}", uuid::Uuid::new_v4());
    let valid_at = Local::now().naive_local() + Duration::hours(24);
    let mut j = json.into_inner();
    let api = url_valid_email.to_string();
    j.code = code;
    j.valid_at = valid_at;
    let jj = j.clone();

    db.send(info)
        .from_err()
        .and_then(move |_| {
            println!("00000000000000000000");
            let url = format!(
                "{}?mail_to={}&token={}&valid_at={}",
                api,
                j.kind_value,
                j.code,
                valid_at.to_string()
            );
            client
                .get(url)
                .header("User-Agent", "Actix-web")
                .send()
                .map_err(Error::from)
        })
        .and_then(|resp| {
            println!("1111111111111111");
            resp.from_err()
                .fold(BytesMut::new(), |mut acc, chunk| {
                    acc.extend_from_slice(&chunk);
                    Ok::<_, Error>(acc)
                })
                .map(|body| {
                    let body: HttpBinResponse = serde_json::from_slice(&body).unwrap();
                    body.status
                })
        })
        .and_then(move |status| {
            //인증코드도 저장
            println!("999999999999999");
            let res = db.send(jj).from_err();
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