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

pub fn send_post(
    send_data: m::SendData,
    get_with_key: m::GetWithKey,
    client: Data<Client>,
    txt: Data<AppStateWithTxt>,
    db2: Data<Addr<DbExecutor>>,
) -> impl Future<Item = std::result::Result<Msg, ServiceError>, Error = ServiceError> {
    println!("==============================================");
    println!("send_post: {:?}", send_data);
    println!("==============================================");
    let shop_id = Uuid::parse_str(&send_data.notification_key_name.clone()).unwrap();
    let d3 = db2.clone();
    let resp = client
        .post(txt.webpush_group_reg_url.clone())
        .header(header::CONTENT_TYPE, "application/json")
        .header("Authorization", txt.get_key())
        .header("project_id", "371794845174".to_string())
        .send_json(&send_data)
        .map_err(|e| {
            println!("check device-send : {:?}", e.error_response());
            ServiceError::BadRequest("check device-send".into())
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
        println!("update shop notification_key  : {:?}", notification_key);
        println!("==============================================");
        db2.send(UpdateNotificationKey {
            id: shop_id,
            notification_key: notification_key,
        })
        .from_err()
    }).and_then(move |res| {
        println!("==============================================");
        println!("insert user device ");
        println!("==============================================");
        d3.send(m::New {
            user_id: get_with_key.params.user_id.clone(),
            name: "test".to_string(),
            sw_token: get_with_key.params.sw_token.clone(),
        })
        .from_err()
    })
}
