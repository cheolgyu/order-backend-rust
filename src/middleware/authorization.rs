use crate::api::v1::ceo::auth::model::Authorization;
use crate::models::DbExecutor;
use crate::utils::jwt::decode_token;
use actix::Addr;
use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse};
use actix_web::{
    web::{Data, Path},
    Error, HttpResponse,
};
use futures::future::Future;
use futures::future::{ok, Either, FutureResult};
use futures::Poll;
use regex::Regex;
use std::cell::RefCell;
use std::rc::Rc;
use diesel::{r2d2::ConnectionManager, PgConnection};
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Check;

impl<S, B> Transform<S> for Check
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        //ok(CheckMiddleware { service })
        ok(CheckMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct CheckMiddleware<S> {
    //service: S,
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for CheckMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    //type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;
    //type Future = Either<S::Future, FutureResult<Self::Response, Self::Error>>;
    //type Future = FutureResult<Self::Transform, Self::InitError>;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;
    //type Future = Box<Either<S::Future, FutureResult<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }
    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        println!("--------authorization start----------: ");
        let pool = req.app_data::<Pool>(). expect("err : get  Pool");
        let db = req
            .app_data::<Addr<DbExecutor>>()
            .expect("err : get  DbExecutor");
        let login_id: &str = req
            .headers()
            .get("id")
            .expect("req header null")
            .to_str()
            .unwrap();
        println!("--------authorization start----------: ");
        let login_role: &str = req
            .headers()
            .get("role")
            .expect("req header null")
            .to_str()
            .unwrap();
        let path: &str = req.path();
        let re = Regex::new(r"[/]+").unwrap();
        let fields: Vec<&str> = re.split(path).collect();
        let req_user_id: &str = match fields.get(4) {
            Some(x) => x,
            None => "",
        };
        let req_shop_id: &str = match fields.get(6) {
            Some(x) => x,
            None => "",
        };
        let req_target: &str = match fields.get(7) {
            Some(x) => x,
            None => "",
        };
        let req_target_id: &str = match fields.get(8) {
            Some(x) => x,
            None => "",
        };
        let data = Authorization {
            role: login_role.to_string(),
            user_id: req_user_id.to_string(),
            shop_id: req_shop_id.to_string(),
            target: req_target.to_string(),
            target_id: req_target_id.to_string(),
        };

        let res = match login_role {
            "ceo" => match login_id {
                req_user_id => "ok user_id",
                _ => "who are yoou ",
            },
            _ => "who are yoou",
        };
        println!("------------------: {}", path);
        println!("------------------: {}", res);
        use crate::errors::ServiceError;
/*
        let chk = db.send(data).from_err().and_then(move |res| {
            println!("request body: {:?}", res);
            res
        });
        */
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::{Integer, Uuid as uu,Text};
let conn: &PgConnection = &pool.get().unwrap();
let q = sql_query("select * from exist_resource($1,$2,$3,$4,$5) ");

        let chk = q
            .bind::<Text, _>(&login_role.to_string())
            .bind::<Text, _>(&req_user_id.to_string())
            .bind::<Text, _>(&req_shop_id.to_string())
            .bind::<Text, _>(&req_target.to_string())
            .bind::<Text, _>(&req_target_id.to_string())
            .execute(conn)
            .expect("Authorization 조회 오류");

        if chk == 1{
            println!("-------------ok-----: ");
        }else{
            println!("-------------nok-----: ");
        }
        Box::new(
            self.service.call(req)
         )
        
        /*
        println!("------------------: chk");
        Box::new(self.service.call(req).and_then(|res| {
            println!("Hi from response");
            Ok(res)
        }))
*/
        /*
        Box::new(
            db.send(data).from_err()
                .and_then(|res| match res {
                Ok(r) => {
                    println!("res:{:?}", r);
                    self.service.call(req)
                }
                Err(e) => {
                    println!("e:{:?}", e);

                }
            })
        )
        */
        /*
                if res == "ok user_id" {
                    db.send(data).from_err().and_then(|res| match res {
                        Ok(r) => {
                            println!("res:{:?}", r);
                            Either::A(self.service.call(req))
                        }
                        Err(e) => {
                            println!("e:{:?}", e);
                            Either::B(ok(
                                req.into_response(HttpResponse::Unauthorized().finish().into_body())
                            ))
                        }
                    })
                } else {
                    Either::B(ok(
                        req.into_response(HttpResponse::Unauthorized().finish().into_body())
                    ))
                }
        */
        /*
        if (true) {
            Either::A(self.service.call(req))
        } else {
            Either::B(ok(
                req.into_response(HttpResponse::Unauthorized().finish().into_body())
            ))
        }
        */
        /*
                let info = Info {
                    ceo_id: ,
                    shop_id: ,
                    product_id: ,
                    auth_user: ,
                }
        */
    }
}
