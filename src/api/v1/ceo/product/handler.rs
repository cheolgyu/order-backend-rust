use crate::api::v1::ceo::product::model::{
    Delete, Get, GetList, New, Product as Object, SimpleProduct, Update,
};
use crate::errors::ServiceError;
use crate::models::DbExecutor;
use crate::schema::product::dsl::{deleted_at, id, name, product as tb, shop_id};
use actix::Handler;



use diesel;
use diesel::prelude::*;

impl Handler<New> for DbExecutor {
    type Result = Result<Object, ServiceError>;

    fn handle(&mut self, msg: New, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        let check = tb.filter(&name.eq(&msg.name)).load::<Object>(conn)?.pop();

        match check {
            Some(_) => Err(ServiceError::BadRequest("중복".into())),
            None => {
                let insert: Object = diesel::insert_into(tb)
                    .values(&msg)
                    .get_result::<Object>(conn)?;

                Ok(insert)
            }
        }
    }
}
use crate::models::msg::Msg;
impl Handler<Update> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: Update, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        let old_item = tb.filter(&id.eq(&msg.id)).get_result::<Object>(conn)?;
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

    fn handle(&mut self, msg: GetList, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;
        use diesel::sql_query;
        use diesel::sql_types::Uuid;

        let res = sql_query("
        SELECT p.id                      AS id, 
            p.shop_id                    AS shop_id, 
            p.name                       AS name, 
            p.price                      AS price, 
            case when array_length(p.opt_group,1) is null then '[]' else Array_to_json(Array_agg(optg.*))  end as option_group_list 
        FROM   PRODUCT AS p 
            LEFT JOIN OPTION_GROUP AS optg 
                ON optg.id = Any(p.opt_group) 
        WHERE  p.shop_id = $1 AND p.deleted_at is null
        GROUP  BY p.id 
        ").bind::<Uuid, _>(&msg.shop_id)
        .get_results::<SimpleProduct>(conn)?;

        let payload = serde_json::json!({
            "items": res,
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
