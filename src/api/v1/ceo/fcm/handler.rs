use crate::models::shop::Shop;
use crate::schema::shop::dsl::{ceo_id, shop};

use crate::errors::ServiceError;
use crate::models::device::{Device as Object, *};
use crate::models::msg::Msg;
use crate::models::DbExecutor;
use crate::schema::user_device::dsl::{id, name, sw_token, user_device as tb, user_id};

use actix::Handler;
use diesel;
use diesel::prelude::*;
use serde_json::json;

impl Handler<New> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: New, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        let check = tb
            .filter(&name.eq(&msg.name))
            .filter(&sw_token.eq(&msg.sw_token))
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

impl Handler<Get> for DbExecutor {
    type Result = Result<GetWithKey, ServiceError>;

    fn handle(&mut self, msg: Get, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        //let item = tb.filter(&sw_token.eq(&msg.sw_token)).load::<Object>(conn)?.pop()
        let count = tb
            .filter(&sw_token.eq(&msg.sw_token))
            .count()
            .get_result::<i64>(conn)?;
        let shop_data = shop
            .filter(&ceo_id.eq(&msg.user_id))
            .load::<Shop>(conn)?
            .pop();
        let s_data = match shop_data {
            Some(_shop) => (_shop.id.to_string(), _shop.notification_key),
            None => ("".to_string(), "".to_string()),
        };
        let data = GetWithKey {
            shop_id: s_data.0.clone(),
            notification_key: s_data.1.clone(),
            device_cnt: count.clone(),
            params: msg.clone(),
        };

        Ok(data)
    }
}

impl Handler<GetList> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: GetList, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;
        let items = tb.filter(&user_id.eq(&msg.user_id)).load::<Object>(conn)?;
        let payload = json!({
            "items": items,
        });

        Ok(Msg {
            status: 200,
            data: payload,
        })
    }
}

impl Handler<Update> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: Update, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        let check = tb.filter(&id.eq(&msg.id)).load::<Object>(conn)?.pop();

        match check {
            Some(_) => Err(ServiceError::BadRequest(
                "이미 있는 이름입니다.".into(),
            )),
            None => {
                let old_item = tb.filter(&id.eq(&msg.id)).get_result::<Object>(conn)?;
                let item_update = diesel::update(&old_item)
                    .set(&msg)
                    .get_result::<Object>(conn)?;

                let payload = serde_json::json!({
                    "item": item_update,
                });
                Ok(Msg {
                    status: 200,
                    data: payload,
                })
            }
        }
    }
}
