use actix_web::Error;
use regex::Regex;
// https://support.symantec.com/ko_KR/article.HOWTO126300.html
pub trait Validate {
    fn validate(&self) -> Result<(), Error>;
}

pub fn re_test_name(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^[a-zA-Z0-9ㄱ-ㅎ가-힣!@#$%^&*()\s]{1,19}$").unwrap();
    }
    RE.is_match(text)
}

pub fn re_test_id(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[[:alpha:]]+[[:alnum:]]{5,19}$").unwrap();
    }
    RE.is_match(text)
}
pub fn re_test_password(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9!@#$%^&]{8,100}$").unwrap();
    }
    RE.is_match(text)
}
pub fn re_test_password_contain_special(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([!@#$%^&])+").unwrap();
    }
    RE.is_match(text)
}
pub fn re_test_password_contain_num(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([0-9])+").unwrap();
    }
    RE.is_match(text)
}
pub fn re_test_email(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^([a-zA-Z0-9._-]+@[a-zA-Z0-9._-]+\.[a-zA-Z0-9_-]+)$").unwrap();
    }
    RE.is_match(text)
}
