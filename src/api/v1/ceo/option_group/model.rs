use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::schema::option_group;
use crate::utils::validator::{re_test_name, Validate};
use actix::Message;
use actix_web::error;
use actix_web::Error;
use diesel;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Message, Insertable)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "option_group"]
pub struct New {
    pub name: String,
    pub shop_id: Uuid,
    pub default: i32,
    pub options: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InpNew {
    pub name: String,
    pub default: i32,
    pub options: Vec<i32>,
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
    pub fn new(&self, shop_id: Uuid) -> New {
        New {
            name: self.name.to_string(),
            shop_id: shop_id,
            options: self.options.clone(),
            default: self.default.clone(),
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
    pub default: i32,
    pub options: Vec<i32>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InpUpdate {
    pub id: i32,
    pub name: String,
    pub default: i32,
    pub options: Vec<i32>,
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
    pub fn new(&self, shop_id: Uuid) -> Update {
        Update {
            id: self.id,
            shop_id: shop_id,
            name: self.name.to_string(),
            options: self.options.clone(),
            default: self.default.clone(),
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

#[derive(Deserialize, Serialize, Debug, Message, Identifiable, AsChangeset)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "option_group"]
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
    pub fn new(&self, shop_id: Uuid) -> Delete {
        Delete {
            id: self.id,
            shop_id: shop_id,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Message)]
#[rtype(result = "Result<Msg, ServiceError>")]
pub struct GetList {
    pub shop_id: Uuid,
}
use diesel::sql_types::{Integer, Json, Text, Uuid as uu};
#[derive(Clone, Debug, Serialize, Deserialize, QueryableByName)]
pub struct SimpleOptionGroup {
    #[sql_type = "Integer"]
    pub og_id: i32,
    #[sql_type = "Text"]
    pub og_nm: String,
    #[sql_type = "Integer"]
    pub og_default: i32,
    #[sql_type = "Json"]
    pub o: serde_json::Value,
}
