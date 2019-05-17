use crate::errors::ServiceError;
use crate::models::DbExecutor;
use crate::svc::option_group::model::{New, OptGroup};
use crate::utils::hash_password;
use actix::Handler;
use actix::Message;
use actix_web::{error, Error};
use bcrypt::verify;
use diesel;
use diesel::prelude::*;
use diesel::prelude::*;
use uuid::Uuid;

impl Handler<New> for DbExecutor {
    type Result = Result<OptGroup, ServiceError>;

    fn handle(&mut self, msg: New, _: &mut Self::Context) -> Self::Result {
        use crate::schema::option_group::dsl::{name, option_group as tb};

        let conn = &self.0.get()?;

        let check = tb.filter(&name.eq(&msg.name)).load::<OptGroup>(conn)?.pop();

        match check {
            Some(_) => Err(ServiceError::BadRequest("중복".into())),
            None => {
                let insert: OptGroup = diesel::insert_into(tb)
                    .values(&msg)
                    .get_result::<OptGroup>(conn)?;

                Ok(insert)
            }
        }
    }
}
