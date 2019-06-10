use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::schema::option_group;
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
#[table_name = "option_group"]
pub struct OptionGroup {
    pub id: i32,
    pub shop_id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize, Debug, Message, Insertable)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "option_group"]
pub struct New {
    pub name: String,
    pub shop_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InpNew {
    pub name: String,
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
            name: self.name.to_string(),
            shop_id: Uuid::parse_str(&shop_id).unwrap(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Message, Identifiable, AsChangeset)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "option_group"]
pub struct Update {
    pub id: i32,
    pub shop_id: Uuid,
    pub name: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InpUpdate {
    pub id: i32,
    pub name: String,
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
    pub fn new(&self, shop_id: String) -> Update {
        Update {
            id: self.id,
            shop_id: Uuid::parse_str(&shop_id).unwrap(),
            name: self.name.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Message, Identifiable)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "option_group"]
pub struct Get {
    pub id: i32,
    pub shop_id: Uuid,
}

#[derive(Deserialize, Serialize, Debug, Message)]
#[rtype(result = "Result<Msg, ServiceError>")]
pub struct GetList {
    pub shop_id: Uuid,
}