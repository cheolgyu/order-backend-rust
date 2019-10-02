use crate::models::order as model;
use crate::models::product::CartProduct;
use crate::utils::validator::Validate;
use actix_web::error;
use actix_web::Error;

use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InpNew {
    pub shop_id: String,
    pub state: i32,
    pub price: f64,
    pub products: Vec<CartProduct>,
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
            products: serde_json::to_value(&self.products)
                .expect("[사용자주문오류]: products serializing "),
            sw_token: self.sw_token.clone(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt::System;
    use futures::future;

    #[test]
    fn test_date_len() {
        assert_eq!(DATE_VALUE_LENGTH, "Sun, 06 Nov 1994 08:49:37 GMT".len());
    }

    #[test]
    fn test_date() {
        let mut rt = System::new("test");

        let _ = rt.block_on(future::lazy(|| {
            let settings = ServiceConfig::new(KeepAlive::Os, 0, 0);
            let mut buf1 = BytesMut::with_capacity(DATE_VALUE_LENGTH + 10);
            settings.set_date(&mut buf1);
            let mut buf2 = BytesMut::with_capacity(DATE_VALUE_LENGTH + 10);
            settings.set_date(&mut buf2);
            assert_eq!(buf1, buf2);
            future::ok::<_, ()>(())
        }));
    }
}
