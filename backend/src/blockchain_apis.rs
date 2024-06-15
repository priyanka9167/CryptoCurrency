
use std::env;

use chrono::{DateTime, Utc};

use reqwest::{
    header::{HeaderMap, HeaderValue},
    ClientBuilder,
    Client
};
use serde_json::Value;

use crate::models::{ApiResponse, Block, BlockchaninTransaction, EthPrice};

use super::models::BitcoinData;


pub async fn fetch_ethereum_price() -> anyhow::Result<EthPrice> {
    let api_url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
    let client = Client::new();
    let response = client.get(api_url).send().await?.json::<EthPrice>().await?;
    Ok(response)
}    




pub async fn fetch_btc_data() -> anyhow::Result<Vec<Block>> {
    let client = ClientBuilder::new()
        .build()
        .expect("Failed to create reqwest client");
    let mut headers = HeaderMap::new();

    let params = [
        ("date", Utc::now().to_rfc3339()),
        ("chain", String::from("0x1")),
    ];

    let api_key = env::var("MORALIS_API_KEY").expect("Key must set");
    // let api_key_header_value =
    //     HeaderValue::from_str(&api_key).expect("Failed to create Header value from API key");
    headers.insert(
        "X-API-Key",
        HeaderValue::from_str(&api_key).expect("Invalid API key"),
    );

    let current_time = Utc::now().to_rfc3339();
    // let current_time_header_value = HeaderValue::from_str(&current_time)
    //     .expect("Failed to create HeaderValue from current time string");
    headers.insert(
        "Date",
        HeaderValue::from_str(&current_time)
            .expect("Failed to crate HeaderValue from current time"),
    );
    let latest_block_resp = client
        .get("https://deep-index.moralis.io/api/v2/dateToBlock")
        .query(&params)
        .headers(headers.clone())
        .send()
        .await?
        .json::<Value>()
        .await?;

    let latest_block = latest_block_resp["block"].as_i64().unwrap();
    let mut block_nr_or_parent_hash = latest_block.to_string();

    println!("\n block_nr_or_parent_hash {} \n", block_nr_or_parent_hash);

    let mut blocks: Vec<Block> = vec![];

    for i in 0..2 {
        let block_resp = match client
            .get(&format!(
                "https://deep-index.moralis.io/api/v2/block/{}",
                block_nr_or_parent_hash
            ))
            .query(&params)
            .headers(headers.clone())
            .send()
            .await
        {
            Ok(response) => response,
            Err(e) => {
                println!("Error fetching block data: {}", e);
                break; // Exit loop or handle error as needed
            }
        };

        let block_resp_json = block_resp.json::<Value>().await?;

        // println!("\n block_resp_json -->> {:?}\n", block_resp_json);

        if let Some(error_msg) = block_resp_json.get("message").and_then(Value::as_str) {
            if error_msg == "No block found" {
                println!(
                    "No block found for block number/hash: {}",
                    block_nr_or_parent_hash
                );
                break; // Exit loop or handle as needed
            }
        }

        block_nr_or_parent_hash = block_resp_json["parent_hash"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("No parent hash found"))
            .map(|s| s.to_string())?;

        // Convert block number from string to i64 if necessary
        let block_number = match block_resp_json["number"] {
            Value::String(ref s) => s.parse::<i64>()?,
            Value::Number(ref n) => n
                .as_i64()
                .ok_or_else(|| anyhow::anyhow!("No block number found"))?,
            _ => return Err(anyhow::anyhow!("unexpected type for block")),
        };

        let transactions = if let Some(transactions_json) = block_resp_json["transactions"].as_array() {
            Some(
                transactions_json
                    .iter()
                    .map(|tx| {
                        let transaction = BlockchaninTransaction {
                            transaction_hash: tx["hash"]
                                .as_str()
                                .ok_or_else(|| anyhow::anyhow!("No transaction hash found"))?
                                .to_string(),
                            time: DateTime::parse_from_rfc3339(
                                tx["block_timestamp"]
                                    .as_str()
                                    .ok_or_else(|| anyhow::anyhow!("No transaction timestamp found"))?,
                            )?
                            .with_timezone(&Utc),
                            from_address: tx["from_address"]
                                .as_str()
                                .map(|v| v.to_string())
                                .or(None), // Change to Option<String>
                            to_address: tx["to_address"]
                                .as_str()
                                .map(|v| v.to_string())
                                .or(None),
                            value: tx["value"]
                                .as_str()
                                .map(|v| v.to_string())
                                .or(None), // Change to Option<String>  
                            gas: tx["gas"]
                                .as_str()
                                .map(|v| v.to_string())
                                .or(None), // Change to Option<String>
                            gas_price: tx["gas_price"]
                                .as_str()
                                .map(|v| v.to_string())
                                .or(None), // Change to Option<String>              
                            
                        };
                        Ok(transaction)
                    })
                    .collect::<Result<Vec<BlockchaninTransaction>, anyhow::Error>>()?,
            )
        } else {
            None
        };



        let block_metrics = Block {
            blockchain: "Etherum".to_string(),
            block_number,
            total_transaction: block_resp_json["transaction_count"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("No transaction found"))?
                .parse::<i64>()?,
            gas_used: block_resp_json["gas_used"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("No gas found"))?
                .to_string(),
            miner: block_resp_json["miner"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("No miner found"))?
                .to_string(),
            time: DateTime::parse_from_rfc3339(
                block_resp_json["timestamp"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("No timestamps found"))?,
            )?
            .with_timezone(&Utc),
            difficulty: block_resp_json["difficulty"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("No difficulty found"))?
                .to_string(),
            transactions,
        };

        blocks.push(block_metrics);

        println!("{:?}", block_number);
    }

    Ok(blocks)

}