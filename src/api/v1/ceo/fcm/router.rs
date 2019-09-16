use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::models::fcm::{ParamsToFcm,ToUserResp,ParamsToUser,ToFcmResp};
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
        .map_err(|e| {
            ServiceError::BadRequest("to_fcm err ".into())
        })
        .and_then(|response| {
            response
                .from_err()
                .fold(BytesMut::new(), |mut acc, chunk| {
                    acc.extend_from_slice(&chunk);
                    Ok::<_, ServiceError>(acc)
                })
                .map(|body| {
                    let body: ToFcmResp = serde_json::from_slice(&body).expect("to_fcm body 변환 오류");
                    
                    body
                })
        });
    resp.and_then(move |res| _db.send(res.new(send_data.order_id.clone())).from_err() )
}

pub fn to_user(
    send_data: ParamsToUser,
    client: Data<Client>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = Result<Msg, ServiceError>, Error = ServiceError> {
    let _db = db.clone();

    println!("batch:4444444444444:{:?}",send_data);
    
    let resp = Client::new()
        .post(send_data.url.clone())
        .header(CONTENT_TYPE, "application/json")
        .header("Authorization", send_data.webpush.key.clone())
        .send_json(&send_data.params)
        .map_err(|e| {
            println!("batch:666666666666");
            eprintln!("{:?}",e);
            panic!("{:?}", e)
        })
        .and_then(|response| {
             println!("batch:5555555555555");
            let res = response
                .from_err()
                .fold(BytesMut::new(), |mut acc, chunk| {
                    acc.extend_from_slice(&chunk);
                    println!("batch:99");
                    Ok::<_, ServiceError>(acc)
                })
                .map(|body| {
                    println!("batch:10");
                    let body: ToUserResp = serde_json::from_slice(&body).expect("to_user body 변환 오류");
                    body
                });
            res 
        });
    println!("batch:777777777");
    resp.and_then(move |res| {
        println!("batch:888888888");
        _db.send(res.new(send_data.order_id.clone())).from_err()
    } )
}
