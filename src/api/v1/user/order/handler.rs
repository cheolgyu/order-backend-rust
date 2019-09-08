use crate::errors::ServiceError;
use crate::models::order::{New, Order as Object,NewRes};
use crate::models::DbExecutor;
use crate::schema::order::dsl::order as tb;
use crate::models::shop::Shop;
use crate::schema::shop::dsl::{shop as tb_shop,id as tb_shop_id};
use actix::Handler;


use crate::models::msg::Msg;
use diesel;
use diesel::prelude::*;

impl Handler<New> for DbExecutor {
    type Result = Result<NewRes, ServiceError>;

    fn handle(&mut self, msg: New, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        let insert: Object = diesel::insert_into(tb)
            .values(&msg)
            .get_result::<Object>(conn)?;
        let shop_data = 
            tb_shop
                .filter(&tb_shop_id.eq(&msg.shop_id))
                .load::<Shop>(conn)?
                .pop().unwrap();

        Ok(NewRes{
            order: insert.clone(),
            shop: shop_data.clone(),
        })
    }
}
