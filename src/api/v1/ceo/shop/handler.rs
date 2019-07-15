use crate::api::v1::ceo::product::model::Product;
use crate::api::v1::ceo::shop::model::{NewShop, ShopID};
use crate::errors::ServiceError;
use crate::model::msg::Msg;
use crate::model::shop::Shop;
use crate::model::DbExecutor;

use actix::Handler;

use diesel;
use diesel::prelude::*;

use serde_json::json;

impl Handler<NewShop> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: NewShop, _: &mut Self::Context) -> Self::Result {
        use crate::schema::shop::dsl::*;
        let conn = &self.0.get()?;

        let check_user = shop
            .filter(&ceo_id.eq(&msg.ceo_id))
            .load::<Shop>(conn)?
            .pop();

        match check_user {
            Some(_) => Err(ServiceError::BadRequest("중복".into())),
            None => {
                let insert: Shop = diesel::insert_into(shop)
                    .values(&msg)
                    .get_result::<Shop>(conn)?;

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
impl Handler<ShopID> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: ShopID, _: &mut Self::Context) -> Self::Result {
        use crate::schema::shop::dsl::{id, shop as tb};
        let conn = &self.0.get()?;
        let shop = tb.filter(&id.eq(&msg.id)).load::<Shop>(conn)?.pop();

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
