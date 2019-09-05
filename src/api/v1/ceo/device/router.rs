use crate::api::v1::ceo::auth::model::{AuthUser, Info};
use crate::api::v1::ceo::device::model as params;
use crate::errors::ServiceError;
use crate::models::device as m;
use crate::models::shop::UpdateNotificationKey;
use crate::models::{AppStateWithTxt, DbExecutor};
use crate::utils::validator::Validate;
use actix::Addr;
use actix_web::{
    client::Client,
    delete,
    http::{header, StatusCode},
    web::{BytesMut, Data, Json, Path},
    Error, HttpResponse, ResponseError,
};
use futures::{future::result, Future, Stream};
use std::fmt;
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize)]
pub struct FcmResponse {
    pub notification_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SendData {
    operation: String,
    notification_key_name: String,
    registration_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FcmSend {
    req_type: String,
    send_data: SendData,
}
// 등록 프로세스 필요.
/**
 * 0. 사용자uuid로 상점의 그룹key 와 토큰 조회
 * 0-1 토큰이 있다면 ok 이미 등록된 디바이스.
 * 0-2 토큰이 없다면 신규 사용자.
 * 0-1-1 기존 디바이스고 푸시키가 없다면 푸시키 발급.
 * 0-1-2 기존 디바이스고 푸시키가 있다면 ok.
 * 0-2-1 신규 디바이스이고 푸시키가 없다면. 푸시키 발급.
 * 0-2-2 신규 디바이스이고 푸시키가 있다면. 푸시키에 신규디바이스 등록.
 *
 * 1. 토큰 db에 저장
 * 2. db 에서 토큰 그룹key 조회
 * 3-1  있다면 기기등록 rest api 실행
 * 3-2 없다면 토큰 그룹key 생성( 생성시 사용자1명 필요.) rest api 실행
 * 3-2-1  생성된 그룹키 db에 저장.
 *
 */

/*
let sendData =  Create{
    operation: "create".to_string(),
    notification_key_name: getWithKey.shop_id.to_string(),
    registration_ids: vec.clone()
};

println!("ws push sendData: {:?}", sendData);

Client::default()
    .post(webpush_group_reg_url) // <- Create request builder
    .header("Authorization", key)
    .header(header::CONTENT_TYPE, "application/json")
    .send_json(&sendData) // <- Send http request
    .map_err(|e| {
        println!("sw push error : {:?}", e.error_response());
        ServiceError::BadRequest("sw push error".into())
    } )
    .and_then(|response| {
        // <- server http response
        println!("sw push Response: {:?}", response);

        //Ok(res)
    }).from_err();
*/

pub fn check(
    json: Json<params::InpCheck>,
    auth_user: AuthUser,
    db: Data<Addr<DbExecutor>>,
    client: Data<Client>,
    txt: Data<AppStateWithTxt>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let sw_token = json.into_inner().sw_token.clone();
    let sw_token2 = sw_token.clone();
    let vec = vec![sw_token2.to_string()];
    let webpush_group_reg_url = txt.webpush_group_reg_url.clone();
    let key = txt.get_key();
    let db2 = db.clone();

    db.send(m::Get {
        sw_token: sw_token,
        user_id: auth_user.id,
    })
    .from_err()
    .and_then(move |res| {
        let chk_type = match &res {
            Ok(get_with_key) => (get_with_key.get_type(), get_with_key.shop_id.clone()),
            Err(e) => {
                println!("err================err==============================");
                ("err".to_string(), "err".to_string())
            }
        };
        let shop_id = Uuid::parse_str(&chk_type.1.clone()).unwrap();
        println!("==============================================");
        println!("chk_type: {:?}", chk_type);
        println!("==============================================");
        if (chk_type.0 == "new group") {
            
        } else if (chk_type.0 == "new device") {

        } else if (chk_type.0 == "pass") {

        }

        let send_data = SendData {
            operation: "create".to_string(),
            notification_key_name: chk_type.1.to_string(),
            registration_ids: vec.clone(),
        };

        println!("ws push sendData: {:?}", send_data);

        let resp = client
            .post(webpush_group_reg_url)
            .header("Authorization", key)
            .header("project_id", "371794845174".to_string())
            .header(header::CONTENT_TYPE, "application/json")
            .send_json(&send_data)
            //.map_err(Error::from)
            .map_err(|e| {
                println!("sw push error : {:?}", e.error_response());
                ServiceError::BadRequest("sw push error".into())
            })
            .and_then(|response| {
                let _notification_key = response
                    .from_err()
                    .fold(BytesMut::new(), |mut acc, chunk| {
                        acc.extend_from_slice(&chunk);
                        Ok::<_, ServiceError>(acc)
                    })
                    .map(|body| {
                        let body: FcmResponse = serde_json::from_slice(&body).unwrap();
                        println!("==============================================");
                        println!("response.body(): {:?}", body);
                        println!("==============================================");
                        body.notification_key
                    });
                _notification_key
            });

        println!("==============================================");

        resp.and_then(move |notification_key| {
            println!("==============================================");
            println!("response.body() notification_key: {:?}", notification_key);
            println!("==============================================");
           
            db2.send(UpdateNotificationKey {
                id: shop_id,
                notification_key: notification_key,
            });
            res
        })
    })
    .from_err()
    .and_then(|res| Ok(HttpResponse::Ok().json(res)))
}

pub fn send(
    fcm_send:FcmSend,
    client: Data<Client>,
    txt: Data<AppStateWithTxt>) -> impl Future<Item = String, Error = ServiceError> {
   
    let resp = client
        .post(txt.webpush_group_reg_url.clone())
        .header(header::CONTENT_TYPE, "application/json")
        .header("Authorization", txt.get_key())
        .header("project_id", "371794845174".to_string())
        .send_json(&fcm_send.send_data)
        .map_err(|e| {
            println!("sw push error : {:?}", e.error_response());
            ServiceError::BadRequest("sw push error".into())
        })
        .and_then(|response| {
            let _notification_key = response
                .from_err()
                .fold(BytesMut::new(), |mut acc, chunk| {
                    acc.extend_from_slice(&chunk);
                    Ok::<_, ServiceError>(acc)
                })
                .map(|body| {
                    let body: FcmResponse = serde_json::from_slice(&body).unwrap();
                    println!("==============================================");
                    println!("response.body(): {:?}", body);
                    println!("==============================================");
                    body.notification_key
                });
            _notification_key
        });
    resp
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
