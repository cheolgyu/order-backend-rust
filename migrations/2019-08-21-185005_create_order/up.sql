-- Your SQL goes here
CREATE TABLE "order_detail" (

  id SERIAL PRIMARY KEY,
  order_id INTEGER NOT NULL,
  shop_id UUID NOT NULL,
  state VARCHAR NOT NULL,
  txt jsonb NOT NULL,
  req_session_id jsonb NOT NULL,
  
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  updated_at TIMESTAMP  DEFAULT CURRENT_TIMESTAMP ,
  deleted_at TIMESTAMP  
);

CREATE TABLE "order" (

  id SERIAL PRIMARY KEY,
  shop_id UUID NOT NULL,
  state VARCHAR NOT NULL,
  price float8 NOT NULL,
  
  products jsonb NOT NULL,
  sw_token VARCHAR NOT NULL,
  
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  updated_at TIMESTAMP  DEFAULT CURRENT_TIMESTAMP ,
  deleted_at TIMESTAMP  
); 