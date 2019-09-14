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
use diesel::sql_types::{Integer, Json, Text, Uuid as uu};
#[derive(Clone, Debug, Serialize, Deserialize, QueryableByName)]
pub struct OrderIds {
    #[sql_type = "Integer"]
    pub id: i32,
}