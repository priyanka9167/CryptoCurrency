use crate::models::{Block, BlockDataFromDB, BlockchaninTransaction, TransactionDataFromDB};

use super::models::BitcoinData;

use anyhow::{Ok, Error as AnyhowError};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use reqwest::Error;
use sqlx::{PgPool, Row};

pub async fn insert_bitcoin_data(pool: &PgPool, bitcoin_data: BitcoinData) -> anyhow::Result<()> {
    let query_str = r#"
        INSERT INTO bitcoin_data ( id, name, bitcoin_height, timestamp) 
        VALUES ( $1, $2, $3, $4 )
        ON CONFLICT(id) 
        DO UPDATE SET
        bitcoin_height = $3,
        timestamp = $4
    "#;

    let res = sqlx::query(query_str)
        .bind(1)
        .bind(bitcoin_data.name)
        .bind(bitcoin_data.bitcoin_height)
        .bind(bitcoin_data.timestamp)
        // .bind(12345)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_data_by_timestamp(pool: &PgPool) -> anyhow::Result<Vec<BlockDataFromDB>, AnyhowError> {
    let start_time = Utc::now() - chrono::Duration::minutes(5);
    
    let query_str = r#"
            SELECT b.blockchain, b.block_number, b.total_transactions, b.gas_used, b.miner, b.block_time, b.difficulty,
                   t.transaction_hash, t.transaction_time, t.from_address, t.to_address, t.value, t.gas, t.gas_price
            FROM blocks b
            JOIN transactions t ON b.id = t.block_id
            WHERE b.block_time >= $1
        "#;
    let rows = sqlx::query(query_str).bind(start_time).fetch_all(pool).await?;

    let mut block_data = vec![];
    let mut current_block: Option<BlockDataFromDB> = None;
    
    for row in rows{
        let blockchain = row.get("blockchain");
        let block_number:i64 = row.get("block_number");
        let total_transaction: i64 = row.get("total_transactions");
        let gas_used: String = row.get("gas_used");
        let miner: String = row.get("miner");
        let block_time: NaiveDateTime = row.try_get("block_time")?;
        let time: DateTime<Utc> = Utc.from_utc_datetime(&block_time); 
        let difficulty: String = row.get("difficulty");

        if current_block.is_none() || current_block.as_ref().unwrap().block_number != block_number{

            if let Some(block) = current_block{
                block_data.push(block)
            }

            current_block = Some(BlockDataFromDB {
                blockchain,
                block_number,
                total_transaction,
                gas_used,
                miner,
                time,
                difficulty,
                transactions: vec![],
            });
        }

        if let Some(ref mut block) = current_block{
            block.transactions.push(TransactionDataFromDB{
                transaction_hash: row.get("transaction_hash"),
                time: Utc.from_utc_datetime(&row.try_get::<NaiveDateTime, _>("transaction_time")?), // Convert transaction_time directly
                from_address: row.get("from_address"),
                to_address: row.get("to_address"),
                value: row.get("value"),
                gas: row.get("gas"),
                gas_price: row.get("gas_price"),
            })
        }
    }

    if let Some(block) = current_block{
        block_data.push(block)
    }

    Ok(block_data)
}

pub async fn insert_blocks(pool: &PgPool, blocks: Vec<Block>) -> anyhow::Result<()> {
    // let query_str = r#"
    //     INSERT INTO blocks ( blockchain, block_number, total_transactions, gas_used, miner, block_time, difficulty)
    //     VALUES ( $1, $2, $3, $4, $5, $6, $7 )
    // "#;

    for block in &blocks {
        let query_str = r#"
        INSERT INTO blocks (blockchain, block_number, total_transactions, gas_used, miner, block_time, difficulty)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT(blockchain, block_number)
        DO UPDATE SET
            total_transactions = EXCLUDED.total_transactions,
            gas_used = EXCLUDED.gas_used,
            miner = EXCLUDED.miner,
            block_time = EXCLUDED.block_time,
            difficulty = EXCLUDED.difficulty
        RETURNING id
    "#;

        let res = sqlx::query(query_str)
            .bind(block.blockchain.to_owned())
            .bind(block.block_number)
            .bind(block.total_transaction)
            .bind(block.gas_used.to_owned())
            .bind(block.miner.to_owned())
            .bind(block.time)
            .bind(block.difficulty.to_owned())
            // .bind(12345)
            .fetch_one(pool)
            .await?;
        let inserted_id: i32 = res.get("id");
        println!("Inserted or updated row ID: {:?}", inserted_id);

        match block.transactions.to_owned() {
            Some(txns) => {
                let _ = insert_transaction(pool, inserted_id, txns).await;
            }
            None => (),
        }
    }

    Ok(())
}

pub async fn insert_transaction(
    pool: &PgPool,
    block_id: i32,
    transactions: Vec<BlockchaninTransaction>,
) -> anyhow::Result<()> {
    for transaction in &transactions {
        sqlx::query(
            r#"
            INSERT INTO transactions (block_id, transaction_hash, transaction_time, from_address, to_address, value, gas, gas_price)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#
        )
        .bind(block_id)
        .bind(transaction.transaction_hash.to_owned())
        .bind(transaction.time)
        .bind(transaction.from_address.to_owned())
        .bind(transaction.to_address.to_owned())
        .bind(transaction.value.to_owned())
        .bind(transaction.gas.to_owned())
        .bind(transaction.gas_price.to_owned())
        .execute(pool)
        .await?;
    }

    println!("Inserted {} transactions.", transactions.len());

    Ok(())
}
