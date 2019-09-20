use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::models::fcm::New;
use crate::fcm::FcmExecutor;
use crate::fcm::model::*;
use crate::schema::fcm::dsl::{fcm as tb, *};

use actix_web::{
    client::Client,
    http::header::CONTENT_TYPE,
    web::{BytesMut, Data},
    ResponseError,
};

use actix::Handler;
use futures::Future;
use futures::stream::Stream;

impl Handler<ReqToFcm> for FcmExecutor {
    type Result = Box<dyn Future<Item =Result<Msg, ServiceError>, Error = ServiceError>>;

    fn handle(&mut self, msg: ReqToFcm, ctx: &mut Self::Context) -> Self::Result {
        Box::new(
            Client::new().post(self.store.webpush.reg.clone())
            .header(CONTENT_TYPE, "application/json")
            .header("Authorization", self.store.webpush.key.clone())
            .header("project_id", self.store.webpush.send_id.clone())
            .send_json(&msg.params)
            .map_err(|e| ServiceError::BadRequest("to_fcm err ".into()))
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

                        let res = body;
                        self.db.send(New{
                            order_id: msg.order_id.clone(),
                            resp: serde_json::to_value(&res).unwrap(),
                        })
                    }) 
            })
        )
       
    }
}

impl Handler<ReqToUser> for FcmExecutor {
    type Result  = Box<dyn Future<Item =Result<Msg, ServiceError>, Error = ServiceError>>;

    fn handle(&mut self, msg: ReqToUser, ctx: &mut Self::Context) -> Self::Result {
        Box::new(
            Client::new()
            .post(self.store.webpush.send.clone())
            .header(CONTENT_TYPE, "application/json")
            .header("Authorization", self.store.webpush.key.clone())
            .send_json(&msg.params)
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
                        let res = body;
                        self.db.send(New{
                            order_id: msg.order_id.clone(),
                            resp: serde_json::to_value(&res).unwrap(),
                        })
                    })
            })
        )
       
        
    }
}
