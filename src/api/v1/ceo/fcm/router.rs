use crate::api::v1::ceo::fcm::model as params;
use crate::models::fcm::SendData;
use crate::errors::ServiceError;
use crate::models::{WebPush, DbExecutor};
use actix::Addr;
use actix_web::{
    client::Client,
    delete,
    http::{header::{CONTENT_TYPE}, StatusCode},
    web::{BytesMut, Data, Json, Path},
    Error, HttpResponse, ResponseError,
};
use futures::{
    Future, Stream,
};


pub fn send (
    send_data: SendData,
    webpush: WebPush,
    client: Data<Client>,
    db2: Data<Addr<DbExecutor>>,
) -> impl Future<Item = String, Error = ServiceError> {
    println!("==============================================");
    println!("send: {:?}", send_data);
    println!("==============================================");
    let resp = client
        .post(send_data.url.clone())
        .header(CONTENT_TYPE, "application/json")
        .header("Authorization", webpush.key.clone())
        .header("project_id", webpush.send_id.clone())
        .send_json(&send_data.params)
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
                    let body: params::FcmResponse = serde_json::from_slice(&body).unwrap();
                    println!("==============================================");
                    println!("post_notification_key: response.body(): {:?}", body);
                    println!("==============================================");
                    body.notification_key
                });
            _notification_key
        });
    resp
}

