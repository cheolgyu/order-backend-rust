

use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::models::DbExecutor;
use crate::models::fcm::{Fcm as Object, *};
use crate::schema::fcm::dsl::{ fcm as tb, order_id,kind};

use actix::Handler;
use diesel;
use diesel::prelude::*;

impl Handler<New> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: New, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        let check = tb
            .filter(&order_id.eq(&msg.order_id))
            .filter(&kind.eq(&msg.kind))
            .load::<Object>(conn)?
            .pop();

        match check {
            Some(_) => Err(ServiceError::BadRequest("중복".into())),
            None => {
                let insert: Object = diesel::insert_into(tb)
                    .values(&msg)
                    .get_result::<Object>(conn)?;

                let payload = serde_json::json!({
                    "item": insert,
                });
                Ok(Msg {
                    status: 200,
                    data: payload,
                })
            }
        }
    }
}