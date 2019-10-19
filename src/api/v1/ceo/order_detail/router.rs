use crate::api::v1::ceo::auth::model::ReqInfo;
use crate::api::v1::ceo::order_detail::model;
use crate::errors::ServiceError;
use crate::models::{AppStateWithTxt, DbExecutor};
use crate::utils::validator::Validate;
use actix::Addr;
use actix_web::{
    web::{Data, Json},
    HttpResponse, ResponseError,
};
use futures::{
    future::{result, Either},
    Future,
};

use crate::fcm::model::*;
use crate::fcm::router::to_user;

fn od_state_to_string(od_state: String) -> String {
    let res = if od_state == "0".to_string() {
        "거절".to_string()
    } else if od_state == "1".to_string() {
        "승인".to_string()
    } else if od_state == "2".to_string() {
        "수령".to_string()
    }else{
        "??".to_string()
    };
    res
}

pub fn put(
    json: Json<model::InpNew>,
    req_info: ReqInfo,
    db: Data<Addr<DbExecutor>>,
    store: Data<AppStateWithTxt>,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    let order_id = json.clone().order_id.clone();
    let j = json.into_inner();
    let db2 = db.clone();

    result(j.validate())
        .from_err()
        .and_then(move |_| db.send(j.new(req_info.req_s_id())).from_err())
        .and_then(move |res_opt| match res_opt {
            Ok(res) => {
                let shop_name = res.shop.name.to_string();
                let inp_state = od_state_to_string(res.order_detail.state.to_string());
                let new_order_detail_id = res.order_detail.id;
                let state_str = format!("[{}] 주문 {}!", shop_name, inp_state).to_string();
                //let title = "[손님]주문에 대한 응답.".to_string();
                let title = state_str.clone();
                let body = state_str;
                let to = res.order.sw_token;

                let send_data = ReqToUser {
                    comm: ReqToComm::new_order_detail(
                        order_id,
                        new_order_detail_id,
                        res.order_detail.state,
                    ),
                    params: ReqToUserData::new(to, title, body),
                };
                Either::A(to_user(send_data, db2, store))
            }
            Err(e) => Either::B(futures::future::ok(Err(e))),
        })
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}
