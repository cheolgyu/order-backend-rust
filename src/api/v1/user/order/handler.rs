use crate::api::v1::user::order::model;
use crate::errors::ServiceError;
use crate::models::order::{New, Order as Object};
use crate::models::DbExecutor;
use crate::schema::order::dsl::{deleted_at, id, order as tb, shop_id};
use actix::Handler;

use crate::models::msg::Msg;
use diesel;
use diesel::prelude::*;

impl Handler<New> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: New, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        let insert: Object = diesel::insert_into(tb)
            .values(&msg)
            .get_result::<Object>(conn)?;

        let payload = serde_json::json!({
            "item": insert,
            "shop_id":msg.shop_id.to_string()
        });
        Ok(Msg {
            status: 200,
            data: payload,
        })
    }
}
