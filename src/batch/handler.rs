use crate::models::DbExecutor;
use actix::Handler;
use crate::batch::model::OrderState;
use diesel;

use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::{Integer, Text};

impl Handler<OrderState> for DbExecutor {
    type Result = ();

    fn handle(&mut self,1 msg: OrderState, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get().expect("batch_db conn err");
      //  println!(" db"); 
        let res = sql_query("select * from order_state() ").execute(conn).expect("batch function error");
        println!(" db : {:?}",res); 
                         
    }
}
