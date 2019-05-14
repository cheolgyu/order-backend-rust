use crate::svc::auth::model::RegUser;
use crate::svc::validator::Validate;
use actix_web::{post, put, web::Json, HttpRequest};

#[put("/signup")]
fn signup(req: HttpRequest, json: Json<RegUser>) -> String {
    println!("REQ: {:?}", req);

    println!("REQ: {:?}", json.login.validate());
    println!("REQ: {:?}", json.validate());

    format!("Hello signup: {:?}!\r\n", json)
}

#[post("/signin")]
fn signin(req: HttpRequest) -> String {
    println!("REQ: {:?}", req);
    format!("Hello signin: {:?}!\r\n", req)
}
