use crate::errors::ServiceError;
use crate::models::msg::Msg;
use actix::Message;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Message)]
#[rtype(result = "Result<Msg, ServiceError>")]
pub struct GetWithId {
    pub id: Uuid,
}

#[derive(Deserialize, Serialize, Debug, Message)]
#[rtype(result = "Result<Msg, ServiceError>")]
pub struct GetList {}
