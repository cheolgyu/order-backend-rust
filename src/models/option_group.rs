use crate::models::option::CartOpt;
use crate::schema::option_group;

use chrono::NaiveDateTime;
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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
