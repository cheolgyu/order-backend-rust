use crate::models::DbExecutor;
use crate::errors::ServiceError;
use actix::Handler;
use crate::batch::model::{OrderState,OrderStateRes};
use diesel;

use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::{Integer, Text};

impl Handler<OrderState> for DbExecutor {
    type Result = Result<Vec<OrderStateRes>, ServiceError>;

    fn handle(&mut self, msg: OrderState, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get().expect("batch_db conn err");
      //  println!(" db"); 
        let res  = sql_query("select * from order_state() ").get_results::<OrderStateRes>(conn)?;
       // println!(" db : {:?}",res); 
        Ok(res)
    }
}
