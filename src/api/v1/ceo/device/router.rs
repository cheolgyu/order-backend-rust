use crate::api::v1::ceo::auth::model::AuthUser;
use crate::api::v1::ceo::device::model as params;
use crate::errors::ServiceError;
use crate::fcm::model::*;
use crate::fcm::router::to_fcm;
use crate::models::device as m;
use crate::models::shop::UpdateNotificationKey;
use crate::models::{AppStateWithTxt, DbExecutor};
use crate::utils::validator::Validate;
use actix::Addr;
use actix_web::{
    client::Client,
    web::{Data, Json},
    Error, HttpResponse, ResponseError,
};
use futures::{
    future::{err, result, Either},
    Future,
};

pub fn check(
    json: Json<params::InpCheck>,
    auth_user: AuthUser,
    db: Data<Addr<DbExecutor>>,
    client: Data<Client>,
    store: Data<AppStateWithTxt>,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    let sw_token = json.into_inner().sw_token.clone();
    let sw_token2 = sw_token.clone();
    let sw_token3 = sw_token.clone();
    let vec = vec![sw_token2.to_string()];

    let db2 = db.clone();

    let db4 = db.clone();

    let user_id = auth_user.id.clone();

    db.send(m::GetWithShop {
        sw_token: sw_token,
        user_id: auth_user.id,
    })
    .from_err()
    .and_then(move |res_opt| match res_opt {
        Ok(res) => match res.operation.as_str() {
            "create" | "add" => {
                let shop_id = res.shop_id.clone();
                let db3 = db.clone();
                Either::A(
                    to_fcm(
                        ReqToFcm {
                            order_id: -1,
                            params: ReqToFcmData {
                                operation: res.operation.clone(),
                                notification_key_name: shop_id.to_string(),
                                notification_key: res.notification_key.clone(),
                                registration_ids: vec,
                            },
                        },
                        db,
                        store,
                    )
                    .and_then(move |res| {
                        let msg = res.unwrap();
                        let shop_id2 = shop_id.clone();
                        let notification_key = msg.data["item"]["resp"]["notification_key"]
                            .as_str()
                            .unwrap();
                        db3.send(UpdateNotificationKey {
                            id: shop_id,
                            notification_key: notification_key.to_string(),
                        })
                        .from_err()
                    })
                    .and_then(move |_res| {
                        db4.send(m::New {
                            user_id: user_id,
                            name: "test".to_string(),
                            sw_token: sw_token3,
                        })
                        .from_err()
                    })
                    .map_err(|e| ServiceError::BadRequest(e.to_string()))
                    .then(|res| match res {
                        Ok(_user) => Ok(HttpResponse::Ok().json("2")),
                        Err(_) => Ok(HttpResponse::InternalServerError().into()),
                    }),
                )
            }
            "pass" => Either::B(result(Ok(HttpResponse::Ok().json("pass")))),
            "" => Either::B(err(ServiceError::BadRequest(
                "check device: whoareyou1".into(),
            ))),
            _ => Either::B(err(ServiceError::BadRequest(
                "check device: whoareyou1".into(),
            ))),
        },
        Err(e) => Either::B(err(ServiceError::BadRequest(e.to_string()))),
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
