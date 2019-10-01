use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::schema::order;

use crate::utils::validator::Validate;
use actix::Message;
use actix_web::error;
use actix_web::Error;

use diesel;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Message, Identifiable, AsChangeset)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "order"]
pub struct Update {
    pub id: i32,
    pub shop_id: Uuid,
    pub state: i32,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InpUpdate {
    pub id: i32,
    pub shop_id: Uuid,
    pub state: i32,
}

impl Validate for InpUpdate {
    fn validate(&self) -> Result<(), Error> {
        let check_name = true;

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
            state: self.state.clone(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Message, Identifiable)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "order"]
pub struct Get {
    pub id: i32,
    pub shop_id: Uuid,
}

#[derive(Deserialize, Serialize, Debug, Message)]
#[rtype(result = "Result<Msg, ServiceError>")]
pub struct GetList {
    pub shop_id: Uuid,
}
use diesel::sql_types::{Integer, Json, Text, Uuid as uu};
#[derive(Clone, Debug, Serialize, Deserialize, QueryableByName)]
pub struct Simple {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "uu"]
    pub shop_id: Uuid,
    #[sql_type = "Text"]
    pub name: String,
    #[sql_type = "Integer"]
    pub default: i32,
    #[sql_type = "Json"]
    pub option_list: serde_json::Value,
}

#[derive(Deserialize, Serialize, Debug, Message)]
#[rtype(result = "Result<Msg, ServiceError>")]
pub struct NowList {
    pub shop_id: Uuid,
}
