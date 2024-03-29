use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::schema::shop;
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
//#[has_many(Product)]
#[table_name = "shop"]
pub struct Shop {
    pub id: Uuid,
    pub ceo_id: Uuid,
    pub name: String,
    pub products: Option<serde_json::Value>,
    pub notification_key: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize, Debug, Message, Identifiable, AsChangeset)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "shop"]
pub struct UpdateNotificationKey {
    pub id: Uuid,
    pub notification_key: String,
}
