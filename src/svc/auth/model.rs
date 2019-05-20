use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::schema::user;
use crate::utils::hash_password;
use crate::utils::jwt::decode_token;
use crate::utils::validator::{
    re_test_email, re_test_id, re_test_password, re_test_password_contain_num,
    re_test_password_contain_special, Validate,
};
use actix::Message;
use actix_web::{dev::Payload, Error, HttpRequest};
use actix_web::{error, middleware::identity::Identity, FromRequest};
use bcrypt::{hash, DEFAULT_COST};
use chrono::{Duration, Local, NaiveDateTime, Utc};
use diesel;
use uuid::Uuid;
#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Identifiable,
    Queryable,
    Insertable,
    QueryableByName,
)]
#[table_name = "user"]
pub struct User {
    pub id: Uuid,
    pub account_id: String,
    pub account_password: String,
    pub email: String,
    pub name: String,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub id: Uuid,
    pub account_id: String,
    pub email: String,
    pub name: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: Uuid,
    pub role: String,
}
impl AuthUser {
    pub fn check_user(&self, path_id: String) -> Result<(), Error> {
        println!("check_user==>");
        if &self.role == "ceo" {
            if path_id == self.id.to_string() {
                Ok(())
            } else {
                Err(error::ErrorUnauthorized(
                    "본인 계정만 이용 가능합니다.",
                ))
            }
        } else {
            Err(error::ErrorUnauthorized("ceo가 아닌계정이군"))
        }
    }
    pub fn role_user(&self, path_id: String) -> bool {
        !(&self.role == "ceo" && path_id == self.id.to_string())
    }
}

impl FromRequest for AuthUser {
    type Config = ();
    type Error = Error;
    type Future = Result<AuthUser, Error>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        if let Some(auth_token) = req.headers().get("authorization") {
            if let Ok(auth) = auth_token.to_str() {
                let user: AuthUser = decode_token(auth)?;

                return Ok(user);
            }
        }
        Err(ServiceError::Unauthorized.into())
    }
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

#[derive(Deserialize, Serialize, Debug, Message)]
#[rtype(result = "Result<SlimUser, ServiceError>")]
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

#[derive(Deserialize, Serialize, Debug)]
pub struct InpNew {
    pub login: Login,
    pub password_comfirm: String,
    pub email: String,
}

#[derive(Deserialize, Serialize, Debug, Message, Insertable)]
#[rtype(result = "Result<Msg, ServiceError>")]
#[table_name = "user"]
pub struct New {
    pub account_id: String,
    pub account_password: String,
    pub email: String,
    pub name: String,
    pub role: String,
}

impl InpNew {
    pub fn new(&self) -> New {
        New {
            account_id: self.login.id.to_string(),
            account_password: self.login.password.to_string(), //hash_password(&self.login.password).unwrap(),
            email: self.email.to_string(),
            role: "ceo".to_string(),
            name: "".to_string(),
        }
    }
}

impl Validate for InpNew {
    fn validate(&self) -> Result<(), Error> {
        let password_comfirm = &self.password_comfirm;
        let login = &self.login;
        let email = &self.email;
        let check_email = re_test_email(email);
        match login.validate() {
            Ok(_) => {
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
            Err(e) => Err(e),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Message)]
#[rtype(result = "Result<SlimUser, ServiceError>")]
pub struct QueryUser {
    pub id: Uuid,
}
