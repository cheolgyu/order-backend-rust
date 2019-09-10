use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::schema::option_group;
use crate::models::option::CartOpt;

use crate::utils::validator::{re_test_name, Validate};
use actix::Message;
use actix_web::Error;

use chrono::NaiveDateTime;
use diesel;
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
    Associations,
    QueryableByName,
)]
#[table_name = "option_group"]
pub struct OptionGroup {
    pub id: i32,
    pub shop_id: Uuid,
    pub name: String,
    pub default: i32,
    pub options: Vec<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
)]
pub struct CartOptionGroup {
    pub id: i32,
    pub shop_id: Uuid,
    pub name: String,
    pub default: i32,
    pub select_opt_id: i32,
    pub select_opt_name: String,
    pub select_opt_price: f64,
    pub option_list: Vec<CartOpt>,
}



/*
use diesel::sql_types::{Integer, Json, Text, Uuid as uu, Double};
#[derive(Clone, Debug, Serialize, Deserialize, QueryableByName)]
pub struct SimpleOptionGroup {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "uu"]
    pub shop_id: Uuid,
    #[sql_type = "Text"]
    pub name: String,
    #[sql_type = "Integer"]
    pub default: i32,
    #[sql_type = "Integer"]
    pub select_opt_id: i32,
    #[sql_type = "Json"]
    pub option_list: serde_json::Value,
}


use diesel::sql_types::{Integer, Json, Text, Double, Uuid as uu};
#[derive(Clone, Debug, Serialize, Deserialize, QueryableByName)]
pub struct SimpleOptionGroup {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "uu"]
    pub shop_id: Uuid,
    #[sql_type = "Text"]
    pub name: String,
    #[sql_type = "Integer"]
    pub default: i32,
    #[sql_type = "Integer"]
    pub select_opt_id: i32,
    #[sql_type = "Text"]
    pub select_opt_name: String,
    #[sql_type = "Double"]
    pub select_opt_price: String,
    #[sql_type = "Json"]
    pub option_list: serde_json::Value,
}
*/