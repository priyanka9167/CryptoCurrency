
use std::env;

use axum::http::response;
use chrono::{DateTime, Utc, NaiveDateTime, Duration};

use reqwest::{
    header::{HeaderMap, HeaderValue},
    ClientBuilder,
    Client
};
use serde_json::Value;

use crate::models::{BlockInfo, Ticker, Transaction, Block, BlockchainResponse};
use std::collections::HashMap;
use std::error::Error;


use super::models::BitcoinData;

pub type TickerResponse = HashMap<String, Ticker>;


async fn fetch_btc_to_usd_rate() -> Result<f64, Box<dyn std::error::Error>> {
    let ticker_url = "https://blockchain.info/ticker";
    let ticker: TickerResponse = reqwest::get(ticker_url).await?.json().await?;
    
    let btc_to_usd_rate = ticker.get("USD").ok_or("USD ticker not found")?.last;
    
    Ok(btc_to_usd_rate)

}

async fn fetch_block_transactions(
    block_hash: &str,
) -> Result<Transaction, Box<dyn std::error::Error>> {
    let tx_url = format!("https://blockchain.info/rawblock/{}", block_hash);
    let response = reqwest::get(&tx_url).await?;
    
    if !response.status().is_success() {
        return Err(format!("Failed to fetch transactions for block {}: {}", block_hash, response.status()).into());
    }
    let transactions: Transaction = response.json().await?;
    Ok(transactions)
}

fn calculate_total_value_usd(transaction: &[Transaction], btc_to_usd_rate: f64) -> f64 {
    let mut total_value_usd = 0.0;
    for tx_details in transaction {
        for output in &tx_details.tx {
            for tx_output in &output.out {
                // Convert Satoshis to BTC
                let value_btc = tx_output.value / 100_000_000.0;
                // Convert BTC to USD
                let value_usd = value_btc * btc_to_usd_rate;
                total_value_usd += (value_usd * 100.0).round() / 100.0;
                
            }
        }
    }
     // Round the final total value to two decimal places
    (total_value_usd * 100.0).round() / 100.0

}
   


pub async fn fetch_btc_data() -> Result<Vec<BlockInfo>, Box<dyn std::error::Error>> {
    // Fetch current BTC to USD exchange rate
    let btc_to_usd_rate = fetch_btc_to_usd_rate().await?;
   
    // Get current time in milliseconds
    let current_time_plus_30min: DateTime<Utc> = Utc::now() + Duration::minutes(15);
    let current_time_milliseconds: i64 = current_time_plus_30min.timestamp() * 1000;
    // Construct URL to fetch recent blocks
    let blocks_url = format!(
        "https://blockchain.info/blocks/{}?format=json",
        current_time_milliseconds
    );

    let response_text = reqwest::get(&blocks_url).await?.text().await?;

    // Deserialize response text into BlockchainResponse
    let parsed_response: Vec<Block> = serde_json::from_str(&response_text)?;

    
    // Process each block to create BlockInfo
    let mut block_infos = Vec::new();
    for block in parsed_response.iter().take(10) {

        let block_hash = &block.hash;
        let block_height = block.height;
        let block_time = block.time;
       
        // Fetch transaction details for the block
        let transactions = fetch_block_transactions(block_hash).await?;
        let total_transaction = transactions.n_tx;
        let total_value_usd:f64 =  calculate_total_value_usd(&[transactions], btc_to_usd_rate);
        println!("{}", total_value_usd);
        // Convert timestamp to DateTime<Utc>
        let timestamp_utc =
            DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(block_time as i64, 0), Utc);

        // Create a BlockInfo instance
        let block_info = BlockInfo {
            block_hash: block_hash.clone(),
            block_height,
            total_transaction,
            time: timestamp_utc,
            transaction_in_usd: total_value_usd,
        };

        block_infos.push(block_info);
    }

    Ok(block_infos)
}

