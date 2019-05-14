
use actix_web::{ put,post,  HttpRequest, web::Json };
use crate::svc::auth::model::{RegUser};
use crate::svc::validator::{Validate};


#[put("/signup")]
fn signup(req: HttpRequest,json: Json<RegUser>) -> String {

    println!("REQ: {:?}", req);
    
    println!("REQ: {:?}",  json.login.validate());
    //println!("REQ: {:?}",  info.validate());
   

    println!("REQ: {:?}", json);
    format!("Hello signup: {:?}!\r\n", json)
}

#[post("/signin")]
fn signin(req: HttpRequest) -> String {
    println!("REQ: {:?}", req);
    format!("Hello signin: {:?}!\r\n", req)
}