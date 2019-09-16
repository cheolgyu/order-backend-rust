pub mod model;
pub mod handler;

use actix::prelude::*;
use std::time::{Duration, Instant};
use crate::models::{AppStateWithTxt, DbExecutor};
use crate::batch::model::OrderState;
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

    fn hb(&self, ctx: &mut actix::Context<Self>) {

        use std::thread;
        println!("batch:222");
        ctx.run_interval(Duration::new(1, 0), move |act, ctx| {
            println!("batch:111");
            let sd = OrderState{
                    db: act.db.clone(),
                    store: act.store.clone(),
                };
            act.db.do_send(sd);
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
#[rtype(result = "Result<Vec<OrderStateRes>, ServiceError>")]
pub struct Update;


impl Handler<Update> for Batch {
    type Result = Result<Vec<OrderStateRes>, ServiceError>;

    fn handle(&mut self, msg: OrderState, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;
      //  println!(" db"); 
        let list  = sql_query("select * from order_state() ").get_results::<OrderStateRes>(conn)?;
       println!(" db handler: {:?}",list.len()); 
         for res in &list { 
                    
          let send_data = ParamsToUser{
          url: msg.store.webpush.send.clone(),
          order_id: res.id.clone(), 
          webpush: msg.store.webpush.clone(),
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
          fcm::to_user(send_data,  web::Data::new(Client::new().clone()), msg.db.clone());

        /*

        Client::new()
        .post(send_data.url.clone())
        .header(CONTENT_TYPE, "application/json")
        .header("Authorization", send_data.webpush.key.clone())
        .send_json(&send_data.params)
        .map_err(|e| {
            println!("batch:666666666666");
            eprintln!("{:?}",e);
            panic!("{:?}", e)
        })
        .and_then(|response| {
             println!("batch:5555555555555");
            let res = response
                .from_err()
                .fold(BytesMut::new(), |mut acc, chunk| {
                    acc.extend_from_slice(&chunk);
                    println!("batch:99");
                    Ok::<_, ServiceError>(acc)
                })
                .map(|body| {
                    println!("batch:10");
                    let body: ToUserResp = serde_json::from_slice(&body).expect("to_user body 변환 오류");
                    body
                });
            res 
        });
        */

         // println!("{:?}",send_data);
          //fcm::to_user(send_data,  web::Data::new(Client::new().clone()), msg.db.clone());
        }
        
        Ok(list)
    }
}

