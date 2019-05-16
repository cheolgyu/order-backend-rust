use crate::errors::ServiceError;
use crate::models::DbExecutor;
use crate::svc::auth::model::{Login, RegUser, SlimUser};
use crate::svc::validator::Validate;
use crate::utils::jwt::create_token;
use actix::Addr;
use actix_web::{
    middleware::identity::Identity,
    post, put,
    web::{self, Data, Json, Path},
    Error, HttpRequest, HttpResponse, Responder, ResponseError,
};
use futures::{future::result, Future};
#[put("/signup")]
pub fn signup(
    req: HttpRequest,
    json: Json<RegUser>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    result(json.validate())
        .from_err()
        .and_then(move |_| db.send(json.into_inner()).from_err())
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}

#[post("/signin")]
pub fn signin(
    req: HttpRequest,
    identity: Identity,
    json: Json<Login>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    result(json.validate())
        .from_err()
        .and_then(move |_| db.send(json.into_inner()).from_err())
        .and_then(move |res| match res {
            Ok(_user) => {
                let token = create_token(&_user)?;
                let t = token.to_string();
                let slim_json = serde_json::to_string(&_user).unwrap();
                #[derive(Debug, Serialize, Deserialize)]
                struct Msg {
                    pub user: SlimUser,
                    pub token: String,
                }
                identity.remember(token);
                Ok(HttpResponse::Ok().json(Msg {
                    user: _user,
                    token: t,
                }))
            }
            //Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}
