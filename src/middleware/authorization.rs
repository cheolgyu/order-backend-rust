use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse};
use actix_web::{Error, HttpResponse};
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Text;
use diesel::{r2d2::ConnectionManager, PgConnection};
use futures::future::{ok, Either, FutureResult};
use futures::Poll;
use regex::Regex;

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
    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let pool = req.app_data::<Pool>().expect("err : get  Pool");
        let conn: &PgConnection = &pool.get().unwrap();

        let login_id: &str = req
            .headers()
            .get("id")
            .expect("req header null")
            .to_str()
            .unwrap();
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
                    1 => Either::A(self.service.call(req)),
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
