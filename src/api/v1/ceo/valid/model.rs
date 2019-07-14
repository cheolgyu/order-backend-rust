use crate::errors::ServiceError;
use crate::models::msg::Msg;

use crate::schema::valid;

use actix::Message;

use chrono::{Duration, Local, NaiveDateTime, Utc};

use uuid::Uuid;
#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Identifiable,
    Queryable,
    Insertable,
    QueryableByName,
)]
#[table_name = "valid"]
pub struct Valid {
    pub id: Uuid,
    pub user_id: Uuid,
    pub kind: String,
    pub kind_value: String,
    pub code: String,
    pub req: Option<String>,
    pub res: Option<String>,
    pub created_at: NaiveDateTime,
    pub valid_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Insertable, Message, Clone)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "valid"]
pub struct New {
    pub user_id: Uuid,
    pub kind: String,
    pub kind_value: String,
    pub code: String,
    pub valid_at: NaiveDateTime,
    pub req: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InpNew {
    pub user_id: Uuid,
    pub kind: String,
    pub kind_value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Message)]
#[rtype(result = "Result<Msg, ServiceError>")]
pub struct ChkValid {
    pub v: InpNew,
    pub code: String,
}
