use crate::errors::ServiceError;
//use crate::models::fcm::{Params, ParamsToFcm};
use crate::models::msg::Msg;
use crate::models::WebPush;
use crate::schema::user_device;
use actix::Message;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Clone,
    Debug,
    Serialize,
    Associations,
    Deserialize,
    PartialEq,
    Identifiable,
    Queryable,
    Insertable,
)]
#[table_name = "user_device"]
pub struct Device {
    pub id: i32,
    pub user_id: Uuid,
    pub name: String,
    pub sw_token: String,

    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize, Debug, Message, Insertable)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "user_device"]
pub struct Check {
    pub user_id: Uuid,
    pub name: String,
    pub sw_token: String,
}

#[derive(Deserialize, Serialize, Debug, Message, Insertable)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "user_device"]
pub struct New {
    pub user_id: Uuid,
    pub name: String,
    pub sw_token: String,
}

#[derive(Deserialize, Serialize, Debug, Message)]
#[rtype(result = "Result<Msg, ServiceError>")]
pub struct GetList {
    pub user_id: Uuid,
}

#[derive(Deserialize, Serialize, Debug, Message, Clone)]
#[rtype(result = "Result<GetWithShopRes, ServiceError>")]
pub struct GetWithShop {
    pub sw_token: String,
    pub user_id: Uuid,
}

use diesel::sql_types::{Double, Integer, Json, Text, Uuid as uu};
#[derive(Clone, Debug, Serialize, Deserialize, QueryableByName)]
pub struct GetWithShopRes { 
     #[sql_type = "uu"]
    pub shop_id: Uuid,
    #[sql_type = "Text"]
    pub notification_key: String,
    #[sql_type = "Integer"]
    pub device_cnt: i32,
    #[sql_type = "Text"]
    pub operation: String,
}
/*
impl GetWithShopRes {
    pub fn get(&self, webpush: WebPush) -> Option<ParamsToFcm> {
        if &self.notification_key == "" {
            Some(ParamsToFcm {
                url: webpush.reg.clone(),
                order_id: -1,
                webpush: webpush,
                params: Params {
                    operation: "create".to_string(),
                    notification_key_name: self.shop_id.clone(),
                    notification_key: self.notification_key.clone(),
                    registration_ids: vec![self.params.sw_token.clone()],
                },
            })
        } else {
            if &self.device_cnt > &0 {
                None
            } else {
                Some(ParamsToFcm {
                    url: webpush.reg.clone(),
                    order_id: -1,
                    webpush: webpush,
                    params: Params {
                        operation: "add".to_string(),
                        notification_key_name: self.shop_id.clone(),
                        notification_key: self.notification_key.clone(),
                        registration_ids: vec![self.params.sw_token.clone()],
                    },
                })
            }
        }
    }
}
*/

#[derive(Deserialize, Serialize, Debug, Message, AsChangeset)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "user_device"]
pub struct Update {
    pub id: i32,
    pub name: String,
    pub sw_token: String,
}
