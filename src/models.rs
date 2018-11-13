use super::schema::trades;
use super::chrono;

#[derive(Queryable, Debug)]
pub struct Trade {
    pub id: i32,
    pub sell_order_id: i32,
    pub buy_order_id: i32,
    pub amount: f64,
    pub market_id: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name="trades"]
pub struct NewTrade<'a> {
    pub sell_order_id: i32,
    pub buy_order_id: i32,
    pub amount: f64,
    pub market_id: &'a str
}
