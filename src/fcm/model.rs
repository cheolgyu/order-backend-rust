use crate::errors::ServiceError;
use crate::models::fcm::New;
use crate::models::msg::Msg;
use actix::Message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReqToComm {
    pub to: String,
    pub order_id: i32,
    pub order_detail_id: i32,
    pub order_detail_state: String,
    pub trigger: String,
    pub req: serde_json::Value,
    pub resp: serde_json::Value,
}
impl ReqToComm {
    pub fn get_new(self) -> New {
        New {
            to: self.to,
            order_id: self.order_id,
            order_detail_id: self.order_detail_id,
            order_detail_state: self.order_detail_state,
            trigger: self.trigger,
            req: self.req,
            resp: self.resp,
        }
    }
    pub fn new_fcm() -> ReqToComm {
        ReqToComm {
            to: "fcm".to_string(),
            order_id: -1,
            order_detail_id: -1,
            order_detail_state: "".to_string(),
            trigger: "device::check".to_string(),
            req: serde_json::json!(null),
            resp: serde_json::json!(null),
        }
    }

    pub fn new_auto_cancle(trigger: String, order_id: i32) -> ReqToComm {
        ReqToComm {
            to: "fcm".to_string(),
            order_id: order_id,
            order_detail_id: -1,
            order_detail_state: "".to_string(),
            trigger: trigger,
            req: serde_json::json!(null),
            resp: serde_json::json!(null),
        }
    }

    pub fn new_order(trigger: String, order_id: i32) -> ReqToComm {
        ReqToComm {
            to: "user".to_string(),
            order_id: order_id,
            order_detail_id: -1,
            order_detail_state: "".to_string(),
            trigger: trigger,
            req: serde_json::json!(null),
            resp: serde_json::json!(null),
        }
    }

    pub fn new_order_detail(order_id: i32,order_detail_id:i32, order_detail_state: String) -> ReqToComm {
        ReqToComm {
           to: "user".to_string(),
            order_id: order_id,
            order_detail_id: order_detail_id,
            order_detail_state: order_detail_state,
            trigger: "new order detail".to_string(),
            req: serde_json::json!(null),
            resp: serde_json::json!(null),
        }
    }
}
///////////////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize, Message)]
#[rtype(result = "Result<Msg, ServiceError>")]
pub struct ReqToFcm {
    pub comm: ReqToComm,
    pub params: ReqToFcmData,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReqToFcmData {
    pub operation: String,
    pub notification_key_name: String,
    pub notification_key: String,
    pub registration_ids: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct RespFcm {
    pub notification_key: String,
}

///////////////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize, Message)]
#[rtype(result = "Result<Msg, ServiceError>")]
pub struct ReqToUser {
    pub comm: ReqToComm,
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
#[derive(Deserialize, Serialize, Debug)]
pub struct RespUser {
    pub success: i32,
    pub failure: i32,
}
