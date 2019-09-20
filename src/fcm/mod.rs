pub mod handler;
pub mod model;
pub mod db;
//pub mod router;


use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::models::{AppStateWithTxt, DbExecutor};
use actix::prelude::*;
use futures::stream::Stream;
use futures::Future;
use std::time::{Duration, Instant};

use actix_web::{
    client::Client,
    web::{self, Data, Json},
    Error, HttpResponse, ResponseError,
};
pub struct FcmExecutor {
    pub db: Data<Addr<DbExecutor>>,
    pub store: Data<AppStateWithTxt>,
}
/*

*/
impl FcmExecutor {
    pub fn new(db: Data<Addr<DbExecutor>>, store: Data<AppStateWithTxt>) -> FcmExecutor {
        FcmExecutor { db, store }
    }
}

impl Actor for FcmExecutor {
    type Context = SyncContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("I am FcmExecutor!"); 
    }
}
