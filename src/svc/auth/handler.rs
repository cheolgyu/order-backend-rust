use crate::models::DbExecutor;
use crate::svc::auth::model::{hash_password, RegUser, SlimUser, User};
use crate::svc::errors::ServiceError;
use actix::Handler;
use actix::Message;
use actix_web::{error, Error};
use diesel;
use diesel::prelude::*;
use diesel::prelude::*;
use uuid::Uuid;
impl Message for RegUser {
    type Result = Result<User, ServiceError>;
}
// register/signup user
// handle msg from api::auth.signup
impl Handler<RegUser> for DbExecutor {
    type Result = Result<User, ServiceError>;

    fn handle(&mut self, msg: RegUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::user;
        use crate::schema::user::dsl::*;
        let conn = &self.0.get()?;

        let check_user = user
            .filter(&account_id.eq(&msg.login.id))
            .load::<User>(conn)?
            .pop();

        match check_user {
            Some(_) => Err(ServiceError::BadRequest("중복".into())),
            None => {
                // hash password
                let _pswd = hash_password(&msg.login.password)?;
                // generae uuid as user.id
                let _id = msg.login.id;
                let _email = msg.email;
                let new_user = User::new(_id, _pswd, _email);
                let insert_user = diesel::insert_into(user)
                    .values(&new_user)
                    .get_result::<User>(conn)?;

                Ok(insert_user)
            }
        }
    }
}
