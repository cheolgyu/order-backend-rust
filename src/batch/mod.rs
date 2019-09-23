pub mod handler;
pub mod model;
use crate::batch::model::{AutoCancel,ComeFind};
use crate::models::{AppStateWithTxt, DbExecutor};
use actix::prelude::*;
use futures::Future;
use std::time::Duration;

use crate::fcm::model::*;
use crate::fcm::router::to_user;
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
        ctx.run_interval(Duration::new(3, 0), move |act, ctx| {
            let result = index4(
                "batch::come_find".to_string(),
                act.db.clone(),
                act.store.clone(),
            );
            // spawn future to reactor
            Arbiter::spawn(
                result
                    .map(|res| {
                        //println!("Got result: {}", res);
                    })
                    .map_err(|e| {
                        println!("batch: auto_cancel : {}", e);
                    }),
            );
        });
    }

    fn auto_cancel(&self, ctx: &mut actix::Context<Self>) {
        ctx.run_interval(Duration::new(3, 0), move |act, ctx| {
            let result = index3(
                "batch::auto_cancel".to_string(),
                act.db.clone(),
                act.store.clone(),
            );
            // spawn future to reactor
            Arbiter::spawn(
                result
                    .map(|res| {
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
    trigger: String,
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
                    let send_data = ReqToUser {
                        comm: ReqToComm::new_auto_cancle(trigger.clone(), res.id.clone()),
                        params: ReqToUserData {
                            notification: Notification {
                                title: "[자동] 수령 하세요.".to_string(),
                                body: "22".to_string(),
                                icon: "33".to_string(),
                                click_action: "44".to_string(),
                            },
                            to: res.notification_key.clone(),
                        },
                    };
                    println!(" db handler  for : ");

                    let result = to_user(send_data, db_addr, store3);
                    Arbiter::spawn(
                        result
                            .map(|res| {
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
                println!(" index3--: errr ");
                ok::<_, Error>("Welcome!2 ERRR")
            }
        })
    })
}


fn index3(
    trigger: String,
    db: Data<Addr<DbExecutor>>,
    store: Data<AppStateWithTxt>,
) -> Box<dyn Future<Item = &'static str, Error = Error>> {
    use futures::future::ok;
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
                    let send_data = ReqToUser {
                        comm: ReqToComm::new_auto_cancle(trigger.clone(), res.id.clone()),
                        params: ReqToUserData {
                            notification: Notification {
                                title: "[자동] 주문 5분 미응답".to_string(),
                                body: "22".to_string(),
                                icon: "33".to_string(),
                                click_action: "44".to_string(),
                            },
                            to: res.notification_key.clone(),
                        },
                    };
                    println!(" db handler  for : ");

                    let result = to_user(send_data, db_addr, store3);
                    Arbiter::spawn(
                        result
                            .map(|res| {
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
                println!(" index3--: errr ");
                ok::<_, Error>("Welcome!2 ERRR")
            }
        })
    })
}
