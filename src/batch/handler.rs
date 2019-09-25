use crate::batch::model::{AutoCancel, AutoCancelRes, ComeFind, ComeFindRes };
use crate::errors::ServiceError;
use crate::models::DbExecutor;
use actix::Handler;
use diesel;

use diesel::prelude::*;
use diesel::sql_query;

impl Handler<ComeFind> for DbExecutor {
    type Result = Result<Vec<ComeFindRes>, ServiceError>;

    fn handle(&mut self, msg: ComeFind, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        let list = sql_query("select * from view_comfind_info ").get_results::<ComeFindRes>(conn)?;

        Ok(list)
    }
}

impl Handler<AutoCancel> for DbExecutor {
    type Result = Result<Vec<AutoCancelRes>, ServiceError>;

    fn handle(&mut self, msg: AutoCancel, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        let list = sql_query("select * from auto_cancle() ").get_results::<AutoCancelRes>(conn)?;

        Ok(list)
    }
}
