use crate::errors::ServiceError;
use crate::models::shop::Shop;
use crate::schema::order;
use actix::Message;
use chrono::NaiveDateTime;
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
    pub state: i32,
    pub price: f64,
    pub cnt: i32,
    pub products: serde_json::Value,
    pub sw_token: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize, Debug, Message, Insertable, Clone)]
#[rtype(result = "Result<NewRes, ServiceError>")]
#[table_name = "order"]
pub struct New {
    pub shop_id: Uuid,
    pub state: i32,
    pub price: f64,
    pub cnt: i32,
    pub products: serde_json::Value,
    pub sw_token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewRes {
    pub order: Order,
    pub shop: Shop,
}
