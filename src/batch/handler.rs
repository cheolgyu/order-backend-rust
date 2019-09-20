use crate::batch::model::{AutoCancel, AutoCancelRes};
use crate::errors::ServiceError;
use crate::models::{ DbExecutor};
use actix::Handler;
use diesel;

use diesel::prelude::*;
use diesel::sql_query;


impl Handler<AutoCancel> for DbExecutor {
    type Result = Result<Vec<AutoCancelRes>, ServiceError>;

    fn handle(&mut self, msg: AutoCancel, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        let list = sql_query("select * from auto_cancle() ").get_results::<AutoCancelRes>(conn)?;

        Ok(list)
    }
}
