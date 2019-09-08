-- Your SQL goes here


CREATE TABLE "fcm" (

  id SERIAL PRIMARY KEY,
  order_id INTEGER NOT NULL,
  kind VARCHAR NOT NULL,
  resp jsonb NOT NULL,
  
 
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  updated_at TIMESTAMP  DEFAULT CURRENT_TIMESTAMP ,
  deleted_at TIMESTAMP  
);