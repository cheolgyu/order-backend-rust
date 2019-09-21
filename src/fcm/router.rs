use crate::errors::ServiceError;
use crate::models::fcm::New;
use crate::fcm::model::*;
use crate::models::msg::Msg;
use crate::models::{DbExecutor, AppStateWithTxt};
use actix::Addr;
use actix_web::{
    client::Client,
    http::header::CONTENT_TYPE,
    web::{BytesMut, Data},
    
};
use futures::stream::Stream;
use futures::Future;

pub fn to_fcm(
    send_data: ReqToFcm,
    db: Data<Addr<DbExecutor>>,
    store: Data<AppStateWithTxt>,
) -> impl Future<Item = Result<Msg, ServiceError>, Error = ServiceError> {
    
    let resp = Client::new().post(store.webpush.reg.clone())
            .header(CONTENT_TYPE, "application/json")
            .header("Authorization", store.webpush.key.clone())
            .header("project_id", store.webpush.send_id.clone())
            .send_json(&send_data.params)
            .map_err(|e| ServiceError::BadRequest(e.to_string()))
            .and_then(|response| {
                response
                    .from_err()
                    .fold(BytesMut::new(), |mut acc, chunk| {
                        acc.extend_from_slice(&chunk);
                        Ok::<_, ServiceError>(acc)
                    })
                    .map(|body| {
                        let body: RespFcm =
                            serde_json::from_slice(&body).expect("to_fcm body 변환 오류");
                        (body,db)
                    }) 
            });
    resp.and_then( |(res,db)| {
        let p = New::new_fcm(serde_json::to_value(&res).unwrap());
        db.send(p).from_err()
    })
}

pub fn to_user(
    send_data: ReqToUser,
    db: Data<Addr<DbExecutor>>,
    store: Data<AppStateWithTxt>,
) -> impl Future<Item = Result<Msg, ServiceError>, Error = ServiceError> {
    let order_id = send_data.order_id.clone();
    
    let resp = Client::new()
            .post(store.webpush.send.clone())
            .header(CONTENT_TYPE, "application/json")
            .header("Authorization", store.webpush.key.clone())
            .send_json(&send_data.params)
            .map_err(|e| ServiceError::BadRequest("to_user ".into()))
            .and_then(|response| {
                
                response
                    .from_err()
                    .fold(BytesMut::new(), |mut acc, chunk| {
                        acc.extend_from_slice(&chunk);
                        Ok::<_, ServiceError>(acc)
                    })
                    .map(|body| {
                        let body: RespUser =
                            serde_json::from_slice(&body).expect("to_user body 변환 오류");
                        (body,db)
                        
                    })
    });
    resp.and_then( move  |(res,db)| {
        let p = New::new_user(order_id ,serde_json::to_value(&res).unwrap());
        db.send(p).from_err()
    })
}
