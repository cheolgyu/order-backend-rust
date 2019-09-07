use crate::api::v1::ceo::auth::model::{AuthUser};
use crate::api::v1::ceo::device::model as params;
use crate::api::v1::ceo::fcm::router as fcm;
use crate::errors::ServiceError;
use crate::models::device as m;
use crate::models::shop::UpdateNotificationKey;
use crate::models::{AppStateWithTxt, DbExecutor};
use crate::utils::validator::Validate;
use actix::Addr;
use actix_web::{
    client::Client,
    web::{ Data, Json},
    Error, HttpResponse, ResponseError,
};
use futures::{
    future::{err, result, Either},
    Future, Stream,
};
use uuid::Uuid;


pub fn check(
    json: Json<params::InpCheck>,
    auth_user: AuthUser,
    db: Data<Addr<DbExecutor>>,
    client: Data<Client>,
    store: Data<AppStateWithTxt>,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    let sw_token = json.into_inner().sw_token.clone();
    let sw_token2 = sw_token.clone();
    let vec = vec![sw_token2.to_string()];
    
    let db2 = db.clone();
    let db3 = db.clone();
    let db4 = db.clone();
    

    db.send(m::Get {
        sw_token: sw_token,
        user_id: auth_user.id,
    })
    .from_err()
    .and_then(move |res| match res {
        Ok(get_with_key) => {
            let opt_send_data = get_with_key.get(store.webpush.clone());
            if opt_send_data.is_some() {
                let send_data = opt_send_data.unwrap();
                let shop_id = Uuid::parse_str(&send_data.params.notification_key_name.clone()).unwrap();
                Either::A(
                    fcm::send(send_data,store.webpush.clone() ,client,  db2)
                        .and_then(move |notification_key| {
                            println!("==============================================");
                            println!("update shop notification_key  : {:?}", notification_key);
                            println!("==============================================");
                            db3.send(UpdateNotificationKey {
                                id: shop_id,
                                notification_key: notification_key,
                            })
                            .from_err()
                        }).and_then(move |res| {
                            println!("==============================================");
                            println!("insert user device ");
                            println!("==============================================");
                            db4.send(m::New {
                                user_id: get_with_key.params.user_id.clone(),
                                name: "test".to_string(),
                                sw_token: get_with_key.params.sw_token.clone(),
                            })
                            .from_err()
                        })
                        .map_err(|e| {
                            println!("check device : {:?}", e.error_response());
                            ServiceError::BadRequest("check device".into())
                        })
                        .then(|res| match res {
                            Ok(user) => Ok(HttpResponse::Ok().json("2")),
                            Err(_) => Ok(HttpResponse::InternalServerError().into()),
                        }),
                )
            } else {
                Either::B(result(Ok(HttpResponse::Ok().json("pass"))))
            }
        }
        Err(e) => Either::B(err(ServiceError::BadRequest("check device".into()))),
    })
}



pub fn put(
    json: Json<params::InpNew>,
    auth_user: AuthUser,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    result(json.validate())
        .from_err()
        .and_then(move |_| db.send(json.into_inner().new(auth_user)).from_err())
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn get(
    auth_user: AuthUser,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    db.send(m::GetList {
        user_id: auth_user.id,
    })
    .from_err()
    .and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(err) => Ok(err.error_response()),
    })
}

pub fn post( 
    json: Json<params::InpUpdate>,
    auth_user: AuthUser,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    result(json.validate())
        .from_err()
        .and_then(move |_| db.send(json.into_inner().update(auth_user)).from_err())
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}
