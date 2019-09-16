use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::schema::order;
use actix::Message;
use chrono::NaiveDateTime;
use uuid::Uuid;
use crate::models::shop::Shop;
use crate::models::{AppStateWithTxt, DbExecutor};
use actix_web::{
    client::Client,
    web,
    };
use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "Result<Vec<OrderStateRes>, ServiceError>")]
pub struct OrderState{
    pub db: web::Data<Addr<DbExecutor>>,
    pub store: web::Data<AppStateWithTxt>
}
/*
pub struct OrderState{
    db: Addr<DbExecutor>,
    client: Client,
    store: AppStateWithTxt
}

*/
use diesel::sql_types::{Integer, Json, Text, Uuid as uu};
#[derive(Clone, Debug, Serialize, Deserialize, QueryableByName)]
pub struct OrderStateRes {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "uu"]
    pub shop_id: Uuid,
    #[sql_type = "Text"]
    pub sw_token: String,
    #[sql_type = "Text"]
    pub notification_key: String,
}