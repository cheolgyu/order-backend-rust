use crate::api::v1::user::order::model;

use crate::models::DbExecutor;

use crate::utils::validator::Validate;
use actix::Addr;
use actix_web::{
    client::Client,
    web::{Data, Json, Path},
    Error, HttpResponse, ResponseError,
};
use futures::{future::result, Future};
use serde_json;
use serde_json::json;
use uuid::Uuid;
use crate::errors::ServiceError;

pub fn put(
    json: Json<model::InpNew>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    result(json.validate())
        //주문 저장
        .and_then(move |_| db.send(json.into_inner().new()).from_err())
        // 사장님에게 알림 서비스 실행.
       // .from_err()
        .and_then( |res| 
                Client::new()
                    .get("http://127.0.0.1:3001/push/109b7b41-f8eb-4702-abdb-6bfb95f57072/msgtest") // <- Create request builder
                    .header("User-Agent", "Actix-web")
                    .send() // <- Send http request
                    .map_err(|e| ServiceError::BadRequest("중복".into()) )
                    .and_then(|response| {
                        // <- server http response
                        println!("Response: {:?}", response);
                        res
                    }).from_err()
                    
        ) .from_err()
        .and_then(|res| 
        /*
        match res {
            Ok(msg) => {
               
                Ok(HttpResponse::Ok().json(msg))
            }
            Err(e) => Ok(e.error_response()),
        }
        */
        Ok(HttpResponse::Ok().json(res))
        )
}
