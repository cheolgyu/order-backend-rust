-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS "user";
DROP TABLE IF EXISTS "shop";
DROP TABLE IF EXISTS "product";
DROP TABLE IF EXISTS  "valid";
DROP TABLE IF EXISTS  "option";
DROP TABLE IF EXISTS  "option_group";



DROP EXTENSION pgcrypto;
DROP FUNCTION IF EXISTS ceo_info (u_id UUID,
                          s_id UUID,
                          p_id INTEGER);


-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS  "order";
DROP TABLE IF EXISTS  "order_detail";
-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS  "fcm";
-- This file should undo anything in `up.sql`
DROP FUNCTION IF EXISTS auto_cancle();
DROP FUNCTION IF EXISTS come_find();
