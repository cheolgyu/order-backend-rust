use crate::errors::ServiceError;
use crate::models::msg::Msg;

use crate::schema::option;

use crate::utils::validator::{re_test_name, Validate};
use actix::Message;
use actix_web::error;
use actix_web::Error;

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

#[derive(Deserialize, Serialize, Debug, Message, Insertable)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "option"]
pub struct New {
    pub name: String,
    pub shop_id: Uuid,
    pub price: f64,
    pub html_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InpNew {
    pub name: String,
    pub price: String,
    pub html_type: String,
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
            price: self.price.parse().unwrap(),
            html_type: self.html_type.parse().unwrap(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Message, Identifiable, AsChangeset)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "option"]
pub struct Update {
    pub id: i32,
    pub shop_id: Uuid,
    pub name: String,
    pub price: f64,
    pub html_type: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InpUpdate {
    pub id: i32,
    pub name: String,
    pub price: String,
    pub html_type: String,
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
            price: self.price.parse().unwrap(),
            html_type: self.html_type.parse().unwrap(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Message, Identifiable)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "option"]
pub struct Get {
    pub id: i32,
    pub shop_id: Uuid,
}

#[derive(Deserialize, Serialize, Debug, Message)]
#[rtype(result = "Result<Msg, ServiceError>")]
pub struct GetList {
    pub shop_id: Uuid,
}

#[derive(Deserialize, Serialize, Debug, Message, Identifiable, AsChangeset)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "option"]
pub struct Delete {
    pub id: i32,
    pub shop_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InpDelete {
    pub id: i32,
}

impl Validate for InpDelete {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl InpDelete {
    pub fn new(&self, shop_id: String) -> Delete {
        Delete {
            id: self.id,
            shop_id: Uuid::parse_str(&shop_id).unwrap(),
        }
    }
}
