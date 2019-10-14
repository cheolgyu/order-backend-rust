use crate::api::v1::ceo::auth::model::AuthUser;

use crate::models::device as m;

use crate::utils::validator::{re_test_name, Validate};
use actix_web::error;
use actix_web::Error;

use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct InpCheck {
    pub sw_token: String,
}

impl Validate for InpCheck {
    fn validate(&self) -> Result<(), Error> {
        let check_name = true;

        if check_name {
            Ok(())
        } else {
            Err(error::ErrorBadRequest("device name"))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InpNew {
    pub name: String,
    pub sw_token: String,
}

impl Validate for InpNew {
    fn validate(&self) -> Result<(), Error> {
        let name = &self.name;
        let check_name = re_test_name(name);

        if check_name {
            Ok(())
        } else {
            Err(error::ErrorBadRequest("device name"))
        }
    }
}

impl InpNew {
    pub fn new(&self, auth_user: AuthUser) -> m::New {
        m::New {
            user_id: auth_user.id,
            name: self.name.clone(),
            sw_token: self.sw_token.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InpGetList {
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InpUpdate {
    pub id: i32,
    pub name: String,
    pub sw_token: String,
}

impl Validate for InpUpdate {
    fn validate(&self) -> Result<(), Error> {
        let name = &self.name;
        let check_name = re_test_name(name);

        if check_name {
            Ok(())
        } else {
            Err(error::ErrorBadRequest("device InpUpdate name"))
        }
    }
}
