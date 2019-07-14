use crate::api::v1::ceo::valid::model::{ChkValid, New, Valid};
use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::models::DbExecutor;
use actix::Handler;

use diesel;

use diesel::prelude::*;

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

impl Handler<ChkValid> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: ChkValid, _: &mut Self::Context) -> Self::Result {
        println!("2222222{:?}", msg);
        use crate::schema::valid::dsl::{code, kind, kind_value, res, user_id, valid as tb};
        let conn = &self.0.get()?;

        let check = tb
            .filter(&kind_value.eq(&msg.v.kind_value))
            .filter(&kind.eq(&msg.v.kind))
            .filter(&user_id.eq(&msg.v.user_id))
            .filter(&code.eq(&msg.code))
            .load::<Valid>(conn)?
            .pop();

        println!("3333333333{:?}", msg);
        match check {
            Some(_target) => {
                use crate::schema::user::dsl::{id, user as u_tb, valid_email};
                diesel::update(u_tb)
                    .filter(&id.eq(&msg.v.user_id))
                    .set(valid_email.eq(true))
                    .execute(conn)?;

                let update: Valid = diesel::update(tb)
                    .filter(&kind_value.eq(&msg.v.kind_value))
                    .filter(&kind.eq(&msg.v.kind))
                    .filter(&user_id.eq(&msg.v.user_id))
                    .filter(&code.eq(&msg.code))
                    .set(res.eq("true"))
                    .get_result::<Valid>(conn)?;

                let payload = json!({ "valid": update });

                Ok(Msg {
                    status: 200,
                    data: payload,
                })
            }
            None => Err(ServiceError::BadRequest("인증 확인하세요.".into())),
        }
    }
}
