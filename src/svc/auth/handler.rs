use crate::errors::ServiceError;
use crate::models::DbExecutor;
use crate::svc::auth::model::{Login, QueryUser, RegUser, SlimUser, User};
use crate::utils::hash_password;
use actix::Handler;
use bcrypt::verify;
use diesel;
use diesel::prelude::*;
// register/signup user
// handle msg from api::auth.signup
impl Handler<RegUser> for DbExecutor {
    type Result = Result<User, ServiceError>;

    fn handle(&mut self, msg: RegUser, _: &mut Self::Context) -> Self::Result {
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
                let insert_user: User = diesel::insert_into(user)
                    .values(&new_user)
                    .get_result::<User>(conn)?;

                Ok(insert_user)
            }
        }
    }
}

impl Handler<Login> for DbExecutor {
    type Result = Result<SlimUser, ServiceError>;

    fn handle(&mut self, msg: Login, _: &mut Self::Context) -> Self::Result {
        use crate::schema::user::dsl::{account_id, user};
        let conn = &self.0.get()?;

        let query_user = user
            .filter(&account_id.eq(&msg.id))
            .load::<User>(conn)?
            .pop();
        let debug = format!("{:?}", query_user);
        println!("{:?}", debug);

        if let Some(check_user) = query_user {
            match verify(&msg.password, &check_user.account_password) {
                Ok(valid) if valid => {
                    return Ok(check_user.into());
                }
                _ => (),
            }
        }
        Err(ServiceError::BadRequest("Auth Failed".into()))
    }
}

impl Handler<QueryUser> for DbExecutor {
    type Result = Result<SlimUser, ServiceError>;

    fn handle(&mut self, uid: QueryUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::user::dsl::*;
        let conn = &self.0.get()?;

        let query_user = user.filter(&id.eq(&uid.id)).get_result::<User>(conn)?;

        Ok(query_user.into())
    }
}
