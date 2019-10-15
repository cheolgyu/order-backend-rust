use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse};
use actix_web::{Error, HttpResponse};
use futures::future::{ok, Either, FutureResult};
use futures::Poll;

use crate::utils::jwt::decode_token;

pub struct CheckToken;

impl<S, B> Transform<S> for CheckToken
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckTokenMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        println!("CheckToken Transform new_transform : ");
        ok(CheckTokenMiddleware { service })
    }
}

pub struct CheckTokenMiddleware<S> {
    service: S,
}

impl<S, B> Service for CheckTokenMiddleware<S>
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
        if let Some(auth_token) = req.headers().get("authorization") {
            if let Ok(auth_token_str) = auth_token.to_str() {
                if let Ok(_user) = decode_token(auth_token_str) {
                    Either::A(self.service.call(req))
                } else {
                    Either::B(ok(
                        req.into_response(HttpResponse::Unauthorized().finish().into_body())
                    ))
                }
            } else {
                Either::B(ok(
                    req.into_response(HttpResponse::Unauthorized().finish().into_body())
                ))
            }
        } else {
            Either::B(ok(
                req.into_response(HttpResponse::Unauthorized().finish().into_body())
            ))
        }
    }
}
