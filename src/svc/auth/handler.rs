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
    type Result = Result<usize, ServiceError>;

    fn handle(&mut self, msg: Info, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;
        println!(" path info start ");
        match msg.auth_user {
            None => Err(ServiceError::Unauthorized),
            Some(u) => {
                use diesel::sql_types::Nullable;
                println!(" path info start match ");
                if u.role == "ceo" {
                    let q = sql_query("select * from ceo_info($1,$2,$3) ");
                    let res = q
                        .bind::<Text, _>(u.id.to_string())
                        .bind::<Nullable<Text>, _>(msg.shop_id)
                        .bind::<Nullable<Integer>, _>(msg.product_id)
                        .execute(conn)?;
                    let res2 = res;
                    println!("=====path info==>{:?}", res2);

                    if res2 == 1 {
                        let res3 = res2 as usize;
                        Ok(res3)
                    } else {
                        Err(ServiceError::Unauthorized)
                    }
                } else if u.role == "super" {
                    Ok(1)
                } else {
                    Err(ServiceError::BadRequest("누구냐".into()))
                }
            }
        }
    }
}

/*
impl Handler<Info> for DbExecutor {
    type Result = Result<usize, ServiceError>;

    fn handle(&mut self, msg: Info, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;
        println!(" path info start ");
        match msg.auth_user {
            None => Err(ServiceError::Unauthorized),
            Some(u) => {
                println!(" path info start match ");
                if u.role == "ceo" {
                    let q = "select * from ceo_info('".to_string()
                        + &u.id.to_string()
                        + "','"
                        + &msg.shop_id.unwrap()
                        + "',"
                        + match &msg.product_id {
                            Some(res) => &res.to_string(),
                            None() => "",
                        }
                        + ")";
                    let res = sql_query(q).execute(conn)?;
                    if res == 1 {
                        Ok(res)
                    } else {
                        Err(ServiceError::Unauthorized)
                    }
                } else if u.role == "super" {
                    Ok(1)
                } else {
                    Err(ServiceError::BadRequest("누구냐".into()))
                }
            }
        }
    }
}
*/
