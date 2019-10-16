use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse};
use actix_web::{http::header, Error, HttpResponse};
use futures::future::{ok, Either, FutureResult};
use futures::Poll;
use std::str::FromStr;

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
    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        println!("--------auth start----------: ");
        if let Some(auth_token) = req.headers().get("authorization") {
            if let Ok(auth_token_str) = auth_token.to_str() {
                if let Ok(auth_user) = decode_token(auth_token_str) {
                    let u_id = auth_user.id.hyphenated().to_string();
                    let u_role = auth_user.role.to_string();

                    req.headers_mut().insert(
                        header::HeaderName::from_str("id").unwrap(),
                        header::HeaderValue::from_str(&u_id).unwrap(),
                    );
                    req.headers_mut().insert(
                        header::HeaderName::from_str("role").unwrap(),
                        header::HeaderValue::from_str(&u_role).unwrap(),
                    );

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
