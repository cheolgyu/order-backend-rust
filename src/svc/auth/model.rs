use crate::schema::user;
use crate::svc::errors::ServiceError;
use crate::svc::validator::{
    re_test_email, re_test_id, re_test_password, re_test_password_contain_num,
    re_test_password_contain_special, Validate,
};
use actix::Message;
use actix_web::{error, Error};
use bcrypt::{hash, DEFAULT_COST};
use chrono::{Duration, Local, NaiveDateTime, Utc};
use diesel;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Identifiable, Queryable, Insertable)]
#[table_name = "user"]
pub struct User {
    pub id: i32,
    pub account_id: String,
    pub account_password: String,
    pub email: String,
    pub name: String,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

pub struct SlimUser {
    pub id: i32,
    pub account_id: String,
    pub email: String,
    pub name: String,
    pub role: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser {
            id: user.id,
            account_id: user.account_id,
            email: user.email,
            name: user.name,
            role: user.role,
        }
    }
}

impl User {
    pub fn new(account_id: String, account_password: String, email: String) -> Self {
        User {
            id: 0,
            account_id: account_id,
            account_password: account_password,
            email: email,
            role: "ceo".to_owned(),
            name: "".to_owned(),
            created_at: Utc::now().naive_utc(),
            updated_at: None,
            deleted_at: None,
        }
    }
}

pub fn hash_password(plain: &str) -> Result<String, ServiceError> {
    // get the hashing cost from the env variable or use default
    let hashing_cost: u32 = match dotenv::var("HASH_ROUNDS") {
        Ok(cost) => cost.parse().unwrap_or(DEFAULT_COST),
        _ => DEFAULT_COST,
    };
    hash(plain, hashing_cost).map_err(|_| ServiceError::InternalServerError)
}

#[derive(Deserialize, Debug)]
pub struct Login {
    pub id: String,
    pub password: String,
}

impl Validate for Login {
    fn validate(&self) -> Result<(), Error> {
        let id = &self.id;
        let psw = &self.password;
        //let check = re_test_name(uname) && re_test_psw(psw);
        let check_id = re_test_id(id);

        let check_pwd = re_test_password(psw);
        let check_pwd_special = re_test_password_contain_special(psw);
        let check_pwd_num = re_test_password_contain_num(psw);
        if check_id {
            if check_pwd {
                if check_pwd_special {
                    if check_pwd_num {
                        Ok(())
                    } else {
                        Err(error::ErrorBadRequest("check_pwd_num"))
                    }
                } else {
                    // 특수문자 미포함
                    Err(error::ErrorBadRequest("check_pwd_special"))
                }
            } else {
                //자리수,첫번째 소,대문자
                Err(error::ErrorBadRequest("check_pwd"))
            }
        } else {
            Err(error::ErrorBadRequest("Invalid id"))
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct RegUser {
    pub login: Login,
    pub password_comfirm: String,
    pub email: String,
}

impl Validate for RegUser {
    fn validate(&self) -> Result<(), Error> {
        let password_comfirm = &self.password_comfirm;
        let login = &self.login;
        let email = &self.email;

        let check_email = re_test_email(email);

        if password_comfirm.trim() != login.password {
            Err(error::ErrorBadRequest("pwd "))
        } else {
            if check_email {
                Ok(())
            } else {
                Err(error::ErrorBadRequest("email "))
            }
        }
    }
}
