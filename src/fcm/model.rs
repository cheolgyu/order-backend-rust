use crate::errors::ServiceError;
use crate::models::fcm::New;
use crate::models::msg::Msg;
use actix::Message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReqToComm {
    pub to: String,
    pub order_id: i32,
    pub order_detail_id: i32,
    pub shop_notification_id: i32,
    pub order_detail_state: i32,
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
            shop_notification_id: self.shop_notification_id,
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
            shop_notification_id: -1,
            order_detail_state: -1,
            trigger: "device::check".to_string(),
            req: serde_json::json!(null),
            resp: serde_json::json!(null),
        }
    }

    pub fn new_auto_cancle(order_id: i32) -> ReqToComm {
        ReqToComm {
            to: "fcm".to_string(),
            order_id: order_id,
            order_detail_id: -1,
            shop_notification_id: -1,
            order_detail_state: -1,
            trigger: "batch::auto_cancle".to_string(),
            req: serde_json::json!(null),
            resp: serde_json::json!(null),
        }
    }

    pub fn new_comefind(
        order_id: i32,
        order_detail_id: i32,
        shop_notification_id: i32,
    ) -> ReqToComm {
        ReqToComm {
            to: "user".to_string(),
            order_id: order_id.clone(),
            order_detail_id: order_detail_id.clone(),
            shop_notification_id: shop_notification_id.clone(),
            order_detail_state: 2,
            trigger: "batch::comfind".to_string(),
            req: serde_json::json!(null),
            resp: serde_json::json!(null),
        }
    }
    pub fn new_order(order_id: i32) -> ReqToComm {
        ReqToComm {
            to: "user".to_string(),
            order_id: order_id,
            order_detail_id: -1,
            shop_notification_id: -1,
            order_detail_state: -1,
            trigger: "new_order".to_string(),
            req: serde_json::json!(null),
            resp: serde_json::json!(null),
        }
    }

    pub fn new_order_detail(
        order_id: i32,
        order_detail_id: i32,
        order_detail_state: i32,
    ) -> ReqToComm {
        ReqToComm {
            to: "user".to_string(),
            order_id: order_id,
            order_detail_id: order_detail_id,
            order_detail_state: order_detail_state,
            shop_notification_id: -1,
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
#[derive(Debug, Serialize, Deserialize, Message, Clone)]
#[rtype(result = "Result<Msg, ServiceError>")]
pub struct ReqToUser {
    pub comm: ReqToComm,
    pub params: ReqToUserData,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReqToUserData {
    pub to: String,
    pub notification: Notification,
    pub data: serde_json::Value,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Notification {
    pub title: String,
    pub body: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct RespUser {
    pub success: i32,
    pub failure: i32,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Data {
    pub title: String,
    pub my_options: MyOptions,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MyOptions {
    pub body: String,
    pub icon: String,
    pub tag: String,
    pub click_action: String,
    pub vibrate: Vec<i32>,
}

impl ReqToUserData {
    pub fn new(target: String, title: String, body: String) -> ReqToUserData {
        ReqToUserData {
            to: target,
            notification: Notification {
                title: title.clone(),
                body: body.clone(),
            },
            data: serde_json::json!(Data {
                title: title.clone(),
                my_options: MyOptions {
                    body: body.clone(),
                    icon: "/icon.png".to_string(),
                    tag: "my_tag".to_string(),
                    click_action: "https://naver.com".to_string(),
                    vibrate: vec![200, 100, 200, 100, 200, 100, 400],
                },
            }),
        }
    }
}
