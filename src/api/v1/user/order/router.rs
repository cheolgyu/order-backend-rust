use crate::api::v1::user::order::model;
use crate::errors::ServiceError;
use crate::models::{AppStateWithTxt, DbExecutor};
use crate::utils::validator::Validate;
use actix::Addr;
use actix_web::{
    client::Client,
    web::{Data, Json},
    Error, HttpResponse, ResponseError
};
use futures::{future::result, Future};

use crate::fcm::router::to_user;
use crate::fcm::model::*;


pub fn put(
    json: Json<model::InpNew>,
    db: Data<Addr<DbExecutor>>,
    client: Data<Client>,
    store: Data<AppStateWithTxt>,
    
) -> impl Future<Item = HttpResponse, Error = Error> {
    let j1 = json.clone();
    let j2 = json.clone();
    let db2 = db.clone();
    let db3 = db.clone();
    let client2 = client.clone();
    let store2 = store.clone();

    let key = store.webpush.key.clone();
    let webpush_url_send = store.webpush.send.clone();
    let websocket_url = store.websocket.send.clone();

    result(json.validate())
        .and_then(move |_| db2.send(json.into_inner().new()).from_err())
        .and_then(
            move |res| {
                let url = format!("{}{}/test", websocket_url, j1.shop_id);
                client
                    .get(url) // <- Create request builder
                    .header("User-Agent", "Actix-web")
                    .send() // <- Send http request
                    .map_err(|e| ServiceError::BadRequest("ws push error".into()))
                    .and_then(|response| {
                        // <- server http response
                        res
                    })
                    .from_err()
            }, //web push
        )
        .and_then(move |res| {
            let send_data = ReqToUser {
                order_id: res.order.id.clone(),
                params: ReqToUserData {
                    notification: Notification {
                        title: "주문!".to_string(),
                        body: "22".to_string(),
                        icon: "33".to_string(),
                        click_action: "44".to_string(),
                    },
                    to: res.shop.notification_key.clone(),
                },
            };

            to_user(send_data,db,store2).from_err()
        })
        //.and_then(|res| Ok(HttpResponse::Ok()))
        .and_then( |res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}
