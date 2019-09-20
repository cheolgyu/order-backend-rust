use crate::errors::ServiceError;
use crate::models::fcm::{ParamsToFcm, ParamsToUser, ToFcmResp, ToUserResp};
use crate::models::msg::Msg;
use crate::models::{DbExecutor, WebPush};
use actix::Addr;
use actix_web::{
    client::Client,
    http::header::CONTENT_TYPE,
    web::{BytesMut, Data},
    ResponseError,
};
use futures::stream::Stream;
use futures::Future;
// 1. params에서 제외 db,webpush 컨텍스트에서 가져오기
// 2. ParamsToFcm ParamsToUser 정리.

pub fn to_fcm(
    send_data: ParamsToFcm,
    webpush: WebPush,
    client: Data<Client>,
    _db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = Result<Msg, ServiceError>, Error = ServiceError> {
    let resp = client
        .post(send_data.url.clone())
        .header(CONTENT_TYPE, "application/json")
        .header("Authorization", webpush.key.clone())
        .header("project_id", webpush.send_id.clone())
        .send_json(&send_data.params)
        .map_err(|e| ServiceError::BadRequest("to_fcm err ".into()))
        .and_then(|response| {
            response
                .from_err()
                .fold(BytesMut::new(), |mut acc, chunk| {
                    acc.extend_from_slice(&chunk);
                    Ok::<_, ServiceError>(acc)
                })
                .map(|body| {
                    let body: ToFcmResp =
                        serde_json::from_slice(&body).expect("to_fcm body 변환 오류");

                    body
                })
        });
    resp.and_then(move |res| _db.send(res.new(send_data.order_id.clone())).from_err())
}

pub fn to_user(
    send_data: ParamsToUser,
    client: Data<Client>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = Result<Msg, ServiceError>, Error = ServiceError> {
    let _db = db.clone();

    let resp = Client::new()
        .post(send_data.url.clone())
        .header(CONTENT_TYPE, "application/json")
        .header("Authorization", send_data.webpush.key.clone())
        .send_json(&send_data.params)
        .map_err(|e| ServiceError::BadRequest("to_user ".into()))
        .and_then(|response| {
            let res = response
                .from_err()
                .fold(BytesMut::new(), |mut acc, chunk| {
                    acc.extend_from_slice(&chunk);
                    Ok::<_, ServiceError>(acc)
                })
                .map(|body| {
                    let body: ToUserResp =
                        serde_json::from_slice(&body).expect("to_user body 변환 오류");
                    body
                });
            res
        });
    resp.and_then(move |res| _db.send(res.new(send_data.order_id.clone())).from_err())
}
