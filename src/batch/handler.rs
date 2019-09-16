use crate::models::{AppStateWithTxt, DbExecutor};
use crate::errors::ServiceError;
use actix::Handler;
use crate::batch::model::{OrderState,OrderStateRes};
use diesel;

use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::{Integer, Text};

use crate::api::v1::ceo::fcm::router as fcm;
use crate::models::fcm::{ParamsToUser,ParamsNotification, Notification, ToUserResp};
use actix_web::{
    web,
    client::Client,
  };
use futures::future::Future;

impl Handler<OrderState> for DbExecutor {
    type Result = Result<Vec<OrderStateRes>, ServiceError>;

    fn handle(&mut self, msg: OrderState, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;
      //  println!(" db"); 
        let list  = sql_query("select * from order_state() ").get_results::<OrderStateRes>(conn)?;
       println!(" db handler: {:?}",list.len()); 
        
        Ok(list)
    }
}
