use crate::api::v1::ceo::option_group::model::{
    Delete, Get, GetList, New, SimpleOptionGroup, Update,
};
use crate::errors::ServiceError;
use crate::models::option_group::OptionGroup as Object;
use crate::models::DbExecutor;
use crate::schema::option_group::dsl::{deleted_at, id, name, option_group as tb, shop_id};
use actix::Handler;

use diesel;
use diesel::prelude::*;

impl Handler<New> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: New, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;
        let check = tb
            .filter(&shop_id.eq(&msg.shop_id))
            .filter(&name.eq(&msg.name))
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

        let old_item = tb
            .filter(&id.eq(&msg.id))
            .filter(&shop_id.eq(&msg.shop_id))
            .get_result::<Object>(conn)?;
        let item_update = diesel::update(&old_item)
            .set(&msg)
            .get_result::<Object>(conn)?;
        let payload = serde_json::json!({
            "item_update": item_update,
        });
        Ok(Msg {
            status: 201,
            data: payload,
        })
    }
}

impl Handler<Delete> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: Delete, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        let old_item = tb
            .filter(&id.eq(&msg.id))
            .filter(&shop_id.eq(&msg.shop_id))
            .get_result::<Object>(conn)?;
        let item_delete = diesel::update(&old_item)
            .set(deleted_at.eq(diesel::dsl::now))
            .get_result::<Object>(conn)?;
        //deleted_at
        let payload = serde_json::json!({
            "item_delete": item_delete,
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

        let item = tb.filter(&id.eq(&msg.id)).get_result::<Object>(conn)?;

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
        use diesel::sql_query;
        use diesel::sql_types::Uuid;

        let res = sql_query("
SELECT og.id 
       AS 
       og_id, 
       og.name 
       AS og_nm, 
       og.default 
       AS og_default, 
       Json_agg(Json_build_object('o_id', o.id, 'o_nm', o.name, 'o_price', 
                o.price, 
                         'o_html_type', o.html_type, 'og_default', og.default)) 
       AS o 
FROM   option_group og 
       left join OPTION o 
              ON o.id = ANY ( og.options ) 
WHERE  og.deleted_at IS NULL 
        AND og.shop_id = $1
GROUP  BY og.id, 
          og.name
        ").bind::<Uuid, _>(&_msg.shop_id)
        .get_results::<SimpleOptionGroup>(conn)?;

        let payload = serde_json::json!({
            "items": res,
        });
        Ok(Msg {
            status: 201,
            data: payload,
        })
    }
}
