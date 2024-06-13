use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Clone)]
pub struct AppState {
    pub pg_pool: PgPool
}


#[derive(Debug, FromRow, Serialize)]
pub struct BitcoinData {
    pub name: String,
    pub bitcoin_height: i32,
    pub timestamp: i32,
}

#[derive(Debug,FromRow)]
pub struct BitcoinDataFromDB {
    pub name: String,
    pub bitcoin_height: u64,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block{
    pub blockchain: String,
    pub block_number: i64,
    pub total_transaction: i64,
    pub gas_used: String,
    pub miner: String,
    pub time: DateTime<Utc>,
    pub difficulty: String,
    pub hashrate: String,
    pub transactions: Option<Vec<Transaction>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction{
    pub transaction_hash: String,
    pub time: DateTime<Utc>,
    pub from_address: String,
    pub to_address: String,
    pub value: String,
    pub gas_used: String,
    pub gas_price: String,
}


// Define a struct to deserialize the JSON response
#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub name: String,
    pub height: u64,
    pub time: String,
}
