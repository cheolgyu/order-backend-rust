use crate::models::order as model;
use crate::models::product::CartProduct;
use crate::utils::validator::Validate;
use actix_web::error;
use actix_web::Error;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InpNew {
    pub shop_id: String,
    pub state: i32,
    pub price: f64,
    pub cnt: i32,
    pub products: HashMap<i32, Vec<CartProduct>>,
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
            cnt: self.cnt.clone(),
            products: serde_json::to_value(&self.products)
                .expect("[사용자주문오류]: products serializing "),
            sw_token: self.sw_token.clone(),
        }
    }
}
