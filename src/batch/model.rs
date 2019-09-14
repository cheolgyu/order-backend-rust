use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::schema::order;
use actix::Message;
use chrono::NaiveDateTime;
use uuid::Uuid;
use crate::models::shop::Shop;


#[derive(Deserialize, Serialize, Debug, Message,  Clone)]
#[rtype(result = "()")]
pub struct OrderState ;