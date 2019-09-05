use crate::api::v1::ceo::auth::model::{AuthUser, Info};
use crate::api::v1::ceo::device::model as params;
use crate::errors::ServiceError;
use crate::models::device as m;
use crate::models::msg::Msg;
use crate::models::shop::UpdateNotificationKey;
use crate::models::{AppStateWithTxt, DbExecutor};
use crate::utils::validator::Validate;
use actix::Addr;
use actix_web::{
    client::Client,
    delete,
    http::{header, StatusCode},
    web::{BytesMut, Data, Json, Path},
    Error, HttpResponse, ResponseError,
};
use futures::{
    future::{err, result, Either, IntoFuture},
    Future, Stream,
};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct FcmResponse {
    pub notification_key: String,
}

pub fn check(
    json: Json<params::InpCheck>,
    auth_user: AuthUser,
    db: Data<Addr<DbExecutor>>,
    client: Data<Client>,
    txt: Data<AppStateWithTxt>,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    let sw_token = json.into_inner().sw_token.clone();
    let sw_token2 = sw_token.clone();
    let vec = vec![sw_token2.to_string()];
    let webpush_group_reg_url = txt.webpush_group_reg_url.clone();
    let key = txt.get_key();
    let db2 = db.clone();

    db.send(m::Get {
        sw_token: sw_token,
        user_id: auth_user.id,
    })
    .from_err()
    .and_then(move |res| match res {
        Ok(get_with_key) => {
            let opt_send_data = get_with_key.get();
            if opt_send_data.is_some() {
                let send_data = opt_send_data.unwrap();

                Either::A(
                    send(send_data, client, txt, db2)
                        .map_err(|e| {
                            println!("sw push error : {:?}", e.error_response());
                            ServiceError::BadRequest("sw push error".into())
                        })
                        .then(|res| match res {
                            Ok(user) => Ok(HttpResponse::Ok().json("2")),
                            Err(_) => Ok(HttpResponse::InternalServerError().into()),
                        }),
                )
            } else {
                Either::B(err(ServiceError::BadRequest("sw push error".into())))
            }
        }
        Err(e) => Either::B(err(ServiceError::BadRequest("sw push error".into()))),
    })
}

pub fn send(
    send_data: m::SendData,
    client: Data<Client>,
    txt: Data<AppStateWithTxt>,
    db2: Data<Addr<DbExecutor>>,
) -> impl Future<Item = std::result::Result<Msg, ServiceError>, Error = ServiceError> {
    println!("==============================================");
    println!("send: {:?}", send_data);
    println!("==============================================");
    let shop_id = Uuid::parse_str(&send_data.notification_key_name.clone()).unwrap();

    let resp = client
        .post(txt.webpush_group_reg_url.clone())
        .header(header::CONTENT_TYPE, "application/json")
        .header("Authorization", txt.get_key())
        .header("project_id", "371794845174".to_string())
        .send_json(&send_data)
        .map_err(|e| {
            println!("sw push error : {:?}", e.error_response());
            ServiceError::BadRequest("sw push error".into())
        })
        .and_then(|response| {
            let _notification_key = response
                .from_err()
                .fold(BytesMut::new(), |mut acc, chunk| {
                    acc.extend_from_slice(&chunk);
                    Ok::<_, ServiceError>(acc)
                })
                .map(|body| {
                    let body: FcmResponse = serde_json::from_slice(&body).unwrap();
                    println!("==============================================");
                    println!("response.body(): {:?}", body);
                    println!("==============================================");
                    body.notification_key
                });
            _notification_key
        });
    resp.and_then(move |notification_key| {
        println!("==============================================");
        println!("response.body() notification_key: {:?}", notification_key);
        println!("==============================================");
        db2.send(UpdateNotificationKey {
            id: shop_id,
            notification_key: notification_key,
        })
        .from_err()
    })
}

pub fn put(
    json: Json<params::InpNew>,
    auth_user: AuthUser,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    result(json.validate())
        .from_err()
        .and_then(move |_| db.send(json.into_inner().new(auth_user)).from_err())
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn get(
    auth_user: AuthUser,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    db.send(m::GetList {
        user_id: auth_user.id,
    })
    .from_err()
    .and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(err) => Ok(err.error_response()),
    })
}

pub fn post(
    json: Json<params::InpUpdate>,
    auth_user: AuthUser,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    result(json.validate())
        .from_err()
        .and_then(move |_| db.send(json.into_inner().update(auth_user)).from_err())
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}
