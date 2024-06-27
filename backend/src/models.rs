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

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockDataFromDB{
    pub blockchain: String,
    pub block_number: i64,
    pub total_transaction: i64,
    pub gas_used: String,
    pub miner: String,
    pub time: DateTime<Utc>,
    pub difficulty: String,
    pub transactions: Vec<TransactionDataFromDB>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionDataFromDB{
    pub transaction_hash: String,
    pub time: DateTime<Utc>,
    pub from_address: Option<String>, // Change to Option<String>
    pub to_address: Option<String>, // Change to Option<String>
    pub value: Option<String>,
    pub gas: Option<String>,
    pub gas_price: Option<String>,
}


#[derive(Debug,FromRow)]
pub struct BitcoinDataFromDB {
    pub name: String,
    pub bitcoin_height: u64,
    pub timestamp: i64,
}


#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub hash: String,
    pub n_tx: u32,
    pub tx: Vec<TransactionDetails>,
    // Add more fields as needed
}

#[derive(Debug, Deserialize)]
pub struct TransactionDetails {
    pub out: Vec<TransactionOutput>,
    // Add more fields as needed
}

#[derive(Debug, Deserialize)]
pub struct TransactionOutput {
    pub value: f64,
    // Add more fields as needed
}


#[derive(Debug, Deserialize)]
pub struct Ticker {
    #[serde(rename = "15m")]
   pub fifteen_min: f64,
    pub last: f64,
   pub  buy: f64,
    pub sell: f64,
   pub  symbol: String,
}




#[derive(Debug, Deserialize)]
pub struct Block {
   pub hash: String,
    pub height: i64,
    pub time: u64,
    pub block_index: u64
    // Add more fields as needed
}

#[derive(Debug, Deserialize)]
pub struct BlockchainResponse {
    pub blocks: Vec<Block>,
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockInfo {
    
    pub block_hash: String,
    pub block_height: i64,
    pub total_transaction: u32,
    pub time: DateTime<Utc>,
    pub transaction_in_usd: f64,
    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockchaninTransaction{
    pub transaction_hash: String,
    pub time: DateTime<Utc>,
    pub from_address: Option<String>, // Change to Option<String>
    pub to_address: Option<String>, // Change to Option<String>
    pub value: Option<String>,
    pub gas: Option<String>,
    pub gas_price: Option<String>,
}


// Define a struct to deserialize the JSON response
#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub name: String,
    pub height: u64,
    pub time: String,
}
