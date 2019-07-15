pub mod msg;
pub mod option;
pub mod option_group;
pub mod product;
pub mod shop;
pub mod user;
pub mod valid;
pub mod ws;
pub mod db;

use actix::{Actor, SyncContext};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}
