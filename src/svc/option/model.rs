use crate::errors::ServiceError;
use crate::schema::option;
use crate::svc::auth::model::AuthUser;
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Identifiable, Queryable, Insertable)]
#[table_name = "option"]
pub struct Opt {
    pub id: Uuid,
    pub name: String,

    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize, Debug, Message, Insertable)]
#[rtype(result = "Result<Opt, ServiceError>")]
#[table_name = "option"]
pub struct New {
    // ... other fields
    pub id: Uuid,
    pub name: String,
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
            Err(error::ErrorBadRequest("option name"))
        }
    }
}

impl InpNew {
    pub fn new(&self) -> New {
        New {
            id: Uuid::new_v4(),
            name: self.name.to_string(),
        }
    }
}
