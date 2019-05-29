use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::models::DbExecutor;
use crate::svc::valid::model::{New, Valid};
use crate::utils::hash_password;
use actix::Handler;
use bcrypt::verify;
use diesel;
use diesel::expression::sql_literal::sql;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::{Bool, Integer, Text};
use serde_json::json;
// register/signup user
// handle msg from api::auth.signup
impl Handler<New> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: New, _: &mut Self::Context) -> Self::Result {
        println!("2222222{:?}", msg);
        use crate::schema::valid::dsl::{kind_value, valid as tb};
        let conn = &self.0.get()?;
        let mut check = None;
        let m2 = msg.clone();

        if msg.kind == "phone" {
            println!("44444444{:?}", msg);
            check = tb
                .filter(&kind_value.eq(&msg.kind_value))
                .load::<Valid>(conn)?
                .pop();
        }
        println!("3333333333{:?}", msg);
        match check {
            Some(_) => Err(ServiceError::BadRequest("중복".into())),
            None => {
                let insert: Valid = diesel::insert_into(tb)
                    .values(&m2)
                    .get_result::<Valid>(conn)?;
                let payload = json!({ "valid": insert });

                Ok(Msg {
                    status: 200,
                    data: payload,
                })
            }
        }
    }
}
