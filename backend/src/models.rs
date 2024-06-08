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


// Define a struct to deserialize the JSON response
#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub name: String,
    pub height: u64,
    pub time: String,
}
