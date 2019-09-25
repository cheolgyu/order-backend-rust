use crate::errors::ServiceError;
use crate::models::{AppStateWithTxt, DbExecutor};
use actix::prelude::*;
use actix::Message;
use actix_web::web;
use diesel::sql_types::{Integer, Text, Uuid as uu};
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "Result<Vec<ComeFindRes>, ServiceError>")]
pub struct ComeFind {
    pub db: web::Data<Addr<DbExecutor>>,
    pub store: web::Data<AppStateWithTxt>,
}

#[derive(Clone, Debug, Serialize, Deserialize, QueryableByName)]
pub struct ComeFindRes {
    #[sql_type = "Integer"]
    pub order_id: i32,
    #[sql_type = "Integer"]
    pub order_detail_id: i32,
    #[sql_type = "uu"]
    pub shop_id: Uuid,
    #[sql_type = "Integer"]
    pub shop_notification_id: i32,
    #[sql_type = "Text"]
    pub to: String,
    #[sql_type = "Text"]
    pub content: String,
    #[sql_type = "Text"]
    pub shop_name: String,
}

#[derive(Message)]
#[rtype(result = "Result<Vec<AutoCancelRes>, ServiceError>")]
pub struct AutoCancel {
    pub db: web::Data<Addr<DbExecutor>>,
    pub store: web::Data<AppStateWithTxt>,
}

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
