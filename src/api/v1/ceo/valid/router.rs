use crate::api::v1::ceo::auth::model::ReqInfo;
use crate::api::v1::ceo::valid::model::{ChkValid, InpNew, New};

use crate::models::DbExecutor;

use crate::utils::client::SSLClinet;
use actix::Addr;
use actix_web::web::BytesMut;
use actix_web::{
    web::{Data, Json},
    Error, HttpResponse, ResponseError,
};
use chrono::format::strftime::StrftimeItems;
use chrono::{Duration, Local};
use futures::{Future, Stream};

#[derive(Debug, Deserialize)]
struct HttpBinResponse {
    status: bool,
}

pub fn valid_email(
    req_info: ReqInfo,
    url_valid_email: Data<String>,
    json: Json<InpNew>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let code = format!("{}", uuid::Uuid::new_v4());
    let valid_at = Local::now().naive_local() + Duration::hours(24);
    let j = json.into_inner();
    let api = url_valid_email.to_string();

    let n = New {
        user_id: req_info.req_u_id(),
        kind: j.kind,
        kind_value: j.kind_value,
        code: code,
        valid_at: valid_at,
        req: "false".to_string(),
    };
    let mut nn = n.clone();

    let fmt = StrftimeItems::new("%Y-%m-%d_%H:%M:%S");

    let url = format!(
        "{}?mail_to={}&token={}&valid_at={}",
        api,
        n.kind_value,
        n.code,
        n.valid_at.format_with_items(fmt.clone()).to_string()
    );

    SSLClinet::build()
        .get(url)
        .header("User-Agent", "Actix-web")
        .send()
        .map_err(Error::from)
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
    json: Json<ChkValid>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    db.send(json.into_inner())
        .from_err()
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn valid_phone(
    json: Json<New>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    db.send(json.into_inner())
        .from_err()
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}
