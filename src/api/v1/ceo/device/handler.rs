use crate::errors::ServiceError;
use crate::models::device::{Device as Object,New,GetWithShop, GetWithShopRes,GetList,Update};
use crate::models::msg::Msg;
use crate::models::DbExecutor;
use crate::schema::user_device::dsl::{id, name, sw_token, user_device as tb, user_id};

use actix::Handler;
use diesel;
use diesel::prelude::*;
use serde_json::json;

use diesel::sql_query;
use diesel::sql_types::{Uuid,Text};


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

impl Handler<GetWithShop> for DbExecutor {
    type Result = Result<GetWithShopRes, ServiceError>;

    fn handle(&mut self, msg: GetWithShop, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        let q = sql_query("
       SELECT
            a.shop_id, 
            a.notification_key, 
            a.device_cnt, 
            CASE 
                WHEN a.notification_key != '' 
                    AND a.device_cnt > 0 THEN 'pass' 
                WHEN a.notification_key != '' THEN 'add' 
                WHEN a.notification_key = '' THEN 'create' 
                ELSE '' 
            END AS operation 
        FROM   (SELECT s.id               AS shop_id, 
                    s.notification_key AS notification_key, 
                    Count(d.id)        AS device_cnt 
                FROM   shop s 
                    LEFT JOIN user_device d 
                            ON s.ceo_id = d.user_id 
                            AND d.sw_token = $2 
                WHERE  s.ceo_id = $1
                GROUP  BY s.id) a      
        ")
        .bind::<Uuid, _>(&msg.user_id)
        .bind::<Text, _>(&msg.sw_token);
        let res = q.get_result::<GetWithShopRes>(conn).expect(" 쿼리오류 ");

        Ok(res)
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
