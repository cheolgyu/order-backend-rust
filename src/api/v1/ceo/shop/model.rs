use crate::api::v1::ceo::auth::model::AuthUser;

use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::schema::shop;

use crate::utils::validator::{
    re_test_email, re_test_id, re_test_name, re_test_password, re_test_password_contain_num,
    re_test_password_contain_special, Validate,
};
use actix::Message;
use actix_web::{dev::Payload, Error, HttpRequest};
use actix_web::{error};



use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Message, Insertable)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "shop"]
pub struct NewShop {
    // ... other fields
    pub id: Uuid,
    pub ceo_id: Uuid,
    pub name: String,
    pub products: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InpNew {
    // ... other fields
    pub name: String,
}

impl Validate for InpNew {
    fn validate(&self) -> Result<(), Error> {
        let name = &self.name;
        let check_name = re_test_name(name);

        if check_name {
            Ok(())
        } else {
            Err(error::ErrorBadRequest("shop name"))
        }
    }
}

impl InpNew {
    pub fn new_shop(&self, auth_user: AuthUser) -> NewShop {
        NewShop {
            id: Uuid::new_v4(),
            ceo_id: auth_user.id,
            name: self.name.to_string(),
            products: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Message)]
#[rtype(result = "Result<Msg, ServiceError>")]
pub struct ShopID {
    // ... other fields
    pub id: Uuid,
}
