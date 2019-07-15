use crate::schema::shop;

use chrono::{Duration, Local, NaiveDateTime, Utc};

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
//#[has_many(Product)]
#[table_name = "shop"]
pub struct Shop {
    pub id: Uuid,
    pub ceo_id: Uuid,
    pub name: String,
    pub products: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}
