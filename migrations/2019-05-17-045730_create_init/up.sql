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

CREATE TABLE "product" (

  id UUID PRIMARY KEY,
  shop_id UUID NOT NULL,
  name VARCHAR NOT NULL ,
  option_groups jsonb NULL,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  updated_at TIMESTAMP ,
  deleted_at TIMESTAMP  
);

CREATE TABLE "option_group" (

  id UUID PRIMARY KEY,
  name VARCHAR NOT NULL ,
  value_type  VARCHAR NOT NULL DEFAULT 'select' ,
  options jsonb NULL,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  updated_at TIMESTAMP ,
  deleted_at TIMESTAMP  
);

CREATE TABLE "option" (

  id UUID PRIMARY KEY,
  name VARCHAR NOT NULL ,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  updated_at TIMESTAMP ,
  deleted_at TIMESTAMP  
);

