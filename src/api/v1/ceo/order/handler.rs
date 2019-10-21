use crate::api::v1::ceo::order::model;
use crate::errors::ServiceError;
use crate::models::order::Order as Object;
use crate::models::DbExecutor;
use crate::schema::order::dsl::{deleted_at, id, order as tb, shop_id, state, created_at};
use actix::Handler;
use chrono::{Duration, Utc};

use crate::models::msg::Msg;
use diesel;
use diesel::prelude::*;

impl Handler<model::Update> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: model::Update, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        let item_update =
            diesel::update(tb.filter(&id.eq(&msg.id)).filter(&shop_id.eq(&msg.shop_id)))
                .set(&msg)
                .get_result::<Object>(conn)?;

        let payload = serde_json::json!({
            "item_update": item_update,
        });
        Ok(Msg {
            status: 200,
            data: payload,
        })
    }
}

impl Handler<model::Get> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: model::Get, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        let item = tb.find(&msg.id).get_result::<Object>(conn)?;

        let payload = serde_json::json!({
            "item": item,
        });
        Ok(Msg {
            status: 200,
            data: payload,
        })
    }
}

impl Handler<model::GetList> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, _msg: model::GetList, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        let item = tb
            .filter(&shop_id.eq(&_msg.shop_id))
            .filter(&deleted_at.is_null())
            .get_results::<Object>(conn)?;

        let payload = serde_json::json!({
            "item": item,
        });
        Ok(Msg {
            status: 200,
            data: payload,
        })
    }
}

impl Handler<model::NowList> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, _msg: model::NowList, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        let now2 = Utc::now().naive_utc().checked_add_signed(Duration::hours(-2)).unwrap();

        let item = tb
            .filter(&shop_id.eq(&_msg.shop_id))
            .filter(&deleted_at.is_null())
            .filter(&created_at.gt(now2))
            .filter(&state.eq(1))
            .or_filter(&state.eq(2))
            .or_filter(&state.eq(3))
            .or_filter(&state.eq(3))
            .get_results::<Object>(conn)?;

        let payload = serde_json::json!({
            "item": item,
        });
        Ok(Msg {
            status: 200,
            data: payload,
        })
    }
}
