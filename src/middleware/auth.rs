use actix_service::{Service, Transform};

use actix_web::{dev::Payload, Error, HttpRequest};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse};

use futures::future::{ok, FutureResult};
use futures::{Future, Poll};

// There are two step in middleware processing.
// 1. Middleware initialization, middleware factory get called with
//    next service in chain as parameter.
// 2. Middleware's call method get called with normal request.
/*
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
*/
pub struct Auth;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for Auth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        println!("Hi from poll_ready");
        self.service.poll_ready()
    }
    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());
        use diesel;
        use diesel::prelude::*;

        let pool = req
            .app_data::<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>()
            .expect("get app_data err 222222222")
            .get_ref()
            .get()
            .expect("pool err1111111111");

        use crate::api::v1::ceo::product::model::{Get, InpNew, New, Product, Update};
        use crate::schema::product::dsl::{id, product as tb};

        let item = tb.filter(&id.eq(1)).load::<Product>(&pool).unwrap();

        let payload = serde_json::json!({
            "item": item,
        });
        println!("{:?}", payload);

        Box::new(self.service.call(req).and_then(|res| {
            println!("Hi from response");
            Ok(res)
        }))
    }
}
