use crate::errors::ServiceError;
use crate::model::msg::Msg;

use crate::schema::option;

use crate::utils::validator::{
    re_test_email, re_test_id, re_test_name, re_test_password, re_test_password_contain_num,
    re_test_password_contain_special, Validate,
};
use actix::Message;
use actix_web::error;
use actix_web::{dev::Payload, Error, HttpRequest};

use chrono::{Duration, Local, NaiveDateTime, Utc};

use crate::schema::option::dsl::{deleted_at, id, name, option as tb, shop_id};
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use uuid::Uuid;

pub trait CURD<T> {
    fn all(_shop_id: &Uuid,conn: &PgConnection) -> QueryResult<Vec<T>>;
    fn get_with_id(id: &i32 ,conn: &PgConnection) -> QueryResult<T>;
    fn insert(data: T,conn: &PgConnection) -> QueryResult<usize>;
    fn update(data: T,conn: &PgConnection) -> QueryResult<usize>;
    fn delete_with_id(id: &i32,conn: &PgConnection) -> QueryResult<usize>;
}

#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Identifiable,
    Queryable,
    Insertable,
    Associations,
    AsChangeset
)]
#[table_name = "option"]
pub struct Opt {
    pub id: i32,
    pub shop_id: Uuid,
    pub name: String,
    pub price: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl CURD<Opt> for Opt {
    fn all(_shop_id: &Uuid, conn: &PgConnection) -> QueryResult<Vec<Opt>> {
        tb
            .filter(&shop_id.eq(&_shop_id))
            .filter(&deleted_at.is_null())
            .load::<Opt>(conn)
    }
    fn get_with_id(_id: &i32 ,conn: &PgConnection) -> QueryResult<Opt>{
        tb.filter(&id.eq(&_id)).get_result::<Opt>(conn)
    }
    fn insert(data: Opt,conn: &PgConnection) -> QueryResult<usize>{
        diesel::insert_into(option::table)
            .values(&data)
            .execute(conn)
    }
    fn update(data: Opt,conn: &PgConnection) -> QueryResult<usize>{
        let update = diesel::update(tb.find(&data.id));
        update.set(&data).execute(conn)
    }
    fn delete_with_id(_id: &i32,conn: &PgConnection) -> QueryResult<usize>{
        let update = diesel::update(tb.find(_id));
        update.set(deleted_at.eq(diesel::dsl::now)).execute(conn)
    }
}
