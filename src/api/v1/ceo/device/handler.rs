use crate::models::shop::Shop;
use crate::schema::shop::dsl::{ceo_id, shop};

use crate::errors::ServiceError;
use crate::models::device::{Device as Object, *};
use crate::models::msg::Msg;
use crate::models::DbExecutor;
use crate::schema::user_device::dsl::{id, name, sw_token, user_device as tb, user_id};

use actix::Handler;
use diesel;
use diesel::prelude::*;
use serde_json::json;

// 등록 프로세스 필요.
/**
 * 0. 사용자uuid로 상점의 그룹key 와 토큰 조회
 * 0-1 토큰이 있다면 ok 이미 등록된 디바이스.
 * 0-2 토큰이 없다면 신규 사용자.
 * 0-1-1 기존 디바이스고 푸시키가 없다면 푸시키 발급.
 * 0-1-2 기존 디바이스고 푸시키가 있다면 ok.
 * 0-2-1 신규 디바이스이고 푸시키가 없다면. 푸시키 발급.
 * 0-2-2 신규 디바이스이고 푸시키가 있다면. 푸시키에 신규디바이스 등록.
 *
 * 1. 토큰 db에 저장
 * 2. db 에서 토큰 그룹key 조회
 * 3-1  있다면 기기등록 rest api 실행
 * 3-2 없다면 토큰 그룹key 생성( 생성시 사용자1명 필요.) rest api 실행
 * 3-2-1  생성된 그룹키 db에 저장.
 *
 */
/*
impl Handler<Check> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: Check, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;
        let shop_data = shop.filter(&ceo_id.eq(&msg.user_id))
            .load::<Shop>(conn)?.pop();
        let notification_key =  match shop_data {
            Some(_shop) => _shop.notification_key,
            None => {
                "".to_string()
            }
        };
        let reg_device = tb.filter(&user_id.eq(&msg.user_id))
            .filter(&sw_token.eq(&msg.sw_token))
            .load::<Object>(conn)?.pop();

        match reg_device {
            Some(dv) => {
                if notification_key=="" {
                    // ok
                }else{
                    // request push key
                }
            }

            ),
            None => {

               if notification_key=="" {
                    // request push key
                }else{
                    // reg device in db
                    // request reg device
                }
            }
        }
    }
}
*/

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

impl Handler<Get> for DbExecutor {
    type Result = Result<GetWithKey, ServiceError>;

    fn handle(&mut self, msg: Get, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        //let item = tb.filter(&sw_token.eq(&msg.sw_token)).load::<Object>(conn)?.pop()
        let count = tb
            .filter(&sw_token.eq(&msg.sw_token))
            .count()
            .get_result::<i64>(conn)?;
        let shop_data = shop
            .filter(&ceo_id.eq(&msg.user_id))
            .load::<Shop>(conn)?
            .pop();
        let s_data = match shop_data {
            Some(_shop) => (_shop.id.to_string(), _shop.notification_key),
            None => ("".to_string(), "".to_string()),
        };
        let data = GetWithKey {
            shop_id: s_data.0.clone(),
            notification_key: s_data.1.clone(),
            device_cnt: count.clone(),
            params: msg.clone(),
        };

        Ok(data)
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
