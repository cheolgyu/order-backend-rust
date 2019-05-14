-- Your SQL goes here
CREATE TABLE "user" (

  id SERIAL PRIMARY KEY,
  uuid UUID  NOT NULL ,
  account_id VARCHAR NOT NULL,
  account_password VARCHAR NOT NULL,
  name VARCHAR NOT NULL ,
  role VARCHAR NOT NULL DEFAULT 'ceo' ,

  created_at TIMESTAMP DEFAULT NOW() ,
  updated_at TIMESTAMP ,
  deleted_at TIMESTAMP 
)