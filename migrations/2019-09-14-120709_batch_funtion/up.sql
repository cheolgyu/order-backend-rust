-- Your SQL goes here

-- 프로시저
CREATE FUNCTION order_state () returns void 
AS
  $$
BEGIN
    update "order" set state = 'test' 
    where state = 'req'
    and created_at <= CURRENT_TIMESTAMP+ time '00:05' ;
END;
$$ LANGUAGE plpgsql; 