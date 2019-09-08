use crate::api::v1::ceo::auth::model::{AuthUser, Info};
use crate::api::v1::ceo::order_detail::model;
use crate::api::v1::ceo::fcm::router as fcm;
use crate::models::{DbExecutor,AppStateWithTxt};
use crate::models::fcm::{ParamsToUser,ParamsNotification, Notification};
use crate::errors::ServiceError;
use crate::utils::validator::Validate;
use actix::Addr;
use actix_web::{
    web::{Data, Json, Path},
    client::Client,
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
    let j = json.into_inner();
    let db2 = db.clone();
    let db3 = db.clone();
    let shop_id = info2.shop_id.unwrap();
    let send_data = ParamsToUser{
        url: store.webpush.send.clone(),
        webpush: store.webpush.clone(),
        params: ParamsNotification{
            notification: Notification{
                title: "".to_string(),
                body: "".to_string(),
                icon: "".to_string(),
                click_action: "".to_string(),
            },
            to: "".to_string(),
        }
    };

    result(j.validate())
        .from_err()
        .and_then(move |_| db.send(info).from_err())
        .and_then(move |_| db2.send(j.new(shop_id)).from_err())
        .and_then(move |_| fcm::to_user(send_data,  client, db3))
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}
