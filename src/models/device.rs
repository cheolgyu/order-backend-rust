use crate::schema::user_device;
use crate::errors::ServiceError;
use crate::models::msg::Msg;
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
#[table_name = "user_device"]
pub struct Device {
    pub id: i32,
    pub user_id: Uuid,
    pub name: String,
    pub sw_token: String,

    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}


#[derive(Deserialize, Serialize, Debug, Message, Insertable)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "user_device"]
pub struct New {
    pub user_id: Uuid,
    pub name: String,
    pub sw_token: String,
}

#[derive(Deserialize, Serialize, Debug, Message)]
#[rtype(result = "Result<Msg, ServiceError>")]
pub struct GetList {
    pub user_id: Uuid,
}


#[derive(Deserialize, Serialize, Debug, Message, AsChangeset)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "user_device"]
pub struct Update {
    pub id: i32,
    pub name: String,
    pub sw_token: String,
}
