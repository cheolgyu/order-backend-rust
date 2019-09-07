use serde::{Deserialize, Serialize};
use crate::models::{WebPush};

#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
    pub operation: String,
    pub notification_key_name: String,
    pub registration_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendData {
    pub url: String,
    pub webpush : WebPush,
    pub params: Params,
}
