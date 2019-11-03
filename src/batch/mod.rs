pub mod handler;
pub mod model;
use crate::batch::model::{AutoCancel, ComeFind};
use crate::models::{AppStateWithTxt, DbExecutor};
use actix::prelude::*;
use futures::Future;
use std::time::Duration;

use crate::errors::ServiceError;
use crate::fcm::model::*;
use crate::fcm::router::to_user;
use crate::utils::client::SSLClinet;
use actix_web::{web::Data, Error};

pub struct Batch {
    pub db: Data<Addr<DbExecutor>>,
    pub store: Data<AppStateWithTxt>,
}

impl Batch {
    pub fn new(db: Data<Addr<DbExecutor>>, store: Data<AppStateWithTxt>) -> Batch {
        Batch { db, store }
    }

    fn come_find(&self, ctx: &mut actix::Context<Self>) {
        ctx.run_interval(Duration::new(3, 0), move |act, _ctx| {
            let result = index4(act.db.clone(), act.store.clone());
            // spawn future to reactor
            Arbiter::spawn(
                result
                    .map(|_res| {
                        //println!("Got result: {}", res);
                    })
                    .map_err(|e| {
                        println!("batch: auto_cancel : {}", e);
                    }),
            );
        });
    }

    fn auto_cancel(&self, ctx: &mut actix::Context<Self>) {
        ctx.run_interval(Duration::new(3, 0), move |act, _ctx| {
            let result = index3(act.db.clone(), act.store.clone());
            // spawn future to reactor
            Arbiter::spawn(
                result
                    .map(|_res| {
                        //println!("Got result: {}", res);
                    })
                    .map_err(|e| {
                        println!("batch: auto_cancel : {}", e);
                    }),
            );
        });
    }
}

impl Actor for Batch {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("I am alive!");
        self.auto_cancel(ctx);
        self.come_find(ctx);
    }
}

fn index4(
    db: Data<Addr<DbExecutor>>,
    store: Data<AppStateWithTxt>,
) -> Box<dyn Future<Item = &'static str, Error = Error>> {
    use futures::future::ok;
    let sd = ComeFind {
        db: db.clone(),
        store: store.clone(),
    };
    let db2 = db.clone();
    let store2 = store.clone();
    Box::new({
        db.send(sd).from_err().and_then(move |res| match res {
            Ok(list) => {
                for res in &list {
                    let db_addr = db2.clone();
                    let store3 = store2.clone();
                    let to = res.to.clone();
                    let title = format!("[{}] 수령하세요.", res.shop_name);
                    let body = res.content.to_string();
                    let send_data = ReqToUser {
                        comm: ReqToComm::new_comefind(
                            res.order_id.clone(),
                            res.order_detail_id.clone(),
                            res.shop_notification_id.clone(),
                        ),
                        params: ReqToUserData::new(to, title, body),
                    };

                    let result = to_user(send_data, db_addr, store3);
                    Arbiter::spawn(
                        result
                            .map(|_res| {
                                // println!("Actor is  map");
                            })
                            .map_err(|e| {
                                println!("Actor is probably dead: {}", e);
                            }),
                    );
                }
                ok::<_, Error>("Welcome!2 Welcome")
            }
            Err(e) => {
                println!(" index4--: errr {:?}", e);
                ok::<_, Error>("Welcome!2 ERRR")
            }
        })
    })
}

fn index3(
    db: Data<Addr<DbExecutor>>,
    store: Data<AppStateWithTxt>,
) -> Box<dyn Future<Item = &'static str, Error = Error>> {
    use futures::future::ok;
    let websocket_url = store.websocket.send.clone();
    let sd = AutoCancel {
        db: db.clone(),
        store: store.clone(),
    };
    let db2 = db.clone();
    let store2 = store.clone();
    Box::new({
        db.send(sd).from_err().and_then(move |res| match res {
            Ok(list) => {
                for res in &list {
                    let db_addr = db2.clone();
                    let store3 = store2.clone();
                    let shop_id = res.shop_id.clone();

                    let to = res.notification_key.clone();
                    let title = format!("[자동] 주문후 5분 미응답");
                    let body = format!("주문 자동취소됨.!");

                    let send_data = ReqToUser {
                        comm: ReqToComm::new_auto_cancle(res.id.clone()),
                        params: ReqToUserData::new(to, title, body),
                    };
                    let websocket_url2 = websocket_url.clone();

                    let o_id = res.id.clone();
                    let user_to = res.sw_token.clone();
                    let user_title = format!("주문이 취소됬습니다.");
                    let user_body = format!("가게가 미응답하여 자동 취소됬습니다.");
                    let u_d=
                            ReqToUser {
                            comm: ReqToComm::new_order(o_id.clone()),
                            params: ReqToUserData::new(user_to.clone(), user_title.clone(), user_body.clone()),
                        };
                    let send_user =to_user(u_d, db_addr.clone(), store2.clone());
 
                    let result = to_user(send_data, db_addr, store3).and_then(|_|{
                        send_user
                    })
                    .and_then(move |res| {
                        let url = format!("{}{}/test", websocket_url2, shop_id);
                        SSLClinet::build()
                            .get(url)
                            .send()
                            .map_err(|e| {
                                println!("SSLClinet::build(): {}", e);
                                ServiceError::BadRequest(e.to_string())
                            })
                            .and_then(|_response| res)
                            .from_err()
                    });
                    Arbiter::spawn(
                        result
                            .map(|_res| {
                                // println!("Actor is  map");
                            })
                            .map_err(|e| {
                                println!("Actor is probably dead: {}", e);
                            }),
                    );
                }
                ok::<_, Error>("Welcome!2 Welcome")
            }
            Err(e) => {
                println!(" index3--: errr {:?}", e);
                ok::<_, Error>("Welcome!2 ERRR")
            }
        })
    })
}
