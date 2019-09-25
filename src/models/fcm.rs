use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::schema::fcm;
use actix::Message;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

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
#[table_name = "fcm"]
pub struct Fcm {
    pub id: i32,
    pub to: String,
    pub order_id: i32,
    pub order_detail_id: i32,
    pub shop_notification_id: i32,

    pub order_detail_state: i32,
    pub trigger: String,

    pub req: serde_json::Value,
    pub resp: serde_json::Value,

    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize, Debug, Message, Insertable)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "fcm"]
pub struct New {
    pub to: String,
    pub order_id: i32,
    pub order_detail_id: i32,
    pub shop_notification_id: i32,
    pub order_detail_state: i32,
    pub trigger: String,
    pub req: serde_json::Value,
    pub resp: serde_json::Value,
}

/*

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ToFcmResp {
    pub notification_key: String,
}
impl ToFcmResp {
    pub fn new(&self, order_id: i32) -> New {
        New {
            order_id: order_id,
            resp: serde_json::to_value(&self).unwrap(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
    pub operation: String,
    pub notification_key_name: String,
    pub notification_key: String,
    pub registration_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParamsToFcm {
    pub url: String,
    pub order_id: i32,
    pub webpush: WebPush,
    pub params: Params,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParamsToUser {
    pub url: String,
    pub order_id: i32,
    pub webpush: WebPush,
    pub params: ParamsNotification,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParamsNotification {
    pub notification: Notification,
    pub to: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    pub title: String,
    pub body: String,
    pub icon: String,
    pub click_action: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ToUserResp {
    pub success: i32,
    pub failure: i32,
}

impl ToUserResp {
    pub fn new(&self, order_id: i32) -> New {
        New {
            order_id: order_id,
            resp: serde_json::to_value(&self).unwrap(),
        }
    }
}

*/
