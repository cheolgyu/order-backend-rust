pub mod model;
pub mod handler;

use actix::prelude::*;
use std::time::{Duration, Instant};
use crate::models::{AppStateWithTxt, DbExecutor};
use crate::batch::model::OrderState;
use crate::api::v1::ceo::fcm::router as fcm;
use actix_web::{
    client::Client,
    web::{Data, Json},
    };


pub struct Batch{
    db: Addr<DbExecutor>,
    client: Client,
    store: AppStateWithTxt,
}

impl Batch {
     pub fn new(
        db: Addr<DbExecutor>,
        client: Client,
        store: AppStateWithTxt,
    ) -> Batch {
        Batch {
            db,
            client,
            store
        }
    }

    fn hb(&self, ctx: &mut actix::Context<Self>) {
        &self.db.do_send(OrderState{});
        ctx.run_later(Duration::new(1, 0), |act, ctx| {
         //   println!("batch:");
            act.hb(ctx);
        });
    }
}

impl Actor for Batch {
    type Context = Context<Self>;

     fn started(&mut self, ctx: &mut Self::Context) {
       println!("I am alive!");
       self.hb(ctx);
    }

}
