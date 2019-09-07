pub mod device;
pub mod msg;
pub mod option;
pub mod option_group;
pub mod order;
pub mod order_detail;
pub mod product;
pub mod shop;
pub mod user;
pub mod valid;
pub mod fcm;

use actix::{Actor, SyncContext};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct AppStateWithTxt {
    pub websocket: WebSocket,
    pub webpush: WebPush,
    pub valid_email: String,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct WebPush {
    pub send: String,
    pub reg: String,
    pub send_id: String,
    pub key: String,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct WebSocket {
    pub send: String,
}
