table! {
    trades (id) {
        id -> Int4,
        sell_order_id -> Int4,
        buy_order_id -> Int4,
        amount -> Float8,
        market_id -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}
