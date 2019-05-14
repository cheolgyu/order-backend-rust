use crate::svc::validator::{
    re_test_email, re_test_id, re_test_password, re_test_password_contain_num,
    re_test_password_contain_special, Validate,
};
use actix_web::{error, Error};

#[derive(Deserialize, Debug)]
pub struct Login {
    id: String,
    password: String,
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
    password_comfirm: String,
    email: String,
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
