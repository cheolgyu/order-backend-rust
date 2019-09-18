use crate::api::v1::ceo::auth::model::{AuthUser, Info};
use crate::fcm::router as fcm;
use crate::api::v1::ceo::order_detail::model;
use crate::errors::ServiceError;
use crate::models::fcm::{Notification, ParamsNotification, ParamsToUser};
use crate::models::{AppStateWithTxt, DbExecutor};
use crate::utils::validator::Validate;
use actix::Addr;
use actix_web::{
    client::Client,
    web::{Data, Json, Path},
    HttpResponse, ResponseError,
};
use futures::{future::result, Future};

pub fn put(
    json: Json<model::InpNew>,
    auth_user: AuthUser,
    path_info: Path<Info>,
    db: Data<Addr<DbExecutor>>,
    client: Data<Client>,
    store: Data<AppStateWithTxt>,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    let mut info = path_info.into_inner();
    info.auth_user = Some(auth_user);
    let info2 = info.clone();
    let order_id = json.clone().order_id.clone();
    let j = json.into_inner();
    let db2 = db.clone();
    let db3 = db.clone();
    let db4 = db.clone();
    let shop_id = info2.shop_id.unwrap();
    let db5 = db.clone();

    result(j.validate())
        .from_err()
        .and_then(move |_| db4.send(info).from_err())
        .and_then(move |_| db2.send(j.new(shop_id)).from_err())
        .and_then(move |res| {
            let ok_res = res.unwrap();
            let state = format!(
                "상태코드: {}",
                ok_res.data["item"]["state"].as_str().unwrap().to_string()
            );
            let to = ok_res.data["order"]["sw_token"]
                .as_str()
                .unwrap()
                .to_string();
            //let order_id = ok_res.data["order"]["od"].as_str().unwrap();
            let send_data = ParamsToUser {
                url: store.webpush.send.clone(),
                order_id: order_id,
                webpush: store.webpush.clone(),
                params: ParamsNotification {
                    notification: Notification {
                        title: "[손님]주문에 대한 응답.".to_string(),
                        body: state,
                        icon: "".to_string(),
                        click_action: "".to_string(),
                    },
                    to: to,
                },
            };

            fcm::to_user(send_data, client, db5)
        })
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}
