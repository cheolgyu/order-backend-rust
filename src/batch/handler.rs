use crate::batch::model::{AutoCancel, AutoCancelRes};
use crate::errors::ServiceError;
use crate::models::{AppStateWithTxt, DbExecutor};
use actix::Handler;
use diesel;

use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::{Integer, Text};

use crate::api::v1::ceo::fcm::router as fcm;
use crate::models::fcm::{Notification, ParamsNotification, ParamsToUser, ToUserResp};
use actix_web::{client::Client, web};
use futures::future::Future;

impl Handler<AutoCancel> for DbExecutor {
    type Result = Result<Vec<AutoCancelRes>, ServiceError>;

    fn handle(&mut self, msg: AutoCancel, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        let list = sql_query("select * from order_state() ").get_results::<AutoCancelRes>(conn)?;

        Ok(list)
    }
}
