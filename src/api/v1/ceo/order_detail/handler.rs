use crate::api::v1::ceo::order_detail::model;
use crate::errors::ServiceError;
use crate::models::order::Order;
use crate::models::order_detail::OrderDetail as Object;
use crate::models::DbExecutor;
use crate::schema::order::dsl::{id, order as tb_order, state as order_state};
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
        let od = tb_order
            .find(&msg.order_id)
            .load::<Order>(conn)?
            .pop();
        match od {
            Some(od_ok) => {
                match od_ok.state {
                    // 거절된 주문
                    -1 => Err(ServiceError::BadRequest(
                        "이미 거절하신 주문 입니다.".to_string(),
                    )),
                    // 대기중인 주문
                    1 => match msg.state {
                        //주문상세 취소
                        0 => Err(ServiceError::BadRequest(
                            "승인된 주문은 거절할수 없습니다.".to_string(),
                        )),
                        //주문상세 승인
                        1 => {
                            let update_order = diesel::update(tb_order.find(&msg.order_id))
                                .set(order_state.eq(2))
                                .get_result::<Order>(conn)?;
                           
                            let item_order_detail: Object = diesel::insert_into(tb)
                                .values(&msg)
                                .get_result::<Object>(conn)?;
                            
                            Ok(model::NewRes {
                                order: update_order,
                                order_detail: item_order_detail,
                            })
                        }
                        //주문상세 수령
                        2 => Err(ServiceError::BadRequest(
                            "먼저 수락을 해주세요.".to_string(),
                        )),
                        
                        _ => Err(ServiceError::BadRequest("누 구 냐?".to_string())),
                    },
                    // 수락한 주문
                    2 => {
                        match msg.state {
                            0 => Err(ServiceError::BadRequest(
                                "이미 수락한 주문입니다. 거절할수 없습니다.".to_string(),
                            )),
                            1 => Err(ServiceError::BadRequest(
                                "이미 수락한 주문입니다.".to_string(),
                            )),
                            2 => {
                                let update_order = diesel::update(tb_order.find(&msg.order_id))
                                    .set(order_state.eq(3))
                                    .get_result::<Order>(conn)?;
                            
                                let item_order_detail: Object = diesel::insert_into(tb)
                                    .values(&msg)
                                    .get_result::<Object>(conn)?;

                                Ok(model::NewRes {
                                    order: update_order,
                                    order_detail: item_order_detail,
                                })
                            }
                            _ => Err(ServiceError::BadRequest("누구 냐?".to_string())),
                        }
                    }
                    _ => Err(ServiceError::BadRequest("누구냐?".to_string())),
                }
            }
            None => Err(ServiceError::BadRequest(
                    "잘못된 주문번호 입니다.".to_string(),
                )),
        }
    }
}
