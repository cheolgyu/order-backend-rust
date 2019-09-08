use serde::{Deserialize, Serialize};
use actix::Message;
use chrono::NaiveDateTime;
use crate::models::WebPush;
use crate::schema::fcm;
use crate::models::msg::Msg;
use crate::errors::ServiceError;


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
    pub order_id: i32,
    pub kind: String,
    pub resp: serde_json::Value,

    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize, Debug, Message, Insertable)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "fcm"]
pub struct New {
    pub order_id: i32,
    pub kind: String,
    pub resp: serde_json::Value,
}




#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
    pub operation: String,
    pub notification_key_name: String,
    pub registration_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendData {
    pub url: String,
    pub webpush: WebPush,
    pub params: Params,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ParamsToUser {
    pub url: String,
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
    pub res: String,
}

impl ToUserResp {
    pub fn new(&self) -> New {
        New {
            order_id: 111,
            kind: "to_user".to_string(),
            resp: serde_json::to_value(&self).unwrap()
        }
    }
}
