-- Your SQL goes here

-- 프로시저
CREATE FUNCTION order_state() returns table(id integer,shop_id uuid,sw_token text,notification_key text) as $$
     WITH updt AS (
      update "order" set state = 'test' 
      where 
      --state = 'req'  and 
      created_at <= CURRENT_TIMESTAMP+ time '00:05' 
      RETURNING id, shop_id,sw_token
    )
    SELECT up.id,up.shop_id,up.sw_token,s.notification_key as notification_key FROM updt up left join shop s on shop_id = s.id;
$$ language 'sql';