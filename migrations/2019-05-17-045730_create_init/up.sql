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
  shop_id UUID NOT NULL ,
  name VARCHAR NOT NULL,
  price float8,
  option_group Jsonb NOT NULL ,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  updated_at TIMESTAMP ,
  deleted_at TIMESTAMP  
);


INSERT INTO "user" ("id", "account_id", "account_password", "email", "name", "role", "created_at", "updated_at", "deleted_at") VALUES
('0290a0ad-9851-461b-af42-0313f15c9702',	'dlacjfrb123',	'$2y$12$D4eLlgcgYwfkFxF4o6hwT.rjtYutR6DjbJk4oDa/YOGESOD9ATPuW',	'asfas.sdf@abac.com',	'',	'ceo',	'2019-05-18 12:44:02.647759',	NULL,	NULL);
INSERT INTO "shop" ("id", "ceo_id", "name", "products", "created_at", "updated_at", "deleted_at") VALUES
('109b7b41-f8eb-4702-abdb-6bfb95f57072',	'0290a0ad-9851-461b-af42-0313f15c9702',	'hello coffee',	NULL,	'2019-05-18 12:44:41.184624',	NULL,	NULL);
