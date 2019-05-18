use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::models::DbExecutor;
use crate::svc::product::model::Product;
use crate::svc::shop::model::{NewShop, Shop, ShopID};
use crate::utils::hash_password;
use actix::Handler;
use actix::Message;
use actix_web::{error, Error};
use bcrypt::verify;
use diesel;
use diesel::prelude::*;
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

impl Handler<NewShop> for DbExecutor {
    type Result = Result<Shop, ServiceError>;

    fn handle(&mut self, msg: NewShop, _: &mut Self::Context) -> Self::Result {
        use crate::schema::shop;
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

                Ok(insert)
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
