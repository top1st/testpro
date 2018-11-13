#[macro_use] extern crate log;
#[macro_use] extern crate diesel;

extern crate chrono;
extern crate simplelog;
extern crate postgres;
extern crate dotenv;
extern crate grpcio;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

use self::models::{Trade, NewTrade};
use simplelog::*;

use std::fs::File;

pub fn create_trade(conn: &PgConnection, sell_order_id: i32, buy_order_id: i32, amount: f64, market_id: &str) -> Trade {
    use schema::trades;

    let new_trade = NewTrade {
        sell_order_id,
        buy_order_id,
        amount,
        market_id
    };

    diesel::insert_into(trades::table)
        .values(&new_trade)
        .get_result(conn)
        .expect("Error saving new trade")

}


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Errror connecting to  {}", database_url))
}

fn main() {

    use schema::trades::dsl::*;
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default()).unwrap(),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();

    let connection = establish_connection();

    let results = trades
        .limit(5)
        .load::<Trade>(&connection)
        .expect("Error loading trades");

    println!("Displaying {} posts", results.len());

    for trade in results {
        println!("{}", trade.id);
    }

    let mkid = "btcusd";

    let trade = create_trade(&connection, 20, 10, 30.1, mkid);

    println!("{}, {:?}", mkid, trade);



    error!("Bright red error");
    info!("This only appears in the log file");
    debug!("This level is currently not enabled for any logger");
}

