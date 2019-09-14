-- Your SQL goes here

-- 프로시저
CREATE FUNCTION order_state() returns table(id integer) as $$
     WITH updt AS (
      update "order" set state = 'test' 
      where state = 'req'
      and created_at <= CURRENT_TIMESTAMP+ time '00:05' 
      RETURNING id
    )
    SELECT id FROM updt;
$$ language 'sql';