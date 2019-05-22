use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::models::DbExecutor;
use crate::svc::auth::model::{Ceo, Info, Login, New, QueryUser, SlimUser, User};
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
        use crate::schema::user::dsl::*;
        let conn = &self.0.get()?;

        let check_user = user
            .filter(&account_id.eq(&msg.account_id))
            .load::<User>(conn)?
            .pop();

        match check_user {
            Some(_) => Err(ServiceError::BadRequest("중복".into())),
            None => {
                let mut s = r#"INSERT INTO "user" ( account_id,account_password, email, name, role) VALUES  "#;
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
        use crate::schema::user::dsl::{account_id, user};
        let conn = &self.0.get()?;

        let mut s = r#"SELECT * FROM "user" WHERE  account_password =  "#;
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
    type Result = Result<SlimUser, ServiceError>;

    fn handle(&mut self, uid: QueryUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::user::dsl::*;
        let conn = &self.0.get()?;

        let query_user = user.filter(&id.eq(&uid.id)).get_result::<User>(conn)?;

        Ok(query_user.into())
    }
}

impl Handler<Info> for DbExecutor {
    type Result = Result<Ceo, ServiceError>;

    fn handle(&mut self, msg: Info, _: &mut Self::Context) -> Self::Result {
        println!("==========================================================================================");
        use crate::schema::user::dsl::*;
        use uuid::Uuid;
        let conn = &self.0.get().expect(" 커넥션 오류 ");
        let uid = Uuid::parse_str(&msg.user_id).unwrap();
        let query_user = user
            .filter(&id.eq(&uid))
            .get_result::<User>(conn)
            .expect(" 조회 오류 ");
        let mut ceo = Ceo {
            user: query_user,
            shop: None,
            product: None,
        };

        match msg.shop_id {
            None => {}
            Some(sid_str) => {
                let sid = Uuid::parse_str(&sid_str)?;
                use crate::schema::shop::dsl::{id, shop as tb};
                use crate::svc::shop::model::Shop;
                //shop id 의 소유확인
                let query_shop = tb.filter(&id.eq(&sid)).get_result::<Shop>(conn)?;
                ceo.shop = Some(query_shop);
                match msg.product_id {
                    None => {}
                    Some(pid) => {
                        //product_id 의 소유확인
                        use crate::schema::product::dsl::{id, product as tb};
                        use crate::svc::product::model::Product;
                        let n: i32 = std::str::FromStr::from_str(&pid).unwrap();
                        let query_product = tb.filter(&id.eq(&n)).get_result::<Product>(conn)?;
                        ceo.product = Some(query_product);
                    }
                }
            }
        }

        println!("{:?}", ceo);
        println!("==========================================================================================");
        Ok(ceo)
    }
}
