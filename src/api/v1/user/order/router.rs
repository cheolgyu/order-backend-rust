use crate::api::v1::user::order::model;
use crate::errors::ServiceError;
use crate::fcm::model::*;
use crate::fcm::router::to_user;
use crate::models::{AppStateWithTxt, DbExecutor};
use crate::utils::client::SSLClinet;
use crate::utils::validator::Validate;
use actix::Addr;
use actix_web::{
    web::{Data, Json},
    Error, HttpResponse, ResponseError,
};
use futures::{future::result, Future};

pub fn put(
    json: Json<model::InpNew>,
    db: Data<Addr<DbExecutor>>,
    store: Data<AppStateWithTxt>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let j1 = json.clone();
    let db2 = db.clone();
    let store2 = store.clone();

    let websocket_url = store.websocket.send.clone();

    result(json.validate())
        .and_then(move |_| db2.send(json.into_inner().new()).from_err())
        .and_then(
            move |res| {
                let url = format!("{}{}/test", websocket_url, j1.shop_id);
                SSLClinet::build()
                    .get(url) // <- Create request builder
                    .header("User-Agent", "Actix-web")
                    .send() // <- Send http request
                    .map_err(|e| ServiceError::BadRequest(e.to_string()))
                    .and_then(|_response| {
                        // <- server http response
                        res
                    })
                    .from_err()
            }, //web push
        )
        .and_then(move |res| {
            let to = res.shop.notification_key.clone();
            let title = format!("[{}] 주문!", res.shop.name);
            let body = format!("주문도착.!");

            let send_data = ReqToUser {
                comm: ReqToComm::new_order(res.order.id),
                params: ReqToUserData::new(to, title, body),
            };

            to_user(send_data, db, store2).from_err()
        })
        //.and_then(|res| Ok(HttpResponse::Ok()))
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}
