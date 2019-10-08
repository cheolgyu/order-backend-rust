use crate::schema::option;
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
#[table_name = "option"]
pub struct Opt {
    pub id: i32,
    pub shop_id: Uuid,
    pub name: String,
    pub price: f64,
    pub html_type: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CartOpt {
    pub id: i32,
    pub shop_id: Uuid,
    pub name: String,
    pub price: f64,
    pub html_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize,Insertable,Queryable)]
#[table_name = "option"]
pub struct OptRes {
    #[column_name = "id"]
    pub o_id: i32,
    #[column_name = "name"]
    pub o_nm: String,
    #[column_name = "price"]
    pub o_price: f64,
    #[column_name = "html_type"]
    pub o_html_type: String,
    
}