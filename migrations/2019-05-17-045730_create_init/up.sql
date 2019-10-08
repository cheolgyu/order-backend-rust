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

CREATE TABLE "user_device" (

  id  SERIAL PRIMARY KEY,
  user_id UUID NOT NULL,
  name VARCHAR NOT NULL ,
  sw_token VARCHAR NOT NULL DEFAULT ''  ,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  updated_at TIMESTAMP   DEFAULT CURRENT_TIMESTAMP ,
  deleted_at TIMESTAMP
);

CREATE TABLE "shop" (
  id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
  ceo_id UUID NOT NULL,
  name VARCHAR NOT NULL ,
  products jsonb NULL,
  notification_key VARCHAR NOT NULL DEFAULT ''  ,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  updated_at TIMESTAMP  DEFAULT CURRENT_TIMESTAMP ,
  deleted_at TIMESTAMP
);

CREATE TABLE "shop_notification" (
  id SERIAL PRIMARY KEY,
  shop_id UUID NOT NULL,
  interval  INTEGER NOT NULL DEFAULT 60,
  content VARCHAR NOT NULL DEFAULT '60초 후 기본 메시지',

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  updated_at TIMESTAMP  DEFAULT CURRENT_TIMESTAMP ,
  deleted_at TIMESTAMP
);

CREATE TABLE "product" (

  id  SERIAL PRIMARY KEY,
  shop_id UUID NOT NULL ,
  name VARCHAR NOT NULL,
  price float8 NOT NULL,
  p_price float8 NOT NULL DEFAULT 0.00,
  optg_price float8 NOT NULL DEFAULT 0.00,
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
  html_type VARCHAR NOT NULL DEFAULT 's',

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  updated_at TIMESTAMP  DEFAULT CURRENT_TIMESTAMP ,
  deleted_at TIMESTAMP
);

CREATE TABLE "option_group" (

  id  SERIAL PRIMARY KEY,
  shop_id UUID NOT NULL,
  name VARCHAR NOT NULL,
  "default" INTEGER NOT NULL,
  options INTEGER[] NOT NULL,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  updated_at TIMESTAMP  DEFAULT CURRENT_TIMESTAMP ,
  deleted_at TIMESTAMP
);

-- 조회 프로시저
CREATE FUNCTION ceo_info(u_id UUID,
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
INSERT INTO "shop" ("id", "ceo_id", "name", "products", "notification_key", "created_at", "updated_at", "deleted_at") VALUES
('109b7b41-f8eb-4702-abdb-6bfb95f57072',	'0290a0ad-9851-461b-af42-0313f15c9702',	'hello coffee',	NULL,	'APA91bG7i0xJglP9m-spquKBSfdDR8ZHTbLCzCzMI_S8Cy2ucPKMjfyiBCrqC7GafMXpByEVmc_yMxppx4L72LlMOprc0Bpbp2RrjrPp-5iKbmzmLa8_baYq9H7SpprsbFOkAaLFsvVzKoZAHYTq9e27vpGNYcAK-A',	'2019-05-18 12:44:41.184624',	'2019-05-18 12:44:41.184624',	NULL);
INSERT INTO "product" ("id", "shop_id", "name", "price", "opt_group", "created_at", "updated_at", "deleted_at") VALUES
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'ffff222',	11110222,	'{2,8,9,1}',	'2019-07-09 03:39:31.970138',	'2019-07-09 03:39:31.970138',	'2019-07-11 04:51:15.996875'),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'testest',	121212,	'{15,1}',	'2019-07-12 01:22:58.386492',	'2019-07-12 01:22:58.386492',	'2019-07-12 01:24:03.411482'),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'카페라떼',	1500,	'{1,16,19}',	'2019-07-06 23:40:22.654707',	'2019-07-06 23:40:22.654707',	NULL),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'바닐라 라뗴',	2000,	'{16,1,18,19}',	'2019-07-17 00:13:50.873908',	'2019-07-17 00:13:50.873908',	NULL),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'아메리카노',	1000,	'{1,19}',	'2019-07-17 00:14:35.034065',	'2019-07-17 00:14:35.034065',	NULL),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'아이스아메리카노',	1000,	'{1,19,20}',	'2019-07-06 23:40:22.654707',	'2019-07-06 23:40:22.654707',	NULL);

INSERT INTO "option_group" ("id", "shop_id", "name", "default","options", "created_at", "updated_at", "deleted_at") VALUES
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'컵사이즈',	'1','{1,2,3}',	'2019-06-11 04:22:30.942314',	'2019-06-11 04:22:30.942314',	NULL),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'ttt',	'1','{1}',	'2019-07-10 00:26:13.954678',	'2019-07-10 00:26:13.954678',	'2019-07-10 00:32:10.468266'),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'tttt','4',	'{3,4}',	'2019-07-10 00:26:29.706441',	'2019-07-10 00:26:29.706441',	'2019-07-10 00:32:19.263172'),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'ttttt1','2',	'{2,4,5,1,3}',	'2019-07-10 00:26:39.847708',	'2019-07-10 00:26:39.847708',	'2019-07-10 00:32:24.207723'),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'tttttt','1',	'{1,2}',	'2019-07-10 00:26:47.860271',	'2019-07-10 00:26:47.860271',	'2019-07-10 00:37:34.859891'),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'온도','4',	'{4,5,2}',	'2019-06-11 04:41:18.949586',	'2019-06-11 04:41:18.949586',	'2019-07-10 01:05:57.424717'),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'tttt1t1','1',	'{1}',	'2019-07-10 01:30:20.906798',	'2019-07-10 01:30:20.906798',	'2019-07-10 01:30:47.867608'),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	't2','1',	'{1}',	'2019-07-10 01:31:43.269405',	'2019-07-10 01:31:43.269405',	'2019-07-10 01:33:54.489075'),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	't3','2',	'{2}',	'2019-07-10 01:32:52.923078',	'2019-07-10 01:32:52.923078',	'2019-07-10 01:34:10.518524'),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	't33','1',	'{1}',	'2019-07-10 01:33:46.516458',	'2019-07-10 01:33:46.516458',	'2019-07-10 01:34:22.428307'),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'test','2',	'{2,1}',	'2019-07-10 01:24:35.132493',	'2019-07-10 01:24:35.132493',	'2019-07-10 01:36:33.041482'),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'tt1','2',	'{2}',	'2019-07-10 01:31:23.579295',	'2019-07-10 01:31:23.579295',	'2019-07-11 04:40:54.48584'),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'tt222222222','1',	'{1,2,5}',	'2019-07-10 01:29:34.972243',	'2019-07-10 01:29:34.972243',	'2019-07-11 04:52:08.007716'),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'tttt1t','1',	'{1}',	'2019-07-10 01:30:03.148586',	'2019-07-10 01:30:03.148586',	'2019-07-11 04:52:09.060436'),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'tt111','1',	'{1,4,5}',	'2019-07-12 01:22:35.653354',	'2019-07-12 01:22:35.653354',	'2019-07-12 01:24:08.027957'),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'온도','4',	'{4,5}',	'2019-06-11 04:22:30.942314',	'2019-06-11 04:22:30.942314',	NULL),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'초코추가',	'9','{9}',	'2019-07-17 00:10:26.476131',	'2019-07-17 00:10:26.476131',	NULL),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'바닐라추가','10',	'{10}',	'2019-07-17 00:10:37.746736',	'2019-07-17 00:10:37.746736',	NULL),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'샷추가',	'11','{11}',	'2019-07-17 00:11:33.620205',	'2019-07-17 00:11:33.620205',	NULL),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'프리미엄샷 추가','12',	'{12}',	'2019-07-17 00:11:46.146837',	'2019-07-17 00:11:46.146837',	NULL);

INSERT INTO "option" ("id", "shop_id", "name", "price", "created_at", "updated_at", "deleted_at", "html_type") VALUES
(DEFAULT,'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'레귤러',	0,	'2019-06-11 04:22:55.019004',	'2019-06-11 04:22:55.019004',	NULL,	's'),
(DEFAULT,'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'그란데',	500,	'2019-06-11 04:22:59.547769',	'2019-06-11 04:22:59.547769',	NULL,	's'),
(DEFAULT,'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'벤티',	1500,	'2019-06-11 04:23:03.244713',	'2019-06-11 04:23:03.244713',	NULL,	's'),
(DEFAULT,'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'차가운',	500,	'2019-06-11 04:23:06.768563',	'2019-06-11 04:23:06.768563',	NULL,	's'),
(DEFAULT,'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'따뜻한',	0,	'2019-06-11 04:23:10.453339',	'2019-06-11 04:23:10.453339',	NULL,	's'),
(DEFAULT,'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'111',	1122322,	'2019-07-10 00:50:46.802188',	'2019-07-10 00:50:46.802188',	'2019-07-11 04:57:22.237488',	's'),
(DEFAULT,'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'ttt1',	1212,	'2019-07-10 00:36:41.130688',	'2019-07-10 00:36:41.130688',	'2019-07-11 04:58:44.031873',	's'),
(DEFAULT,'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'tttt',	12,	'2019-07-12 01:23:32.049281',	'2019-07-12 01:23:32.049281',	'2019-07-12 01:23:55.428754',	's'),
(DEFAULT,'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'초코크림',	500,	'2019-07-17 00:09:58.403791',	'2019-07-17 00:09:58.403791',	NULL,	's'),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'바닐라크림',	500,	'2019-07-17 00:10:09.53551',	'2019-07-17 00:10:09.53551',	NULL,	's'),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'일반샷',	0,	'2019-07-19 07:45:22.34306',	'2019-07-19 07:45:22.34306',	NULL,	's'),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'프리미엄샷',	500,	'2019-07-19 07:45:35.201037',	'2019-07-19 07:45:35.201037',	NULL,	's'),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'머그컵 (매장용)',	0,	'2019-07-20 05:03:25.730931',	'2019-07-20 05:03:25.730931',	NULL,	's'),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'텀블러 (고객용)',	-300,	'2019-07-20 05:03:45.191222',	'2019-07-20 05:03:45.191222',	NULL,	's'),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'일회용 (테이크아웃용)',	0,	'2019-07-20 05:04:03.991165',	'2019-07-20 05:04:03.991165',	NULL,	's'),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'일반샷 추가',	500,	'2019-07-17 00:11:00.605369',	'2019-07-17 00:11:00.605369',	'2019-07-20 05:57:42.217618',	's'),
(DEFAULT,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	'프리미엄 추가',	500,	'2019-07-17 00:11:09.611149',	'2019-07-17 00:11:09.611149',	'2019-07-20 05:57:45.26888',	's');

INSERT INTO "shop_notification" ("id", "shop_id", "interval", "content", "created_at", "updated_at", "deleted_at") VALUES
(1,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	60,	'60 초후 기본 메시지',	'2019-09-24 00:41:01.722745',	'2019-09-24 00:41:01.722745',	NULL),
(2,	'109b7b41-f8eb-4702-abdb-6bfb95f57072',	120,	'120 초 후 기본 메시지',	'2019-09-24 00:41:01.722745',	'2019-09-24 00:41:01.722745',	NULL);

SELECT  p.prosrc
FROM    pg_catalog.pg_namespace n
JOIN    pg_catalog.pg_proc p
ON      p.pronamespace = n.oid
WHERE   n.nspname = 'public'
and p.proname ='ceo_info';



-- Your SQL goes here
CREATE TABLE "order_detail" (

  id SERIAL PRIMARY KEY,
  order_id INTEGER NOT NULL,
  shop_id UUID NOT NULL,
  state INTEGER NOT NULL,
  txt jsonb NOT NULL,
  req_session_id jsonb NOT NULL,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  updated_at TIMESTAMP  DEFAULT CURRENT_TIMESTAMP ,
  deleted_at TIMESTAMP
);
COMMENT ON COLUMN "order_detail"."state" IS '0: 거절, 1:승인, 2: 수령';

CREATE TABLE "order" (

  id SERIAL PRIMARY KEY,
  shop_id UUID NOT NULL,
  state INTEGER NOT NULL DEFAULT 0,
  price float8 NOT NULL,
  cnt  INTEGER NOT NULL DEFAULT 0,
  products jsonb NOT NULL,
  sw_token VARCHAR NOT NULL,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  updated_at TIMESTAMP  DEFAULT CURRENT_TIMESTAMP ,
  deleted_at TIMESTAMP
);
COMMENT ON COLUMN "order"."state" IS '-2: 미응답거절, -1: 거절, 1: 대기, 2: 수락, 3: 수령 ';
-- Your SQL goes here

/*
- 사장님 디바이스 추가시
- 사장님에게 발신
    고객이 주문시
    시스템:배치가 주문자동취소시
- 고객에게 발신 -
    주문상세-사장님이 주문 승인시
    주문상세-사장님이 주문 거절시
    주문상세-사장님이 주문 수령시
    사장님이 주문 추가 수령시
    시스템:배치가 주문 자동취소시
    시스템:배치가 주문 자동 수령요청시 최대 2회 5분간격.
*/

CREATE TABLE "fcm" (

  id SERIAL PRIMARY KEY,
  "to" VARCHAR NOT NULL,
  order_id INTEGER NOT NULL,
  order_detail_id INTEGER NOT NULL DEFAULT '0' ,
  shop_notification_id INTEGER NOT NULL DEFAULT '0' ,
  order_detail_state  INTEGER NOT NULL,
  trigger  VARCHAR NOT NULL DEFAULT ''  ,
  req jsonb NOT NULL,
  resp jsonb NOT NULL,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
  updated_at TIMESTAMP  DEFAULT CURRENT_TIMESTAMP ,
  deleted_at TIMESTAMP
);


-- Your SQL goes here

-- 구매한 주문의 자동 취소 프로시저
CREATE FUNCTION auto_cancle() returns table(id integer,shop_id uuid,sw_token text,notification_key text) as $$
     WITH updt AS (
      update "order" set state = -2
      where
      state = 1  and
      Date_trunc('minute', CURRENT_TIMESTAMP) = Date_trunc('minute', created_at+ time '00:05' )
      RETURNING id, shop_id,sw_token
    )
    SELECT up.id,up.shop_id,up.sw_token,s.notification_key as notification_key FROM updt up left join shop s on shop_id = s.id;
$$ language 'sql';


CREATE VIEW view_comfind_info AS
SELECT    od.order_id AS order_id ,
          od.id       AS order_detail_id,
          od.shop_id  AS shop_id,
          sn.id as shop_notification_id,
          o.sw_token  AS to ,
          sn.content  AS content,
          s.NAME      AS shop_name
FROM      "order_detail" od
LEFT JOIN "order" o
ON        od.order_id = o.id
LEFT JOIN shop s
ON        od.shop_id = s.id
LEFT JOIN shop_notification sn
ON        od.shop_id = sn.shop_id
AND       sn.id NOT IN
          (
                 SELECT f.shop_notification_id
                 FROM   fcm f
                 WHERE  f.order_detail_state=2
                 AND    f.TRIGGER= 'batch::comfind' )
WHERE     od.state = 2
AND       Date_trunc('minute', CURRENT_TIMESTAMP) = date_trunc('minute', (od.created_at + interval '1 seconds'*sn.interval))
;
-- 구매한 주문의 사장님의 제조완료에 대한 배치 프로시저
CREATE FUNCTION come_find()
returns TABLE(order_id integer,order_detail_id integer,shop_id uuid,shop_notification_id integer,"to" text,content text,shop_name text) AS $$
SELECT *
FROM   view_comfind_info $$ language 'sql';


CREATE VIEW view_shop_info AS
SELECT s_id, 
       Json_build_object('s_id', s_id, 's_nm', s_nm, 'p', Json_agg( 
       Json_build_object('p_id', p_id, 'p_nm', p_nm, 'price', price, 'p_price' 
                                                          , p_price 
       , 'optg_price', optg_price, 'og', og))) AS s_info 
FROM   (SELECT s_id, 
               s_nm, 
               p_id, 
               p_nm, 
               price, 
               p_price, 
               optg_price, 
               opt_group, 
               Json_agg(Json_build_object('og_id', og_id, 'og_nm', og_nm, 'og_default',og_default,'o', o 
                        )) AS 
                      og 
        FROM   (SELECT s.id 
                               AS s_id 
                               , 
                       s.name 
                               AS s_nm, 
                       p.id 
                               AS p_id, 
                       p.name 
                               AS p_nm, 
                       p.price 
                               AS price, 
                       p.p_price 
                               AS p_price, 
                       p.optg_price 
                               AS optg_price, 
                       p.opt_group, 
                       og.id 
                               AS og_id, 
                       og.name 
                               AS og_nm, 
                       og.default 
                               AS og_default,
                       og.options, 
                       Json_agg(Json_build_object('o_id', o.id, 'o_nm', o.name, 
                                'o_price', 
                                o.price, 
                                         'o_html_type', o.html_type, 
                                'og_default', 
                                og.default)) 
                               AS o 
                FROM   shop s 
                       left join product p 
                              ON s.id = p.shop_id 
                                 AND p.deleted_at IS NULL 
                       left join option_group og 
                              ON og.id = ANY ( p.opt_group ) 
                       left join OPTION o 
                              ON o.id = ANY ( og.options ) 
                GROUP  BY s.id, 
                          s.name, 
                          p.id, 
                          p.name, 
                          p.opt_group, 
                          og.id, 
                          og.name, 
                          og.options) t 
        GROUP  BY s_id, 
                  s_nm, 
                  p_id, 
                  p_nm, 
                  price, 
                  p_price, 
                  optg_price, 
                  opt_group)b 
GROUP  BY s_id, 
          s_nm 
;

