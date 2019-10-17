use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse};
use actix_web::{http::header, Error, HttpResponse};
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Text;
use diesel::{r2d2::ConnectionManager, PgConnection};
use futures::future::{ok, Either, FutureResult};
use futures::Poll;
use std::str::FromStr;

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
        ok(CheckMiddleware { service })
    }
}

pub struct CheckMiddleware<S> {
    service: S,
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
    type Future = Either<S::Future, FutureResult<Self::Response, Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }
    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        let pool = req.app_data::<Pool>().expect("err : get  Pool");
        let conn: &PgConnection = &pool.get().unwrap();

        let login_id: &str = req
            .headers()
            .get("auth_id")
            .expect("req auth_id null")
            .to_str()
            .unwrap();
        let login_role: &str = req
            .headers()
            .get("auth_role")
            .clone()
            .expect("req auth_role null")
            .to_str()
            .unwrap();
        let path: String = req.path().to_string();
        let fields: Vec<&str> = path.split("/").collect();
        println!("=========================={:?}", fields);
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

        let q = sql_query("select * from exist_resource($1,$2,$3,$4,$5) ");

        let chk = q
            .bind::<Text, _>(&login_role.to_string())
            .bind::<Text, _>(&req_user_id.to_string())
            .bind::<Text, _>(&req_shop_id.to_string())
            .bind::<Text, _>(&req_target.to_string())
            .bind::<Text, _>(&req_target_id.to_string())
            .execute(conn)
            .expect("Authorization 조회 오류");

        match login_role {
            "ceo" => match login_id {
                login_id if login_id == req_user_id => match chk {
                    1 => Either::A({
                        println!("=========================={:?}", req_user_id.clone());
                        println!("=========================={:?}", req_shop_id.clone());
                        println!("=========================={:?}", req_target.clone());
                        println!("=========================={:?}", req_target_id.clone());
                        req.headers_mut().insert(
                            header::HeaderName::from_str("req_u_id").unwrap(),
                            header::HeaderValue::from_str(req_user_id.clone()).unwrap(),
                        );
                        req.headers_mut().insert(
                            header::HeaderName::from_str("req_s_id").unwrap(),
                            header::HeaderValue::from_str(req_shop_id.clone()).unwrap(),
                        );
                        req.headers_mut().insert(
                            header::HeaderName::from_str("req_tg_type").unwrap(),
                            header::HeaderValue::from_str(req_target.clone()).unwrap(),
                        );
                        req.headers_mut().insert(
                            header::HeaderName::from_str("req_tg_id").unwrap(),
                            header::HeaderValue::from_str(req_target_id.clone()).unwrap(),
                        );

                        self.service.call(req)
                    }),
                    _ => Either::B(ok(
                        req.into_response(HttpResponse::Unauthorized().finish().into_body())
                    )),
                },
                _ => Either::B(ok(
                    req.into_response(HttpResponse::Unauthorized().finish().into_body())
                )),
            },
            "super" => Either::A(self.service.call(req)),
            _ => Either::B(ok(
                req.into_response(HttpResponse::Unauthorized().finish().into_body())
            )),
        }
    }
}
