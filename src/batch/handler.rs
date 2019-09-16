use crate::models::{AppStateWithTxt, DbExecutor};
use crate::errors::ServiceError;
use actix::Handler;
use crate::batch::model::{OrderState,OrderStateRes};
use diesel;

use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::{Integer, Text};

use crate::api::v1::ceo::fcm::router as fcm;
use crate::models::fcm::{ParamsToUser,ParamsNotification, Notification, ToUserResp};
use actix_web::{
    web,
    client::Client,
  };
use futures::future::Future;

impl Handler<OrderState> for DbExecutor {
    type Result = Result<Vec<OrderStateRes>, ServiceError>;

    fn handle(&mut self, msg: OrderState, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get().expect("batch_db conn err");
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




          /////////////////////
          /// 
          /// 
          use actix_web::{
              client::Client,
              http::header::CONTENT_TYPE,
              web::{BytesMut, Data},
              ResponseError
          };
          use futures::Future;
          use futures::stream::Stream;



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

         // println!("{:?}",send_data);
          //fcm::to_user(send_data,  web::Data::new(Client::new().clone()), msg.db.clone());
        }
        Ok(list)
    }
}
