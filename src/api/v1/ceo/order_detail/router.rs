use crate::api::v1::ceo::auth::model::{AuthUser, Info};
use crate::api::v1::ceo::order_detail::model;
use crate::errors::ServiceError;
use crate::models::{AppStateWithTxt, DbExecutor};
use crate::utils::validator::Validate;
use actix::Addr;
use actix_web::{
    client::Client,
    web::{Data, Json, Path},
    HttpResponse, ResponseError,
};
use futures::{
    future::{err, result, Either},
    Future,
};

use crate::fcm::model::*;
use crate::fcm::router::to_user;
use crate::models::msg::Msg;
use futures::future::FutureResult;
#[derive(Debug, PartialEq)]
pub enum ExampleFutureError {
    Oops,
}
type ExampleFuture = FutureResult<Result<Msg, ServiceError>, ServiceError>;

pub fn new_example_future_err(msg: String) -> ExampleFuture {
    futures::future::err(ServiceError::BadRequest(msg.into()))
}

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
        .and_then(move |res_opt| match res_opt {
            Ok(res) => {
                let inp_state = res.order_detail.state;
                let new_order_detail_id = res.order_detail.id;
                let state = format!("상태코드: {}", inp_state);
                let to = res.order.sw_token;

                let send_data = ReqToUser {
                    comm: ReqToComm::new_order_detail(order_id, new_order_detail_id, inp_state),
                    params: ReqToUserData {
                        notification: Notification {
                            title: "[손님]주문에 대한 응답.".to_string(),
                            body: state,
                            icon: "".to_string(),
                            click_action: "".to_string(),
                        },
                        to: to,
                    },
                };
                Either::A(to_user(send_data, db, store))
            }
            Err(e) => Either::B(new_example_future_err(
                "서버오류, 주문응답 요청".to_string(),
            )),
        })
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}
