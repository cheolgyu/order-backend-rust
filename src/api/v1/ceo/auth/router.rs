use crate::errors::ServiceError;
use crate::models::DbExecutor;
use crate::api::v1::ceo::auth::model::{AuthUser, InpNew, Login, QueryUser, SlimUser};
use crate::utils::jwt::{create_token, decode_token};
use crate::utils::validator::Validate;
use actix::Addr;
use actix_web::{
    get, post, put,
    web::{self, Data, Json, Path},
    Error, HttpRequest, HttpResponse, Responder, ResponseError,
};
use futures::{future::result, Future};
use uuid::Uuid;

pub fn signup(
    json: Json<InpNew>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    result(json.validate())
        .from_err()
        .and_then(move |_| db.send(json.into_inner().new()).from_err())
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn signin(
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
                #[derive(Debug, Serialize, Deserialize)]
                struct Msg {
                    pub user: SlimUser,
                    pub token: String,
                }
                Ok(HttpResponse::Ok().json(Msg {
                    user: _user,
                    token: t,
                }))
            }
            //Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn getme(
    req: HttpRequest,
    auth_user: AuthUser,
    path_id: Path<String>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    println!(" ====================get me ");
    println!(" ====================get me {:?}", auth_user);
    println!(" ====================get me {:?}", req);
    result(auth_user.check_user(path_id.into_inner()))
        .from_err()
        .and_then(move |_| db.send(QueryUser { id: auth_user.id }).from_err())
        .and_then(move |res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(er) => Ok(er.error_response()),
        })
}
