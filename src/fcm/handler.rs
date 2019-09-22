use crate::errors::ServiceError;
use crate::models::fcm::{Fcm as Object, *};
use crate::models::msg::Msg;
use crate::models::DbExecutor;
use crate::schema::fcm::dsl::{fcm as tb, *};

use actix::Handler;
use diesel;
use diesel::prelude::*;
// fcm 내역 저장
impl Handler<New> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: New, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

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
