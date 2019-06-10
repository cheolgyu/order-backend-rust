use crate::errors::ServiceError;
use crate::models::DbExecutor;
use crate::schema::option_group::dsl::{id, name, option_group as tb};
use crate::svc::option_group::model::{Get, GetList, InpNew, New, OptionGroup as object, Update};
use actix::Handler;
use actix::Message;
use actix_web::{error, Error};
use bcrypt::verify;
use diesel;
use diesel::prelude::*;
use uuid::Uuid;

impl Handler<New> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: New, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;
        let check = tb.filter(&name.eq(&msg.name)).load::<object>(conn)?.pop();

        match check {
            Some(_) => Err(ServiceError::BadRequest("중복".into())),
            None => {
                let insert: object = diesel::insert_into(tb)
                    .values(&msg)
                    .get_result::<object>(conn)?;

                let payload = serde_json::json!({
                    "item": insert,
                });
                Ok(Msg {
                    status: 201,
                    data: payload,
                })
            }
        }
    }
}
use crate::models::msg::Msg;
impl Handler<Update> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: Update, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        let old_item = tb.filter(&id.eq(&msg.id)).get_result::<object>(conn)?;
        let item_update = diesel::update(&old_item)
            .set(&msg)
            .get_result::<object>(conn)?;
        let payload = serde_json::json!({
            "item_update": item_update,
        });
        Ok(Msg {
            status: 201,
            data: payload,
        })
    }
}

impl Handler<Get> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: Get, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        let item = tb.filter(&id.eq(&msg.id)).get_result::<object>(conn)?;

        let payload = serde_json::json!({
            "item": item,
        });
        Ok(Msg {
            status: 201,
            data: payload,
        })
    }
}

impl Handler<GetList> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, _msg: GetList, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        let item = tb.load::<object>(conn)?;

        let payload = serde_json::json!({
            "items": item,
        });
        Ok(Msg {
            status: 201,
            data: payload,
        })
    }
}
