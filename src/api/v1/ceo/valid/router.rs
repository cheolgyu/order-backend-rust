use crate::api::v1::ceo::auth::model::{AuthUser, Info};
use crate::api::v1::ceo::valid::model::{ChkValid, InpNew, New};

use crate::model::DbExecutor;

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
use futures::{Future, Stream};

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
    json: Json<ChkValid>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let mut info = path_info.into_inner();
    info.auth_user = Some(auth_user);

    db.send(info)
        .from_err()
        .and_then(move |_| {
            println!("999999999999999");
            let res = db.send(json.into_inner()).from_err();
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
