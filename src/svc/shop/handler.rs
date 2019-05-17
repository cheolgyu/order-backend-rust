use crate::errors::ServiceError;
use crate::models::DbExecutor;
use crate::svc::shop::model::{NewShop, Shop};
use crate::utils::hash_password;
use actix::Handler;
use actix::Message;
use actix_web::{error, Error};
use bcrypt::verify;
use diesel;
use diesel::prelude::*;
use diesel::prelude::*;
use uuid::Uuid;

impl Handler<NewShop> for DbExecutor {
    type Result = Result<Shop, ServiceError>;

    fn handle(&mut self, msg: NewShop, _: &mut Self::Context) -> Self::Result {
        use crate::schema::shop;
        use crate::schema::shop::dsl::*;
        let conn = &self.0.get()?;

        let check_user = shop
            .filter(&ceo_id.eq(&msg.ceo_id))
            .load::<Shop>(conn)?
            .pop();

        match check_user {
            Some(_) => Err(ServiceError::BadRequest("중복".into())),
            None => {
                let insert: Shop = diesel::insert_into(shop)
                    .values(&msg)
                    .get_result::<Shop>(conn)?;

                Ok(insert)
            }
        }
    }
}
