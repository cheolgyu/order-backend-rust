use crate::errors::ServiceError;
use crate::models::DbExecutor;
use crate::svc::product::model::{Get, GetList, InpNew, New, Product, Update};
use crate::utils::hash_password;
use actix::Handler;
use actix::Message;
use actix_web::{error, Error};
use bcrypt::verify;
use diesel;
use diesel::prelude::*;
use uuid::Uuid;
impl Handler<New> for DbExecutor {
    type Result = Result<Product, ServiceError>;

    fn handle(&mut self, msg: New, _: &mut Self::Context) -> Self::Result {
        use crate::schema::product::dsl::{name, product as tb};
        let conn = &self.0.get()?;

        let check = tb.filter(&name.eq(&msg.name)).load::<Product>(conn)?.pop();

        match check {
            Some(_) => Err(ServiceError::BadRequest("중복".into())),
            None => {
                println!("{:?}", &msg.option_group);
                let insert: Product = diesel::insert_into(tb)
                    .values(&msg)
                    .get_result::<Product>(conn)?;

                Ok(insert)
            }
        }
    }
}
use crate::models::msg::Msg;
impl Handler<Update> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: Update, _: &mut Self::Context) -> Self::Result {
        use crate::schema::product::dsl::{id, name, product as tb};
        let conn = &self.0.get()?;

        let old_item = tb.filter(&id.eq(&msg.id)).get_result::<Product>(conn)?;
        let item_update = diesel::update(&old_item)
            .set(&msg)
            .get_result::<Product>(conn)?;
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
        use crate::schema::product::dsl::{id, name, product as tb};
        let conn = &self.0.get()?;

        let item = tb.filter(&id.eq(&msg.id)).get_result::<Product>(conn)?;

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

    fn handle(&mut self, msg: GetList, _: &mut Self::Context) -> Self::Result {
        use crate::schema::product::dsl::{name, product as tb, shop_id};
        let conn = &self.0.get()?;

        let item = tb
            .filter(&shop_id.eq(&msg.shop_id))
            .get_result::<Product>(conn)?;

        let payload = serde_json::json!({
            "items": item,
        });
        Ok(Msg {
            status: 201,
            data: payload,
        })
    }
}
