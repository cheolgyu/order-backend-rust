-- Your SQL goes here
create extension pgcrypto;

CREATE TABLE "valid" (

  id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
  user_id UUID NOT NULL,
  kind VARCHAR NOT NULL,
  kind_value VARCHAR NOT NULL,
  code VARCHAR  NOT NULL,
  req VARCHAR ,
  res VARCHAR ,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  valid_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  updated_at TIMESTAMP   DEFAULT CURRENT_TIMESTAMP ,
  deleted_at TIMESTAMP  
);

CREATE TABLE "user" (

  id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
  account_id VARCHAR NOT NULL,
  account_password text NOT NULL,
  email VARCHAR NOT NULL,
  valid_email bool NOT NULL DEFAULT 'false' ,
  phone VARCHAR  NULL,
  name VARCHAR NOT NULL ,
  role VARCHAR NOT NULL DEFAULT 'ceo' ,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  updated_at TIMESTAMP   DEFAULT CURRENT_TIMESTAMP ,
  deleted_at TIMESTAMP  
);


CREATE TABLE "shop" (

  id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
  ceo_id UUID NOT NULL,
  name VARCHAR NOT NULL ,
  products jsonb NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  updated_at TIMESTAMP  DEFAULT CURRENT_TIMESTAMP ,
  deleted_at TIMESTAMP  
);

CREATE TABLE "product" (

  id  SERIAL PRIMARY KEY,
  shop_id UUID NOT NULL ,
  name VARCHAR NOT NULL,
  price float8 NOT NULL,
  opt_group INTEGER[]  Not NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  updated_at TIMESTAMP  DEFAULT CURRENT_TIMESTAMP ,
  deleted_at TIMESTAMP  
);

CREATE TABLE "option" (

  id  SERIAL PRIMARY KEY,
  shop_id UUID NOT NULL,
  name VARCHAR NOT NULL,
  price float8 NOT NULL,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  updated_at TIMESTAMP  DEFAULT CURRENT_TIMESTAMP ,
  deleted_at TIMESTAMP  
);

CREATE TABLE "option_group" (

  id  SERIAL PRIMARY KEY,
  shop_id UUID NOT NULL,
  name VARCHAR NOT NULL,
  options INTEGER[] NOT NULL,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  updated_at TIMESTAMP  DEFAULT CURRENT_TIMESTAMP ,
  deleted_at TIMESTAMP  
);

-- 조회 프로시저
CREATE FUNCTION ceo_info (u_id UUID,
                          s_id UUID,
                          p_id INTEGER) returns SETOF int
AS
  $$
BEGIN
    IF (p_id IS NULL) THEN
        IF (s_id  IS NULL) THEN
           RETURN QUERY 
           
           SELECT count(id)::int
            FROM   "user" u
            WHERE  u.id = u_id ;
        ELSE
          RETURN QUERY 
           SELECT count(s.id)::int FROM ( SELECT u.id FROM "user" as u WHERE u.id = u_id)
            as u left join shop s ON u.id = s.ceo_id AND s.id = s_id ;
        END IF ;
    ELSE
        RETURN QUERY 
           SELECT count(p.id)::int
            FROM (
                    SELECT u.id
                    FROM   "user" u
                    WHERE  u.id = u_id
                ) as u
                left join shop s
                ON u.id = s.ceo_id and   s.id = s_id
                left join product as p
                on s.id = p.shop_id and p.id =p_id 
                ;
    END IF ;
    
END;
$$ LANGUAGE plpgsql; 




INSERT INTO "user" ("id", "account_id", "account_password", "email", "name", "role", "created_at", "updated_at", "deleted_at") VALUES
('0290a0ad-9851-461b-af42-0313f15c9702',	'dlacjfrb123',	
crypt('dlacjfrb123!@#',gen_salt('bf'))
,	'cjfrb119@hanmail.net',	'',	'ceo',	'2019-05-18 12:44:02.647759',	'2019-05-18 12:44:02.647759',	NULL);
INSERT INTO "shop" ("id", "ceo_id", "name", "products", "created_at", "updated_at", "deleted_at") VALUES
('109b7b41-f8eb-4702-abdb-6bfb95f57072',	'0290a0ad-9851-461b-af42-0313f15c9702',	'hello coffee',	NULL,	'2019-05-18 12:44:41.184624',	'2019-05-18 12:44:41.184624',	NULL);

INSERT INTO "option_group" ("id", "shop_id", "name", "options", "created_at", "updated_at", "deleted_at") VALUES
(1,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'2121',	'{1,2,3,4,5}',	'2019-06-11 04:22:30.942314',	'2019-06-11 04:22:30.942314',	NULL),
(2,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'sdafasdf',	'{1,2,3,4,5}',	'2019-06-11 04:41:18.949586',	'2019-06-11 04:41:18.949586',	NULL);
INSERT INTO "option" ("id", "shop_id", "name", "price", "created_at", "updated_at", "deleted_at") VALUES
(1,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'1',	1,	'2019-06-11 04:22:55.019004',	'2019-06-11 04:22:55.019004',	NULL),
(2,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'2',	2,	'2019-06-11 04:22:59.547769',	'2019-06-11 04:22:59.547769',	NULL),
(3,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'3',	3,	'2019-06-11 04:23:03.244713',	'2019-06-11 04:23:03.244713',	NULL),
(4,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'4',	4,	'2019-06-11 04:23:06.768563',	'2019-06-11 04:23:06.768563',	NULL),
(5,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'5',	5,	'2019-06-11 04:23:10.453339',	'2019-06-11 04:23:10.453339',	NULL);


SELECT  p.prosrc
FROM    pg_catalog.pg_namespace n
JOIN    pg_catalog.pg_proc p
ON      p.pronamespace = n.oid
WHERE   n.nspname = 'public'
and p.proname ='ceo_info';
