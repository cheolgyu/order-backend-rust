use crate::errors::ServiceError;
use crate::models::msg::Msg;
use actix::Message;
use serde::{Deserialize, Serialize};

///////////////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize,Message)]
#[rtype(result = "Result<Msg, ServiceError>")]
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
#[rtype(result = "Result<Msg, ServiceError>")]
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
