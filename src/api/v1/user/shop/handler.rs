use crate::api::v1::ceo::product::model::{ShopInfo};
use crate::api::v1::user::shop::model::{GetList, GetWithId};
use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::models::shop::Shop;
use crate::models::DbExecutor;
use crate::schema::shop::dsl::{id, shop as tb};
use actix::Handler;
use diesel;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Uuid;
use serde_json::json;

impl Handler<GetWithId> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: GetWithId, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;
        let shop = tb.filter(&id.eq(&msg.id)).load::<Shop>(conn)?.pop();
        match shop {
            Some(_shop) => {
                let res = sql_query(
                    "
                    select s_id,s_info
                    from 
                    view_shop_info_user
                    where s_id = $1
                ",
                )
                .bind::<Uuid, _>(&msg.id)
                .get_result::<ShopInfo>(conn)?;

                let payload = json!({
                    "shop_info": res,
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

impl Handler<GetList> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, _msg: GetList, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;
        let shops = tb.load::<Shop>(conn)?;
        let payload = json!({
            "shops": shops,
        });

        Ok(Msg {
            status: 200,
            data: payload,
        })
    }
}
