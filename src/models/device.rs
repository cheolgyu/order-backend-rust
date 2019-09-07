use crate::errors::ServiceError;
use crate::models::fcm::{Params, SendData};
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
#[rtype(result = "Result<GetWithKey, ServiceError>")]
pub struct Get {
    pub sw_token: String,
    pub user_id: Uuid,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetWithKey {
    pub shop_id: String,
    pub notification_key: String,
    pub device_cnt: i64,
    pub params: Get,
}

impl GetWithKey {
    pub fn get(&self, webpush: WebPush) -> Option<SendData> {
        if &self.notification_key == "" {
            Some(SendData {
                url: webpush.reg.clone(),
                webpush: webpush,
                params: Params {
                    operation: "create".to_string(),
                    notification_key_name: self.shop_id.clone(),
                    registration_ids: vec![self.params.sw_token.clone()],
                },
            })
        } else {
            if &self.device_cnt > &0 {
                None
            } else {
                Some(SendData {
                    url: webpush.reg.clone(),
                    webpush: webpush,
                    params: Params {
                        operation: "add".to_string(),
                        notification_key_name: self.shop_id.clone(),
                        registration_ids: vec![self.params.sw_token.clone()],
                    },
                })
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Message, AsChangeset)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "user_device"]
pub struct Update {
    pub id: i32,
    pub name: String,
    pub sw_token: String,
}
