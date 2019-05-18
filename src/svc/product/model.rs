use crate::errors::ServiceError;
use crate::schema::{product, shop as tb_shop};
use crate::svc::auth::model::AuthUser;
use crate::svc::shop::model::Shop;
use crate::utils::jwt::decode_token;
use crate::utils::validator::{
    re_test_email, re_test_id, re_test_name, re_test_password, re_test_password_contain_num,
    re_test_password_contain_special, Validate,
};
use actix::Message;
use actix_web::{dev::Payload, Error, HttpRequest};
use actix_web::{error, middleware::identity::Identity, FromRequest};
use bcrypt::{hash, DEFAULT_COST};
use chrono::{Duration, Local, NaiveDateTime, Utc};
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
)]
#[belongs_to(Shop)]
#[table_name = "product"]
pub struct Product {
    pub id: i32,
    pub shop_id: Uuid,
    pub name: String,
    pub price: Option<f64>,

    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize, Debug, Message, Insertable)]
#[rtype(result = "Result<Product, ServiceError>")]
#[table_name = "product"]
pub struct New {
    pub shop_id: Uuid,
    pub name: String,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InpNew {
    pub name: String,
    pub price: f64,
}

impl Validate for InpNew {
    fn validate(&self) -> Result<(), Error> {
        let name = &self.name;
        let check_name = re_test_name(name);

        if check_name {
            Ok(())
        } else {
            Err(error::ErrorBadRequest("option name"))
        }
    }
}

impl InpNew {
    pub fn new(&self, shop_id: String) -> New {
        New {
            shop_id: Uuid::parse_str(&shop_id).unwrap(),
            name: self.name.to_string(),
            price: self.price,
        }
    }
}