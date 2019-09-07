use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::schema::order_detail;

use crate::utils::validator::{ Validate};
use actix::Message;
use actix_web::error;
use actix_web::Error;

use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Message, Queryable, Insertable)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "order_detail"]
pub struct New {
    pub shop_id: Uuid,
    pub order_id: i32,
    pub state: String,
    pub txt: serde_json::Value,
    pub req_session_id: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InpNew {
    pub shop_id: Uuid,
    pub order_id: i32,
    pub state: String,
    pub txt: serde_json::Value,
    pub req_session_id: serde_json::Value,
}

impl Validate for InpNew {
    fn validate(&self) -> Result<(), Error> {
        let check_name = true;

        if check_name {
            Ok(())
        } else {
            Err(error::ErrorBadRequest("option name"))
        }
    }
}

impl InpNew {
    pub fn new(&self, shop_id: String) -> New {
        New {
            shop_id: Uuid::parse_str(&shop_id).unwrap(),
            order_id: self.order_id.clone(),
            state: self.state.clone(),
            txt: self.txt.clone(),
            req_session_id: self.req_session_id.clone(),
        }
    }
}
