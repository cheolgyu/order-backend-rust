use crate::api::v1::ceo::auth::model::{Login, New, QueryUser, SlimUser, User};
use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::models::shop::Shop;
use crate::models::DbExecutor;
use actix::Handler;

use diesel;

use diesel::prelude::*;
use diesel::sql_query;
use serde_json::json;
// register/signup user
// handle msg from api::auth.signup
impl Handler<New> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: New, _: &mut Self::Context) -> Self::Result {
        use crate::schema::user::dsl::*;
        let conn = &self.0.get()?;

        let check_user = user
            .filter(&account_id.eq(&msg.account_id))
            .load::<User>(conn)?
            .pop();

        match check_user {
            Some(_) => Err(ServiceError::BadRequest("중복".into())),
            None => {
                let  s = r#"INSERT INTO "user" ( account_id,account_password, email, name, role) VALUES  "#;
                let s2 = s.to_string()
                    + "("
                    + "'"
                    + &msg.account_id
                    + "'"
                    + ","
                    + "crypt("
                    + "'"
                    + &msg.account_password
                    + "'"
                    + ", gen_salt('bf'))"
                    + ","
                    + "'"
                    + &msg.email
                    + "'"
                    + ","
                    + "'"
                    + &msg.name
                    + "'"
                    + ","
                    + "'"
                    + &msg.role
                    + "'"
                    + ")";
                let res = sql_query(s2).execute(conn)?;

                let payload = json!({
                   "res": res,
                });

                Ok(Msg {
                    status: 200,
                    data: payload,
                })
            }
        }
    }
}

impl Handler<Login> for DbExecutor {
    type Result = Result<SlimUser, ServiceError>;

    fn handle(&mut self, msg: Login, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        let s = r#"SELECT * FROM "user" WHERE  account_password =  "#;
        let s2 = s.to_string() + "crypt(" + "'" + &msg.password + "'" + ", account_password)";
        let s2 = s2.to_string() + " AND account_id = " + "'" + &msg.id + "'";
        let res: Option<User> = sql_query(s2).load::<User>(conn)?.pop();

        match res {
            Some(u) => Ok(u.into()),
            None => Err(ServiceError::BadRequest("누구냐".into())),
        }
    }
}

impl Handler<QueryUser> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, uid: QueryUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::shop::dsl::{ceo_id, shop as s_tb};
        use crate::schema::user::dsl::*;
        let conn = &self.0.get()?;

        let query_user = user.filter(&id.eq(&uid.id)).get_result::<User>(conn)?;
        let query_shop = s_tb.filter(&ceo_id.eq(&uid.id)).get_result::<Shop>(conn);

        let shop_info: Option<Shop> = match query_shop {
            Ok(s) => Some(s),
            Err(_e) => None,
        };

        let payload = json!({
           "user": query_user,
           "shop": shop_info,
        });

        Ok(Msg {
            status: 200,
            data: payload,
        })
    }
}
