use crate::schema::order;
use crate::models::msg::Msg;
use crate::errors::ServiceError;
use chrono::NaiveDateTime;
use actix::Message;
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
#[table_name = "order"]
pub struct Order {
    pub id: i32,
    pub shop_id: Uuid,
    pub state: String,
    pub price: f64,
    pub products: serde_json::Value,
    pub sw_token: serde_json::Value,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize, Debug, Message, Insertable)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "order"]
pub struct New {
    pub shop_id: Uuid,
    pub state: String,
    pub price: f64,
    pub products: serde_json::Value,
    pub sw_token: serde_json::Value,
}