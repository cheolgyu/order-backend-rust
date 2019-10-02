use crate::api::v1::ceo::order_detail::model;
use crate::errors::ServiceError;
use crate::models::order::Order;
use crate::models::order_detail::OrderDetail as Object;
use crate::models::DbExecutor;
use crate::schema::order::dsl::{id, order as tb_order};
use crate::schema::order_detail::dsl::{order_detail as tb, order_id, state};
use actix::Handler;

use diesel;
use diesel::prelude::*;

impl Handler<model::New> for DbExecutor {
    type Result = Result<model::NewRes, ServiceError>;

    fn handle(&mut self, msg: model::New, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;
        let check = tb
            .filter(&order_id.eq(&msg.order_id))
            .order(state.desc())
            .load::<Object>(conn)?
            .pop();
        match check {
            Some(check_ok) => {
                match check_ok.state {
                    0 => Err(ServiceError::BadRequest(
                        "거절하신 주문 입니다.".to_string(),
                    )),
                    1 => match msg.state {
                        2 => {
                            let item_order_detail: Object = diesel::insert_into(tb)
                                .values(&msg)
                                .get_result::<Object>(conn)?;
                            let item_order = tb_order
                                .filter(&id.eq(&msg.order_id))
                                .get_result::<Order>(conn)?;
                            Ok(model::NewRes {
                                order: item_order,
                                order_detail: item_order_detail,
                            })
                        }
                        1 => Err(ServiceError::BadRequest(
                            "이미 승인된 주문입니다.".to_string(),
                        )),
                        0 => Err(ServiceError::BadRequest(
                            "승인된 주문은 거절할수 없습니다.".to_string(),
                        )),
                        _ => Err(ServiceError::BadRequest("누 구 냐?".to_string())),
                    },
                    2 => {
                        match msg.state {
                            2 => {
                                //추가 수령하세요 요청.
                                let item_order_detail: Object = diesel::insert_into(tb)
                                    .values(&msg)
                                    .get_result::<Object>(conn)?;
                                let item_order = tb_order
                                    .filter(&id.eq(&msg.order_id))
                                    .get_result::<Order>(conn)?;
                                Ok(model::NewRes {
                                    order: item_order,
                                    order_detail: item_order_detail,
                                })
                            }
                            1 => Err(ServiceError::BadRequest(
                                "주문을 승인할수 없습니다.".to_string(),
                            )),
                            0 => Err(ServiceError::BadRequest(
                                "주문을 거절할수 없습니다.".to_string(),
                            )),
                            _ => Err(ServiceError::BadRequest("누구 냐?".to_string())),
                        }
                    }
                    _ => Err(ServiceError::BadRequest("누구냐?".to_string())),
                }
            }
            None => match msg.state {
                2 => Err(ServiceError::BadRequest(
                    "승인이나 거절부터하십시요.".to_string(),
                )),
                _ => {
                    let item_order_detail: Object = diesel::insert_into(tb)
                        .values(&msg)
                        .get_result::<Object>(conn)?;
                    let item_order = tb_order
                        .filter(&id.eq(&msg.order_id))
                        .get_result::<Order>(conn)?;
                    Ok(model::NewRes {
                        order: item_order,
                        order_detail: item_order_detail,
                    })
                }
            },
        }
    }
}
