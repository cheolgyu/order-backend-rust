-- This file should undo anything in `up.sql`
DROP TABLE "user";
DROP TABLE "shop";
DROP TABLE "product";
DROP EXTENSION pgcrypto;
DROP FUNCTION IF EXISTS ceo_info (u_id UUID,
                          s_id UUID,
                          p_id INTEGER);
