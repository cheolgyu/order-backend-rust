use actix_service::{Service, Transform};
use actix_web::Error;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse};

use crate::models::product::Product as Object;
use crate::schema::product::dsl::{id, product as tb};
use futures::future::{ok, FutureResult};
use futures::{Future, Poll};
pub struct Auth;

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
        self.service.poll_ready()
    }
    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        use diesel::prelude::*;

        let pool = req
            .app_data::<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>()
            .expect("AuthMiddleware 오류")
            .get_ref()
            .get()
            .expect("AuthMiddleware 오류 get pull");

        let item = tb.filter(&id.eq(1)).load::<Object>(&pool).unwrap();

        Box::new(self.service.call(req).and_then(|res| Ok(res)))
    }
}
