use crate::models::option_group::CartOptionGroup;
use crate::models::shop::Shop;
use crate::schema::product;

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
)]
#[belongs_to(Shop)]
#[table_name = "product"]
pub struct Product {
    pub id: i32,
    pub shop_id: Uuid,
    pub name: String,
    pub price: f64,
    pub p_price: f64,
    pub optg_price: f64,
    pub opt_group: Vec<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CartProduct {
    pub id: i32,
    pub shop_id: Uuid,
    pub name: String,
    pub price: f64,
    pub option_group_list: Vec<CartOptionGroup>,
}
