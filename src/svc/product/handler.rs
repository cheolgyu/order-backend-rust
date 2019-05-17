use crate::errors::ServiceError;
use crate::models::DbExecutor;
use crate::svc::product::model::{InpNew, New, Product};
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
    type Result = Result<Product, ServiceError>;

    fn handle(&mut self, msg: New, _: &mut Self::Context) -> Self::Result {
        use crate::schema::product::dsl::{name, product as tb};
        let conn = &self.0.get()?;

        let check = tb.filter(&name.eq(&msg.name)).load::<Product>(conn)?.pop();

        match check {
            Some(_) => Err(ServiceError::BadRequest("중복".into())),
            None => {
                let insert: Product = diesel::insert_into(tb)
                    .values(&msg)
                    .get_result::<Product>(conn)?;

                Ok(insert)
            }
        }
    }
}
