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
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    let user_token = json.sw_token.clone();
    let j1 = json.clone();
    let db2 = db.clone();
    let store2 = store.clone();

    let websocket_url = store.websocket.send.clone();

    let validation = futures::future::result(json.validate()).map_err(|e| {
                            println!("[err] send ws =>{:?}",e );
                            ServiceError::BadRequest(e.to_string())
                        });

    let db_insert = db2.send(json.into_inner().new()).from_err();

    validation.and_then(|_| {
        db_insert.and_then(move |res_opt|{
            let res = res_opt.unwrap();
            let url = format!("{}{}/test", websocket_url, j1.shop_id);
            let send_ws = SSLClinet::build()
                    .get(url) // <- Create request builder
                    .send() // <- Send http request
                    .map_err(|e| {
                        println!("[err] send ws =>{:?}",e );
                        ServiceError::BadRequest(e.to_string())
                    });
                    
            let to = res.shop.notification_key.clone();
            let title = format!("[{}] 주문!", res.shop.name);
            let body = format!("주문도착.!");

            let send_data = ReqToUser {
                comm: ReqToComm::new_order(res.order.id),
                params: ReqToUserData::new(to, title, body),
            };

            let send_shop = to_user(send_data, db.clone(), store2.clone()).from_err();

            let o_id = res.order.id.clone();

            let user_to = user_token.clone();
            let user_title = format!("주문을 하셨습니다.");
            let user_body = format!("주문을 하셨습니다.");
            let u_d=
                    ReqToUser {
                    comm: ReqToComm::new_order(o_id.clone()),
                    params: ReqToUserData::new(user_to.clone(), user_title.clone(), user_body.clone()),
                };
            let send_user =to_user(u_d, db.clone(), store2.clone());

            send_ws.and_then( |_|{ 
                send_shop.and_then(|_|{
                    send_user.and_then(|res| match res {
                        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                        Err(e) => Ok(e.error_response()),
                    })
                })
            })
        })
    })

}

