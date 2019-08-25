use crate::schema::order;

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
    pub state: String,
    pub price: f64,
    pub products: serde_json::Value,
    pub req_session_id: serde_json::Value,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}