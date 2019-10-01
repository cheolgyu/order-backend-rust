use crate::errors::ServiceError;
use crate::fcm::model::*;
use crate::models::fcm::New;
use crate::models::msg::Msg;
use crate::models::{AppStateWithTxt, DbExecutor};
use crate::utils::client::SSLClinet;
use actix::Addr;

use actix_web::{
    client::Client,
    http::header::CONTENT_TYPE,
    web::{BytesMut, Data},
};
use futures::stream::Stream;
use futures::Future;

pub fn to_fcm(
    mut send_data: ReqToFcm,
    db: Data<Addr<DbExecutor>>,
    store: Data<AppStateWithTxt>,
) -> impl Future<Item = Result<Msg, ServiceError>, Error = ServiceError> {
    let resp = SSLClinet::build()
        .post(store.webpush.reg.clone())
        .header(CONTENT_TYPE, "application/json")
        .header("Authorization", store.webpush.key.clone())
        .header("project_id", store.webpush.send_id.clone())
        .send_json(&send_data.params)
        .map_err(|e| {
            let msg = format!("to_fcm=>: {}", e.to_string());

            ServiceError::BadRequest(msg)
        })
        .and_then(|response| {
            let status = response.status().as_u16();

            response
                .from_err()
                .fold(BytesMut::new(), |mut acc, chunk| {
                    acc.extend_from_slice(&chunk);
                    Ok::<_, ServiceError>(acc)
                })
                .map(move |body| {
                    let body: RespFcm =
                        serde_json::from_slice(&body).expect("to_fcm body 변환 오류");
                    let resp_json = serde_json::json!({
                        "status": status,
                        "body": body
                    });
                    (db, resp_json)
                })
        });
    resp.and_then(|(db, resp_json)| {
        send_data.comm.req = serde_json::to_value(send_data.params).unwrap();
        send_data.comm.resp = resp_json;
        let p = send_data.comm.get_new();
        db.send(p).from_err()
    })
}

pub fn to_user(
    mut send_data: ReqToUser,
    db: Data<Addr<DbExecutor>>,
    store: Data<AppStateWithTxt>,
) -> impl Future<Item = Result<Msg, ServiceError>, Error = ServiceError> {
    let order_id = send_data.comm.order_id.clone();
    println!(
        "to_user=>:  url={:?}, key={:?}, params={:?} ",
        store.webpush.send.clone(),
        store.webpush.key.clone(),
        &send_data.params
    );

    let req = SSLClinet::build()
        .post(store.webpush.send.clone())
        .header(CONTENT_TYPE, "application/json")
        .header("Authorization", store.webpush.key.clone())
        .send_json(&send_data.params)
        .map_err(|e| {
            let msg = format!("to_user=>: err: {}, ", e.to_string());

            ServiceError::BadRequest(msg)
        })
        .and_then(|response| {
            let status = response.status().as_u16();

            response
                .from_err()
                .fold(BytesMut::new(), |mut acc, chunk| {
                    acc.extend_from_slice(&chunk);
                    Ok::<_, ServiceError>(acc)
                })
                .map(move |body| {
                    let body: RespUser =
                        serde_json::from_slice(&body).expect("to_user body 변환 오류");

                    let resp_json = serde_json::json!({
                        "status": status,
                        "body": body
                    });
                    (db, resp_json)
                })
        });

    req.and_then(move |(db, resp_json)| {
        send_data.comm.req = serde_json::to_value(send_data.params).unwrap();
        send_data.comm.resp = resp_json;
        let p = send_data.comm.get_new();
        db.send(p).from_err()
    })
}
