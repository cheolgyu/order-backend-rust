use std::ops::Deref;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::model::option::{Opt, CURD};
use uuid::Uuid;
pub type PgPool = Pool<ConnectionManager<PgConnection>>;

use crate::model::db;

pub fn get_all(_shop_id: &Uuid, pool: &PgPool) -> Result<Vec<Opt>, &'static str> {
    Opt::all(&_shop_id, db::get_conn(pool)?.deref()).map_err(|_| "Error inserting Opt")
}

pub fn get_with_id(_id: &i32, pool: &PgPool) -> Result<Opt, &'static str> {
    Opt::get_with_id(&_id, db::get_conn(pool)?.deref()).map_err(|_| "Error inserting Opt")
}

pub fn insert(data: Opt, pool: &PgPool) -> Result<(), &'static str> {
    Opt::insert(data, db::get_conn(pool)?.deref()).map(|_| ()).map_err(|_| "Error inserting Opt")
}

pub fn update(data: Opt, pool: &PgPool) -> Result<(), &'static str> {
    Opt::update(data, db::get_conn(pool)?.deref()).map(|_| ()).map_err(|_| "Error inserting Opt")
}

pub fn delete_with_id(_id: &i32, pool: &PgPool) -> Result<(), &'static str> {
    Opt::delete_with_id(&_id, db::get_conn(pool)?.deref()).map(|_| ()).map_err(|_| "Error inserting Opt")
}
