use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::models::product::Product;
use crate::schema::product;

use crate::utils::validator::{re_test_name, Validate};
use actix::Message;
use actix_web::error;
use actix_web::Error;

use diesel;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Message, Insertable)]
#[rtype(result = "Result<Product, ServiceError>")]
#[table_name = "product"]
pub struct New {
    pub shop_id: Uuid,
    pub name: String,
    pub price: f64,
    pub p_price: f64,
    pub og_price: f64,
    pub opt_group: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InpNew {
    pub name: String,
    pub price: f64,
    pub p_price: f64,
    pub og_price: f64,
    pub opt_group: Vec<i32>,
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
            shop_id: shop_id,
            name: self.name.to_string(),
            price: self.price.clone(),
            p_price: self.p_price.clone(),
            og_price: self.og_price.clone(),
            opt_group: self.opt_group.clone(),
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
    pub p_price: f64,
    pub og_price: f64,
    pub opt_group: Vec<i32>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InpUpdate {
    pub id: i32,
    pub name: String,
    pub price: f64,
    pub p_price: f64,
    pub og_price: f64,
    pub opt_group: Vec<i32>,
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
    pub fn new(&self) -> Update {
        Update {
            id: self.id,
            name: self.name.to_string(),
            price: self.price.clone(),
            p_price: self.p_price.clone(),
            og_price: self.og_price.clone(),
            opt_group: self.opt_group.clone(),
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
    pub shop_id: Uuid,
}
use diesel::sql_types::{ Json,  Uuid as uu};

#[derive(Clone, Debug, Serialize, Deserialize, QueryableByName)]
pub struct ShopInfo {
    #[sql_type = "uu"]
    pub s_id: Uuid,
    #[sql_type = "Json"]
    pub s_info: serde_json::Value,
}

#[derive(Deserialize, Serialize, Debug, Message, Identifiable, AsChangeset)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "product"]
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
