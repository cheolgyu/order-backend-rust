pub mod msg;
pub mod option;
pub mod option_group;
pub mod order;
pub mod order_detail;
pub mod product;
pub mod shop;
pub mod user;
pub mod valid;
pub mod device;


use actix::{Actor, SyncContext};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

#[derive(Clone, Debug)]
pub struct AppStateWithTxt {
    pub websocket_url: String,
    pub webpush_url: String,
    pub webpush_key: String,
    pub valid_email: String,
}
