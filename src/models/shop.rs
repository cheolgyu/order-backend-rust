use crate::api::v1::ceo::auth::model::AuthUser;
use crate::api::v1::ceo::product::model::Product;
use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::schema::shop;
use crate::utils::jwt::decode_token;
use crate::utils::validator::{
    re_test_email, re_test_id, re_test_name, re_test_password, re_test_password_contain_num,
    re_test_password_contain_special, Validate,
};
use actix::Message;
use actix_web::{dev::Payload, Error, HttpRequest};
use actix_web::{error, FromRequest};
use bcrypt::{hash, DEFAULT_COST};
use chrono::{Duration, Local, NaiveDateTime, Utc};
use diesel;
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
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}
