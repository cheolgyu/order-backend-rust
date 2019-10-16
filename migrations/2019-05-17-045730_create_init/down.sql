-- This file should undo anything in `up.sql`
DROP VIEW IF EXISTS view_shop_info;
DROP VIEW IF EXISTS view_shop_info_user;

DROP VIEW IF EXISTS view_comfind_info;
DROP TABLE IF EXISTS "user";


DROP TABLE IF EXISTS "product";
DROP TABLE IF EXISTS  "valid";
DROP TABLE IF EXISTS  "option";
DROP TABLE IF EXISTS  "option_group";
DROP TABLE IF EXISTS  "user_device";




DROP FUNCTION IF EXISTS ceo_info(u_id UUID,
                          s_id UUID,
                          p_id INTEGER);

DROP FUNCTION IF EXISTS exist_resource(u_role VARCHAR,
                          u_id VARCHAR,
                          s_id VARCHAR,
                          tg VARCHAR,
                          tg_id VARCHAR
                          );
                          


-- This file should undo anything in `up.sql`

-- This file should undo anything in `up.sql`

-- This file should undo anything in `up.sql`
DROP FUNCTION IF EXISTS auto_cancle();
DROP FUNCTION IF EXISTS come_find();

DROP TABLE IF EXISTS "shop_notification";
DROP TABLE IF EXISTS  "order";
DROP TABLE IF EXISTS  "order_detail";
DROP TABLE IF EXISTS  "fcm";
DROP TABLE IF EXISTS "shop";
DROP EXTENSION pgcrypto; 
