-- Your SQL goes here
create extension pgcrypto;

CREATE TABLE "user" (

  id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
  account_id VARCHAR NOT NULL,
  account_password text NOT NULL,
  email VARCHAR NOT NULL,
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
  price float8,
  option_group Jsonb NOT NULL ,

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
'$2y$12$D4eLlgcgYwfkFxF4o6hwT.rjtYutR6DjbJk4oDa/YOGESOD9ATPuW'
,	crypt('dlacjfrb123!@#',gen_salt('bf')),	'',	'ceo',	'2019-05-18 12:44:02.647759',	'2019-05-18 12:44:02.647759',	NULL);
INSERT INTO "shop" ("id", "ceo_id", "name", "products", "created_at", "updated_at", "deleted_at") VALUES
('109b7b41-f8eb-4702-abdb-6bfb95f57072',	'0290a0ad-9851-461b-af42-0313f15c9702',	'hello coffee',	NULL,	'2019-05-18 12:44:41.184624',	'2019-05-18 12:44:41.184624',	NULL);



SELECT  p.prosrc
FROM    pg_catalog.pg_namespace n
JOIN    pg_catalog.pg_proc p
ON      p.pronamespace = n.oid
WHERE   n.nspname = 'public'
and p.proname ='ceo_info';
