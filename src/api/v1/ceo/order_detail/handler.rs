use crate::api::v1::ceo::order_detail::model;
use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::models::order::Order;
use crate::models::order_detail::OrderDetail as Object;
use crate::models::DbExecutor;
use crate::schema::order::dsl::{id, order as tb_order};
use crate::schema::order_detail::dsl::{order_detail as tb, order_id, state};
use actix::Handler;

use diesel;
use diesel::prelude::*;

impl Handler<model::New> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: model::New, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;
        let check = tb
            .filter(&order_id.eq(&msg.order_id))
            .filter(&state.eq(&msg.state))
            .load::<Object>(conn)?
            .pop();

        match check {
            Some(_) => {
                println!("  이미 요청하셧습니다. ");
                let payload = serde_json::json!({
                    "msg": "이미 요청하셧습니다."
                });

                Ok(Msg {
                    status: 400,
                    data: payload,
                })
            }
            None => {
                println!("   요청을 저장 합니다. ");
                let insert: Object = diesel::insert_into(tb)
                    .values(&msg)
                    .get_result::<Object>(conn)?;
                let item_order = tb_order
                    .filter(&id.eq(&msg.order_id))
                    .get_result::<Order>(conn)?;

                let payload = serde_json::json!({
                    "item": insert,
                    "order": item_order,
                });
                Ok(Msg {
                    status: 200,
                    data: payload,
                })
            }
        }
    }
}
