use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::models::order as model;
use crate::schema::order;
use crate::utils::validator::{re_test_name, Validate};
use actix::Message;
use actix_web::error;
use actix_web::Error;

use chrono::NaiveDateTime;
use diesel;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InpNew {
    pub shop_id: String,
    pub state: String,
    pub price: f64,
    pub products: serde_json::Value,
    pub sw_token: String,
}

impl Validate for InpNew {
    fn validate(&self) -> Result<(), Error> {
        let check_name = true;

        if check_name {
            Ok(())
        } else {
            Err(error::ErrorBadRequest("shop name"))
        }
    }
}

impl InpNew {
    pub fn new(&self) -> model::New {
        model::New {
            shop_id: Uuid::parse_str(&self.shop_id).unwrap(),
            state: self.state.clone(),
            price: self.price.clone(),
            products: self.products.clone(),
            sw_token: self.sw_token.clone(),
        }
    }
}
