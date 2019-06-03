use crate::errors::ServiceError;
use crate::schema::{product, shop as tb_shop};
use crate::svc::auth::model::AuthUser;
//use crate::svc::option::model::Opt;
//use crate::svc::option_group::model::OptGroup;
use crate::models::msg::Msg;
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
    pub option_group: serde_json::Value,
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
    pub option_group: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Opt {
    pub name: String,
    pub price: f64,
    pub soft: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OptGroup {
    pub name: String,
    pub opt: Vec<Opt>,
    pub kind: String,
    pub soft: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InpNew {
    pub name: String,
    pub price: f64,
    pub option_group: Option<Vec<OptGroup>>,
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
    pub fn new(&self, shop_id: String, opts: Option<Vec<OptGroup>>) -> New {
        New {
            shop_id: Uuid::parse_str(&shop_id).unwrap(),
            name: self.name.to_string(),
            price: self.price,
            option_group: serde_json::to_value(opts).unwrap(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Message, Identifiable, AsChangeset)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "product"]
pub struct Update {
    pub id: i32,
    pub name: String,
    pub price: f64,
    pub option_group: serde_json::Value,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InpUpdate {
    pub id: i32,
    pub name: String,
    pub price: f64,
    pub option_group: Option<Vec<OptGroup>>,
}

impl Validate for InpUpdate {
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

impl InpUpdate {
    pub fn new(&self, opts: Option<Vec<OptGroup>>) -> Update {
        Update {
            id: self.id,
            name: self.name.to_string(),
            price: self.price,
            option_group: serde_json::to_value(opts).unwrap(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Message, Identifiable)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "product"]
pub struct Get {
    pub id: i32,
}

#[derive(Deserialize, Serialize, Debug, Message)]
#[rtype(result = "Result<Msg, ServiceError>")]
pub struct GetList {
    pub shop_id: Uuid
}
