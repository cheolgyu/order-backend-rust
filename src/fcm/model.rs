use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::models::WebPush;
use crate::schema::fcm;
use actix::Message;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use futures::Future;

///////////////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize,Message)]
#[rtype(result = "Box<dyn Future<Item =Result<Msg, ServiceError>, Error = ServiceError>>")]
pub struct ReqToFcm {
    pub order_id: i32,
    pub params: ReqToFcmData,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReqToFcmData {
    pub operation: String,
    pub notification_key_name: String,
    pub notification_key: String,
    pub registration_ids: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RespFcm {
    pub notification_key: String,
}

///////////////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize,Message)]
#[rtype(result = "Box<dyn Future<Item =Result<Msg, ServiceError>, Error = ServiceError>>")]
pub struct ReqToUser {
    pub order_id: i32,
    pub params: ReqToUserData,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReqToUserData {
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
pub struct RespUser {
    pub success: i32,
    pub failure: i32,
}
