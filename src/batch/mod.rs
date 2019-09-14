use actix::prelude::*;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Batch;

impl Batch {
    fn hb(&self, ctx: &mut actix::Context<Self>) {
        ctx.run_later(Duration::new(1, 0), |act, ctx| {
            println!("test");
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
