use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::models::product::Product;
use crate::models::shop::Shop;

use crate::schema::user;
use crate::utils::validator::{
    re_test_email, re_test_id, re_test_password, re_test_password_contain_num,
    re_test_password_contain_special, Validate,
};

use actix::Message;
use actix_web::{dev::Payload, Error, HttpRequest};
use actix_web::{error, http::header::HeaderMap, FromRequest};

use chrono::NaiveDateTime;

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
    pub valid_email: bool,
    pub phone: Option<String>,
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
    pub valid_email: bool,
    pub name: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReqInfo {
    pub auth_id: String,
    pub auth_role: String,
    pub req_u_id: String,
    pub req_s_id: String,
    pub req_target_type: String,
    pub req_target_id: String,
}

impl ReqInfo {
    pub fn auth_id(&self) -> Uuid {
        Uuid::parse_str(&self.auth_id).unwrap()
    }
    pub fn auth_role(&self) -> String {
        self.auth_role.clone()
    }
    pub fn req_u_id(&self) -> Uuid {
        if &self.req_u_id == "" {
            println!("================req_u_id==========:{:?}", &self.req_u_id);
        }

        Uuid::parse_str(&self.req_u_id).unwrap()
    }
    pub fn req_s_id(&self) -> Uuid {
        Uuid::parse_str(&self.req_s_id).unwrap()
    }
    pub fn req_target_type(&self) -> String {
        self.req_target_type.clone()
    }
    pub fn req_target_id(&self) -> i32 {
        self.req_target_type.clone().parse::<i32>().unwrap()
    }
}

fn get_header_value(hm: &HeaderMap, key: String) -> String {
    let res = hm
        .get(&key)
        .expect(&format!("req get {:?} null", key))
        .to_str()
        .expect(&format!("req  to_str {:?} null", key));

    res.to_string()
}

impl FromRequest for ReqInfo {
    type Config = ();
    type Error = Error;
    type Future = Result<ReqInfo, Error>;

    fn from_request(req: &HttpRequest, _pl: &mut Payload) -> Self::Future {
        let hm = req.headers().to_owned();
        let key = vec![
            "auth_id",
            "auth_role",
            "req_u_id",
            "req_s_id",
            "req_tg_type",
            "req_tg_id",
        ];
        let mut val = Vec::new();

        for x in &key {
            val.push(get_header_value(&hm, x.to_string()));
        }

        Ok(ReqInfo {
            auth_id: val[0].clone(),
            auth_role: val[1].clone(),
            req_u_id: val[2].clone(),
            req_s_id: val[3].clone(),
            req_target_type: val[4].clone(),
            req_target_id: val[5].clone(),
        })
    }
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser {
            id: user.id,
            account_id: user.account_id,
            email: user.email,
            valid_email: user.valid_email,
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
                        Err(error::ErrorBadRequest("resp.signup.check.pwd.number"))
                    }
                } else {
                    // 특수문자 미포함
                    Err(error::ErrorBadRequest("resp.signup.check.pwd.special"))
                }
            } else {
                //자리수,첫번째 소,대문자
                Err(error::ErrorBadRequest("resp.signup.check.pwd.len_en"))
            }
        } else {
            Err(error::ErrorBadRequest("resp.signup.check.id.len_en"))
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
                    Err(error::ErrorBadRequest(" 비밀번호가 일치하지 않습니다. "))
                } else {
                    if check_email {
                        Ok(())
                    } else {
                        Err(error::ErrorBadRequest(" 이메일 형식이 아닙니다. "))
                    }
                }
            }
            Err(e) => Err(e),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Message, Queryable)]
#[rtype(result = "Result<Msg, ServiceError>")]
pub struct QueryUser {
    pub id: Uuid,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Ceo {
    pub user: Option<User>,
    pub shop: Option<Shop>,
    pub product: Option<Product>,
}
