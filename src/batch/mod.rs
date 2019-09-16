pub mod model;
pub mod handler;

use actix::prelude::*;
use std::time::{Duration, Instant};
use crate::models::{AppStateWithTxt, DbExecutor};
use crate::batch::model::OrderState;
use crate::api::v1::ceo::fcm::router as fcm;
use crate::models::fcm::{ParamsToUser,ParamsNotification, Notification};
use crate::errors::ServiceError;
use futures::Future;
use futures::stream::Stream;

use actix_web::{
    
    client::Client,
    web::{self,Data, Json},
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

    fn hb(&self, ctx: &mut actix::Context<Self>) {
       
        /*
         match res2 {
            Ok(res_list) => {
                println!("{:?}",res_list.len());
                for res in &res_list {
                    
                    let send_data = ParamsToUser{
                    url: self.store.webpush.send.clone(),
                    order_id: res.id.clone(), 
                    webpush: self.store.webpush.clone(),
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
                    println!("{:?}",send_data);
                    fcm::to_user(send_data,  &self.client, &self.db);
                    /*
                    fcm::to_user(send_data,  &self.client, &self.db).map_err(|e| {
                        println!("배치 오류발생.2222222");
                        ServiceError::BadRequest("batch:fcm::to_user".into());
                    });
                    
                     */
                }

                Ok(())

            },
            Err(e) => {println!("배치 오류발생."); Ok(())},
        }
         self.db.do_send(
                OrderState{
                db: act.db.clone(),
                client: act.client.clone(),
                store: act.store.clone(),
                }
                );
     
        self.db.do_send(OrderState{
                db: self.db.clone(),
                store: self.store.clone(),
        });
           */
        use std::thread;
        println!("batch:222");
        ctx.run_later(Duration::new(1, 0), |act, ctx| {
            println!("batch:111");
            act.db.do_send(OrderState{
                    db: act.db.clone(),
                    store: act.store.clone(),
            }); 
            
            act.hb(ctx);
        });
    }
}

impl Actor for Batch {
    type Context = Context<Self>;

     fn started(&mut self, ctx: &mut Self::Context) {
       println!("I am alive!");
       self.hb(ctx);
    }
}

#[derive(Message)]
struct UpdateOrder;

/// Handle stream of TcpStream's
impl Handler<UpdateOrder> for Batch {
    /// this is response for message, which is defined by `ResponseType` trait
    /// in this case we just return unit.
    type Result = ();

    fn handle(&mut self, msg: UpdateOrder, _: &mut Context<Self>) {
        println!("I am UpdateOrder!");

    }
}