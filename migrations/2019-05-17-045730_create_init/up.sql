-- Your SQL goes here
CREATE TABLE "user" (

  id UUID PRIMARY KEY,
  account_id VARCHAR NOT NULL,
  account_password VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  name VARCHAR NOT NULL ,
  role VARCHAR NOT NULL DEFAULT 'ceo' ,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  updated_at TIMESTAMP ,
  deleted_at TIMESTAMP  
);

CREATE TABLE "shop" (

  id UUID PRIMARY KEY,
  ceo_id UUID NOT NULL,
  name VARCHAR NOT NULL ,
  products jsonb NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  updated_at TIMESTAMP ,
  deleted_at TIMESTAMP  
);

CREATE TABLE "dict" (

  id BIGSERIAL PRIMARY KEY,
  kor VARCHAR NOT NULL,
  eng VARCHAR NOT NULL,
  kind VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  updated_at TIMESTAMP ,
  deleted_at TIMESTAMP  
);

CREATE TABLE "product" (

  id  SERIAL PRIMARY KEY,
  shop_id UUID NOT NULL references shop (id),
  name VARCHAR NOT NULL,
  price float8,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  updated_at TIMESTAMP ,
  deleted_at TIMESTAMP  
);

CREATE TABLE "option_group" (

  id  SERIAL PRIMARY KEY,
  product_id int NOT NULL ,
  name VARCHAR NOT NULL ,
  value_type  VARCHAR NOT NULL ,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  updated_at TIMESTAMP ,
  deleted_at TIMESTAMP  
);

CREATE TABLE "option" (

  id SERIAL PRIMARY KEY,
  option_group_id int NOT NULL ,
  name VARCHAR NOT NULL ,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  updated_at TIMESTAMP ,
  deleted_at TIMESTAMP  
);

