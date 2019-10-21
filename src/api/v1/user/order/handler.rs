use crate::errors::ServiceError;
use crate::models::order::{New, NewRes,Order as Object, OrderRes};
use crate::models::shop::Shop;
use crate::models::DbExecutor;
use crate::schema::order::dsl::order as tb;
use crate::schema::shop::dsl::{id as tb_shop_id, shop as tb_shop};
use actix::Handler;

use diesel;
use diesel::prelude::*;

impl Handler<New> for DbExecutor {
    type Result = Result<NewRes, ServiceError>;

    fn handle(&mut self, msg: New, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        use diesel::sql_types::{Uuid,Text,Integer,Double};
        use diesel::pg::types::sql_types::Jsonb;

        let insert_and_id = diesel::sql_query(
            " select insert_order($1, $2, $3, $4, $5, $6)
        ",
        )
        .bind::<Uuid, _>(&msg.shop_id)
        .bind::<Integer, _>(&msg.state)
        .bind::<Double, _>(&msg.price)
        .bind::<Integer, _>(&msg.cnt)
        .bind::<Jsonb, _>(&msg.products)
        .bind::<Text, _>(&msg.sw_token)
        .get_result::<OrderRes>(conn)?;

        let o = tb.find(&insert_and_id.insert_order).get_result::<Object>(conn)?;

        let shop_data = tb_shop
            .filter(&tb_shop_id.eq(&msg.shop_id))
            .load::<Shop>(conn)?
            .pop()
            .unwrap();

        Ok(NewRes {
            order: o.clone(),
            shop: shop_data.clone(),
        })
    }
}
