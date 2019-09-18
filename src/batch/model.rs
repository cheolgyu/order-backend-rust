use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::models::shop::Shop;
use crate::models::{AppStateWithTxt, DbExecutor};
use crate::schema::order;
use actix::prelude::*;
use actix::Message;
use actix_web::{client::Client, web};
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "Result<Vec<AutoCancelRes>, ServiceError>")]
pub struct AutoCancel {
    pub db: web::Data<Addr<DbExecutor>>,
    pub store: web::Data<AppStateWithTxt>,
}

use diesel::sql_types::{Integer, Json, Text, Uuid as uu};
#[derive(Clone, Debug, Serialize, Deserialize, QueryableByName)]
pub struct AutoCancelRes {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "uu"]
    pub shop_id: Uuid,
    #[sql_type = "Text"]
    pub sw_token: String,
    #[sql_type = "Text"]
    pub notification_key: String,
}
