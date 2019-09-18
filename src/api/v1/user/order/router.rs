use crate::fcm::router as fcm;
use crate::api::v1::user::order::model;
use crate::errors::ServiceError;
use crate::models::fcm::{Notification, ParamsNotification, ParamsToUser};
use crate::models::{AppStateWithTxt, DbExecutor};
use crate::utils::validator::Validate;
use actix::Addr;
use actix_web::{
    client::Client,
    http::header,
    web::{Data, Json},
    Error, HttpResponse, ResponseError,
};
use futures::{future::result, Future};

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

    let key = store.webpush.key.clone();
    let webpush_url_send = store.webpush.send.clone();
    let websocket_url = store.websocket.send.clone();

    result(json.validate())
        //주문 저장
        .and_then(move |_| db2.send(json.into_inner().new()).from_err())
        // 사장님에게 알림 서비스 실행.
        //web socket
        .and_then(
            move |res| {
                /*
                let _r = res.unwrap();
                let _shop_id = _r.data["shop_id"].clone();
                let o_r = Ok(_r);
                */
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
            let send_data = ParamsToUser {
                url: store.webpush.send.clone(),
                order_id: res.order.id.clone(),
                webpush: store.webpush.clone(),
                params: ParamsNotification {
                    notification: Notification {
                        title: "주문!".to_string(),
                        body: "22".to_string(),
                        icon: "33".to_string(),
                        click_action: "44".to_string(),
                    },
                    to: res.shop.notification_key.clone(),
                },
            };

            fcm::to_user(send_data, client2, db3).from_err()
        })
        //.and_then(|res| Ok(HttpResponse::Ok().json(res)))
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}
