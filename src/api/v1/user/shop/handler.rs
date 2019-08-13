use crate::api::v1::ceo::product::model::SimpleProduct;
use crate::api::v1::user::shop::model::{GetList, GetWithId};
use crate::errors::ServiceError;
use crate::models::msg::Msg;
use crate::models::shop::Shop;
use crate::models::DbExecutor;
use crate::schema::shop::dsl::{id, shop as tb};
use actix::Handler;
use diesel;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Uuid;
use serde_json::json;

impl Handler<GetWithId> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: GetWithId, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;
        let shop = tb.filter(&id.eq(&msg.id)).load::<Shop>(conn)?.pop();
        match shop {
            Some(_shop) => {
                let res = sql_query(
                    "
                    SELECT p.id AS id, 
                        p.shop_id                    AS shop_id, 
                        p.name                       AS name, 
                        p.price                      AS price, 
                        CASE 
                            WHEN Array_length(p.opt_group, 1) IS NULL THEN '[]' 
                            ELSE Array_to_json(Array_agg(optg.*  ORDER BY  array_position(p.opt_Group, optg.id) )) 
                        END  AS option_group_list 
                    FROM   product AS p 
                        left join (SELECT optg.id      AS id, 
                                            optg.shop_id AS shop_id, 
                                            optg.name    AS name, 
                                            optg.default    AS default, 
                                            CASE 
                                            WHEN Array_length(optg.options, 1) IS NULL THEN '[]' 
                                            ELSE Array_to_json(Array_agg(opt.* ORDER BY  array_position(optg.options, opt.id) )) 
                                            END          AS option_list 
                                    FROM   option_group AS optg 
                                            left join OPTION AS opt 
                                                    ON opt.id = ANY ( optg.options ) 
                                    WHERE  optg.shop_id = $1
                                            AND optg.deleted_at IS NULL 
                                    GROUP  BY optg.id) AS optg 
                                ON optg.id = ANY ( p.opt_group ) 
                    WHERE  p.shop_id = $1
                        AND p.deleted_at IS NULL 
                    GROUP  BY p.id 
                ",
                )
                .bind::<Uuid, _>(&msg.id)
                .get_results::<SimpleProduct>(conn)?;

                let payload = json!({
                    "shop": _shop,
                    "product": res,
                });

                Ok(Msg {
                    status: 200,
                    data: payload,
                })
            }
            None => Err(ServiceError::BadRequest("없다".into())),
        }
    }
}

impl Handler<GetList> for DbExecutor {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, _msg: GetList, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;
        let shops = tb.load::<Shop>(conn)?;
        let payload = json!({
            "shops": shops,
        });

        Ok(Msg {
            status: 200,
            data: payload,
        })
    }
}
