pub mod model;
pub mod handler;
use actix::prelude::*;
use std::time::{Duration, Instant};
use crate::models::DbExecutor;

use crate::batch::model::OrderState;



pub struct Batch{
    db: Addr<DbExecutor>
}

impl Batch {
     pub fn new(
        db: Addr<DbExecutor>
    ) -> Batch {
        Batch {
            db,
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
