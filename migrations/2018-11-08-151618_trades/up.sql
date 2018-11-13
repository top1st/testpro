CREATE TABLE trades (
  id SERIAL PRIMARY KEY,
  sell_order_id INT NOT NULL ,
  buy_order_id INT NOT NULL,
  amount FLOAT NOT NULL ,
  market_id VARCHAR NOT NULL ,
  created_at TIMESTAMP(0) WITHOUT TIME ZONE DEFAULT (now() AT TIME ZONE 'utc')
)