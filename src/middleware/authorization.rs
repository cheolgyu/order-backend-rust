use crate::models::DbExecutor;
use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse};
use actix_web::{
    web::{Data, Path},
    Error, HttpResponse,
};
use futures::future::{ok, Either, FutureResult};
use futures::Poll;

use crate::api::v1::ceo::auth::model::{AuthUser, Info};
use crate::utils::jwt::decode_token;

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
        println!("Check Transform new_transform : ");
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
    //type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;
    type Future = Either<S::Future, FutureResult<Self::Response, Self::Error>>;
    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }
    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let db = req.app_data::<DbExecutor>().unwrap();
        let login_id = req.headers().get("id");
        let login_role = req.headers().get("role");
        let path = req.path();
        println!("........Hi from start. You requested: {}", path);
        if (true) {
            Either::A(self.service.call(req))
        } else {
            Either::B(ok(
                req.into_response(HttpResponse::Unauthorized().finish().into_body())
            ))
        }
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
