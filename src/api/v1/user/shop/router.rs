use crate::api::v1::ceo::auth::model::{AuthUser, Info};
use crate::api::v1::ceo::shop::model::{InpNew, ShopID};

use crate::models::DbExecutor;

use crate::utils::validator::Validate;
use actix::Addr;
use actix_web::{
    delete, error, get, post, put,
    web::{self, Data, Json, Path},
    Either, Error, HttpRequest, HttpResponse, Responder, ResponseError,
};
use futures::{future::result, Future};
use uuid::Uuid;


pub fn get(
    path_info: Path<Info>,
    db: Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    println!("shop ---g et ");
    let info = path_info.into_inner();
    let sid = info.shop_id.unwrap();
    let uuid_shop_id = Uuid::parse_str(&sid).unwrap();
    db.send(ShopID { id: uuid_shop_id })
        .from_err()
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(err) => Ok(err.error_response()),
        })
}
