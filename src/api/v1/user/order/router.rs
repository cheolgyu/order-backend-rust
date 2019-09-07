use crate::api::v1::user::order::model;

use crate::models::{AppStateWithTxt, DbExecutor};

use crate::errors::ServiceError;
use crate::utils::validator::Validate;
use actix::Addr;
use actix_web::{
    client::Client,
    http::header,
    web::{Data, Json},
    Error, HttpResponse, ResponseError,
};
use futures::{future::result, Future};

#[derive(Debug, Serialize, Deserialize)]
struct SendData {
    notification: Notification,
    to: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Notification {
    title: String,
    body: String,
    icon: String,
    click_action: String,
}

pub fn put(
    json: Json<model::InpNew>,
    db: Data<Addr<DbExecutor>>,
    client: Data<Client>,
    store: Data<AppStateWithTxt>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let j1 = json.clone();
    let j2 = json.clone();

    let key = store.webpush.key.clone();
    let webpush_url_send = store.webpush.send.clone();
    print!("{:?}", webpush_url_send);
    print!("{:?}", key);
    let websocket_url = store.websocket.send.clone();
    result(json.validate())
        //주문 저장
        .and_then(move |_| db.send(json.into_inner().new()).from_err())
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
                println!("{:?}", url);
                client
                    .get(url) // <- Create request builder
                    .header("User-Agent", "Actix-web")
                    .send() // <- Send http request
                    .map_err(|e| {
                        println!("ws push error : {:?}", e.error_response());
                        ServiceError::BadRequest("ws push error".into())
                    })
                    .and_then(|response| {
                        // <- server http response
                        println!("ws push Response: {:?}", response);
                        res
                    })
                    .from_err()
            }, //web push
        )
        .and_then(move |res| {
            let notification = Notification {
                title: "bbb".to_string(),
                body: "bbb".to_string(),
                icon: "bbb".to_string(),
                click_action: "bbb".to_string(),
            };
            let send_data = SendData {
                notification: notification,
                //to:"cksPMoBdGEs:APA91bG9tzqfByJDuxoeD7F-c2w8ENhZvtl6fxHaujVuXeFeD1cJYoAsYyz0rLB-4G3bBZMC4TwoSr1W_EGKdwIpFanOppFXDc22O72yLfH_KIZ2Wm50NXFpft0EfcGQ8oBP_3PYkruw".to_string()
                to: j2.sw_token.clone(),
            };

            println!("ws push sendData: {:?}", send_data);

            Client::default()
                .post(webpush_url_send) // <- Create request builder
                .header("Authorization", key)
                .header(header::CONTENT_TYPE, "application/json")
                .send_json(&send_data) // <- Send http request
                .map_err(|e| {
                    println!("sw push error : {:?}", e.error_response());
                    ServiceError::BadRequest("sw push error".into())
                })
                .and_then(|response| {
                    // <- server http response
                    println!("sw push Response: {:?}", response);

                    Ok(res)
                })
                .from_err()
        })
        .from_err()
        .and_then(|res| Ok(HttpResponse::Ok().json(res)))
}
