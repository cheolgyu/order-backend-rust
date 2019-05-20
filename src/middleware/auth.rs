use crate::errors::ServiceError;
use crate::svc::auth::model::AuthUser;
use crate::utils::jwt::decode_token;
use actix_service::{Service, Transform};
use actix_web::{dev::Payload, Error, HttpRequest};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse};
use actix_web::{error, middleware::identity::Identity, FromRequest};
use futures::future::{ok, FutureResult};
use futures::{Future, Poll};
use regex::Regex;

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
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        println!("Hi from poll_ready");
        self.service.poll_ready()
    }
    // https://actix.rs/actix-web/src/actix_web/middleware/identity.rs.html#1-1037 245 줄 보고 핸들러에서 identity 파라메터로 사용하는것처럼
    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());
        let p = req.path();

        let re = Regex::new(r"^/api/v1/auth").unwrap();
        let r1 = re.is_match(p);

        if (r1) {
            println!("match !!");
        } else {
            println!(" not match !! check toke");
            if let Some(auth_token) = req.headers().get("authorization") {
                if let Ok(auth) = auth_token.to_str() {
                    let user: AuthUser = decode_token(auth).unwrap();

                    println!(" not match !! check token  :{:?}", user);

                    //사용자 권한이 필요한 경로 입니다.
                    //토큰의 사용자 role 과 id 로 db를 조회후 사용자 요청한 자원에 대한 접근 권환을 확인합니다.
                    //요청한 경로의 자원에 접근 권한이 있음으로 핸들러에게 조회 한 자원을 같이 전달합니다.
                    //핸들러에서는 인증된 자원임을 믿고 자원 id 를 이용합니다.
                    //근데 인증된 자원을 핸들러에게 어떻게 넘깁니까?

                    //println!(" not match !! token check req.req :{:?}", req.req);
                    //println!(" not match !! token check req.payload:{:?}", req.payload);
                    //req.req.Set("AuthUser");
                }
            }
        }

        /*
         // Create the HTTP response
        let resp = http::Response::ok()
            .with_body(b"hello world\n");

        // Return the response as an immediate future
        futures::finished(resp).boxed()

         */

        println!("Hi from start. You requested: {}", req.path());

        // Box::new( self.from_request(&mut req).into_future().then())
        Box::new(self.service.call(req).and_then(|res| {
            println!("Hi from response");
            Ok(res)
        }))
    }
}
