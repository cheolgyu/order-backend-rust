use crate::schema::order_detail;

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
#[table_name = "order_detail"]
pub struct OrderDetail {
    pub id: i32,
    pub order_id: i32,
    pub shop_id: Uuid,
    pub state: String,
    pub txt: serde_json::Value,
    pub req_session_id: serde_json::Value,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}