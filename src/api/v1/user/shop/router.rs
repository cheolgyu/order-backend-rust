use crate::api::v1::user::shop::model::{GetList, GetWithId};
use crate::errors::ServiceError;
use crate::models::DbExecutor;

use actix::Addr;
use actix_web::{
    web::{Data, Path},
    HttpResponse, ResponseError,
};
use futures::{future::result, Future};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Info {
    pub shop_id: String,
}

pub fn get(
    path_info: Path<Info>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    let info = path_info.into_inner();
    let sid = info.shop_id;

    result(Uuid::parse_str(&sid))
        .from_err()
        .and_then(move |_| {
            let uuid_shop_id = Uuid::parse_str(&sid).unwrap();
            db.send(GetWithId { id: uuid_shop_id }).from_err()
        })
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn get_list(
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    db.send(GetList {}).from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(e) => Ok(e.error_response()),
    })
}
