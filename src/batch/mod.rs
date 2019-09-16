pub mod model;
pub mod handler;
use actix::prelude::*;
use std::time::{Duration, Instant};
use crate::models::{AppStateWithTxt, DbExecutor};
use crate::batch::model::{OrderState,OrderStateRes};
use crate::api::v1::ceo::fcm::router as fcm;
use crate::models::fcm::{ParamsToUser,ParamsNotification, Notification};
use crate::errors::ServiceError;
use crate::models::msg::Msg;
use futures::Future;
use futures::stream::Stream;

use actix_web::{
    
    client::Client,
    web::{self,Data, Json},
    Error, HttpResponse, ResponseError,
    };
pub struct Batch{
    pub db: Data<Addr<DbExecutor>>,
    pub store: Data<AppStateWithTxt>,
}

impl Batch {
     pub fn new(
        db: Data<Addr<DbExecutor>>,
        store: Data<AppStateWithTxt>,
    ) -> Batch {
        Batch {
            db,
            store
        }
    }
}

// https://rust-examples.blogspot.com/2018/10/how-to-execute-function-every-n-seconds.html

//https://docs.rs/tokio/0.2.0-alpha.4/tokio/?search=#examples
////https://github.com/actix/book/blob/master/actix/src/sec-2-actor.md#spawning-an-actor
//https://github.com/paulkernfeld/future-by-example/blob/master/src/lib.rs

impl Actor for Batch {
    type Context = Context<Self>;

     fn started(&mut self, ctx: &mut Self::Context) {
        println!("I am alive!");
       
        ctx.run_interval(Duration::new(3, 0), move |act, ctx| {
            let result = index3(act.db.clone(),act.store.clone());
            // spawn future to reactor
            Arbiter::spawn(
                result.map(|res| {
                    println!("Got result: {}", res);
                })
                .map_err(|e| {
                    println!("Actor is probably dead: {}", e);
                }));
            
        });

    }
}


fn index3( db: Data<Addr<DbExecutor>>, store: Data<AppStateWithTxt>) -> Box<dyn Future<Item = &'static str, Error = Error>> {
    use futures::future::{ok, Future};
    let sd = OrderState{
            db: db.clone(),
            store: store.clone(),
        };
    let db2= db.clone();
    let store2= store.clone();
    println!(" index3--:  "); 
    Box::new( {
        println!(" index3--:  1 "); 
        db.send(sd).from_err()
            .and_then(move |res| match res {
                Ok(list)=>{
                    println!(" index3--: {:?}",list.len()); 
                    println!(" index3--:  12 "); 

                     for res in &list { 
                         let db_addr= db2.clone();
                    
                        let send_data = ParamsToUser{
                        url: store2.webpush.send.clone(),
                        order_id: res.id.clone(), 
                        webpush: store2.webpush.clone(),
                        params: ParamsNotification{
                            notification: Notification{
                                title: "[자동] 주문 5분 미응답".to_string(),
                                body: "22".to_string(),
                                icon: "33".to_string(),
                                click_action: "44".to_string(),
                            },
                            to: res.notification_key.clone(),
                        }
                        };

                        println!(" db handler  for : "); 
                        
                        let result = fcm::to_user(send_data,  web::Data::new(Client::new().clone()), db_addr);
                        Arbiter::spawn(
                            result.map(|res| {
                                //println!("Got result: {}", res);
                                println!("Actor is  map");
                            })
                            .map_err(|e| {
                                1111
                                println!("Actor is probably dead: {}", e);
                            }));
                        /*
                        Arbiter::spawn(
                            result.map(|res| match res {
                                    Ok(msg) => {
                                        println!("===============>{:#?}",msg);
                                        ok::<_, Error>("Welcome!2 33333333")
                                    },
                                    Err(e) => {
                                        //println!("===============>{:#?}",e);
                                        ok::<_, Error>("Welcome!2 4444444")
                                    },
                                })
                            .map_err(|e| {
                                println!("Actor is probably dead: {}", e);
                                ok::<_, Error>("Welcome!2 555555555")
                        }));
                        */
                    }


                    ok::<_, Error>("Welcome!2 Welcome")
                },
                Err(e)=>{
                    println!(" index3--: errr "); 
                    ok::<_, Error>("Welcome!2 ERRR")
                }
            } )
           
    }
        
    )
    

}

fn index2() -> Box<dyn Future<Item = &'static str, Error = Error>> {
    use futures::future::{ok, Future};
    Box::new(ok::<_, Error>("Welcome!"))
}

fn no_params() -> &'static str {
    "Hello world!\r\n"
}

/*
#[derive(Message)]
#[rtype(result = "Future<Item = HttpResponse, Error = Error>")]
pub struct Update{
    pub db: Data<Addr<DbExecutor>>,
    pub store: Data<AppStateWithTxt>,
}


impl Handler<Update> for Batch {
    type Result = impl Future<Item = HttpResponse, Error = Error>;

    fn handle(&mut self, msg: OrderState, _: &mut Self::Context) -> Self::Result {
        let db = msg.db;
        let store = msg.store;
        let sd = OrderState{
                db: db.clone(),
                store: store.clone(),
            };
        db.send(sd).from_err()
        .and_then(|res| match res {
            Ok(msg) => Ok(msg),
            Err(e) => Err(e),
        })
    }
}

*/