use crate::api::v1::ceo::auth::model::LoginUser;

use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::schema::shop;

use crate::utils::validator::{re_test_name, Validate};
use actix::Message;
use actix_web::error;
use actix_web::Error;

use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Message, Insertable)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "shop"]
pub struct NewShop {
    pub ceo_id: Uuid,
    pub name: String,
    pub products: Option<serde_json::Value>,
}
/*
impl From<InpNew> for NewShop{
    fn from(inp: InpNew) -> Self {
        NewShop { ceo_id: u_id.clone(),name: inp.name.clone(), products: None}
    }
}
*/
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
    pub fn new_shop(&self, login_user: LoginUser) -> NewShop {
        NewShop {
            ceo_id: login_user.id,
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

#[derive(Deserialize, Serialize, Debug, Message, Insertable, AsChangeset)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "shop"]
pub struct UpdateShop {
    // ... other fields
    pub id: Uuid,
    pub ceo_id: Uuid,
    pub name: String,
    pub products: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InpUpdate {
    // ... other fields
    pub name: String,
}

impl Validate for InpUpdate {
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

impl InpUpdate {
    pub fn update_shop(&self, login_user: LoginUser) -> UpdateShop {
        UpdateShop {
            id: Uuid::new_v4(),
            ceo_id: login_user.id,
            name: self.name.to_string(),
            products: None,
        }
    }
}
