use crate::errors::ServiceError;
use crate::models::DbExecutor;
use crate::schema::option_group::dsl::{id, name, option_group as tb, shop_id};
use crate::svc::option_group::model::{
    Get, GetList, InpNew, New, OptionGroup as Object, SimpleOptionGroup, Update,
};
use actix::Handler;
use actix::Message;
use actix_web::{error, Error};
use bcrypt::verify;
use diesel;
use diesel::prelude::*;
use uuid::Uuid;

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
        SELECT optg.id                      AS id, 
            optg.shop_id                    AS shop_id, 
            optg.name                       AS name, 
            case when count(opt.id) = 0 then '[]' else Array_to_json(Array_agg(opt.*))  end as option_list 
        FROM   option_group AS optg 
            LEFT JOIN OPTION AS opt 
                ON opt.id = Any(optg.options) 
        WHERE  optg.shop_id = $1 
        GROUP  BY optg.id 
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
