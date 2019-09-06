use crate::errors::ServiceError;
use crate::models::msg::Msg;
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
#[table_name = "fcm"]
pub struct Fcm {
    pub id: i32,
    pub operation: String,
    pub notification_key_name: String,
    pub registration_ids: Vec<String>,

    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize, Debug, Message, Insertable)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "fcm"]
pub struct SendData {
    pub operation: String,
    pub notification_key_name: String,
    pub registration_ids: Vec<String>,
}
