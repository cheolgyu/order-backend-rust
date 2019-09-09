use crate::api::v1::ceo::fcm::model as params;
use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::models::fcm::{SendData,ToUserResp,ParamsToUser};
use crate::models::{DbExecutor, WebPush};
use actix::Addr;
use actix_web::{
    client::Client,
    http::header::CONTENT_TYPE,
    web::{BytesMut, Data},
    ResponseError
};
use futures::Future;
use futures::stream::Stream;

pub fn to_fcm(
    send_data: SendData,
    webpush: WebPush,
    client: Data<Client>,
    _db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = String, Error = ServiceError> {
    println!("==============================================");
    println!("to_fcm: {:?}", send_data.params.operation);
    println!("to_fcm: {:?}", send_data);
    println!("==============================================");
    let resp = client
        .post(send_data.url.clone())
        .header(CONTENT_TYPE, "application/json")
        .header("Authorization", webpush.key.clone())
        .header("project_id", webpush.send_id.clone())
        .send_json(&send_data.params)
        .map_err(|e| {
            println!("to_fcm err : {:?}", e.error_response());
            ServiceError::BadRequest("to_fcm err ".into())
        })
        .and_then(|response| {
            let _notification_key = response
                .from_err()
                .fold(BytesMut::new(), |mut acc, chunk| {
                    acc.extend_from_slice(&chunk);
                    Ok::<_, ServiceError>(acc)
                })
                .map(|body| {
                    println!("to_fcm: response.body(): {:?}", body);
                    let body: params::FcmResponse = serde_json::from_slice(&body).expect("to_fcm body 변환 오류");
                    println!("==============================================");
                    println!("to_fcm: response.body(): {:?}", body);
                    println!("==============================================");
                    body.notification_key
                });
            _notification_key
        });
    resp
}

pub fn to_user(
    send_data: ParamsToUser,
    client: Data<Client>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = Result<Msg, ServiceError>, Error = ServiceError> {
    let _db = db.clone();
    println!("==============================================");
    println!("to_user: {:?}", send_data);
    println!("==============================================");
    let resp = client
        .post(send_data.url.clone())
        .header(CONTENT_TYPE, "application/json")
        .header("Authorization", send_data.webpush.key.clone())
        .send_json(&send_data.params)
        .map_err(|e| {
            println!("to_user : {:?}", e.error_response());
            ServiceError::BadRequest("to_user ".into())
        })
        .and_then(|response| {
            let res = response
                .from_err()
                .fold(BytesMut::new(), |mut acc, chunk| {
                    acc.extend_from_slice(&chunk);
                    Ok::<_, ServiceError>(acc)
                })
                .map(|body| {
                    let body: ToUserResp = serde_json::from_slice(&body).unwrap();
                  
                    println!("==============================================");
                    println!("to_user: response.body(): {:?}", body);
                    println!("==============================================");
                    body
                });
            res
        });
    resp.and_then(move |res| _db.send(res.new(send_data.order_id.clone())).from_err() )
}
