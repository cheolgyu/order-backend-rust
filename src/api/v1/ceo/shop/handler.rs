use crate::models::product::Product ;
use crate::api::v1::ceo::shop::model::{NewShop, ShopID, UpdateShop};
use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::models::shop::{Shop as Object, UpdateNotificationKey};
use crate::models::DbExecutor;
use crate::schema::shop::dsl::{ceo_id, id, name, shop as tb};

use actix::Handler;

use diesel;
use diesel::prelude::*;

use serde_json::json;

impl Handler<NewShop> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: NewShop, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        let check_user = tb
            .filter(&ceo_id.eq(&msg.ceo_id))
            .load::<Object>(conn)?
            .pop();

        match check_user {
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
impl Handler<ShopID> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: ShopID, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;
        let shop = tb.filter(&id.eq(&msg.id)).load::<Object>(conn)?.pop();

        match shop {
            Some(_shop) => {
                let shop_product = Product::belonging_to(&_shop)
                    .get_results::<Product>(conn)
                    .expect("Couldn't find associated posts");

                let payload = json!({
                    "shop": _shop,
                    "product": shop_product,
                });

                Ok(Msg {
                    status: 200,
                    data: payload,
                })
            }
            None => Err(ServiceError::BadRequest("없다".into())),
        }
    }
}

impl Handler<UpdateShop> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: UpdateShop, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        let check_user = tb.filter(&name.eq(&msg.name)).load::<Object>(conn)?.pop();

        match check_user {
            Some(_) => Err(ServiceError::BadRequest(
                "이미 있는 샵이름입니다.".into(),
            )),
            None => {
                let old_item = tb
                    .filter(&ceo_id.eq(&msg.ceo_id))
                    .get_result::<Object>(conn)?;
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

impl Handler<UpdateNotificationKey> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: UpdateNotificationKey, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        let check_item = tb.filter(&id.eq(&msg.id)).get_result::<Object>(conn)?;
        let item_update = diesel::update(&check_item)
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
