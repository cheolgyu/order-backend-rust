-- Your SQL goes here
CREATE TABLE "user" (

  id SERIAL PRIMARY KEY,
  account_id VARCHAR NOT NULL,
  account_password VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  name VARCHAR NOT NULL ,
  role VARCHAR NOT NULL DEFAULT 'ceo' ,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  updated_at TIMESTAMP ,
  deleted_at TIMESTAMP  
)