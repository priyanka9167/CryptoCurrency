use crate::models::{BlockInfo};


use anyhow::{Ok, Result,Error as AnyhowError};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use reqwest::Error;
use sqlx::{PgPool, Row};

pub async fn insert_block_info(pool: &PgPool, block_info:  Vec<BlockInfo>) -> Result<()> {

    for block in &block_info { 
    let query_str = r#"
        INSERT INTO blocks (block_hash, block_height, total_transaction, time, transaction_in_usd)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (block_hash) DO UPDATE
        SET block_height = EXCLUDED.block_height,
            total_transaction = EXCLUDED.total_transaction,
            time = EXCLUDED.time,
            transaction_in_usd = EXCLUDED.transaction_in_usd
    "#;

    println!("{}", &block.block_hash);

    let res = sqlx::query(query_str)
        .bind(&block.block_hash)
        .bind(block.block_height)
        .bind(block.total_transaction as i64)
        .bind(block.time)
        .bind(block.transaction_in_usd)
        .fetch_optional(pool)
        .await?;

        if let Some(row) = res {
            let inserted_id: i32 = row.get("id");
            println!("Inserted or updated row ID: {:?}", inserted_id);
        } else {
            println!("No row inserted or updated for block hash: {}", block.block_hash);
        }
    }
    

    Ok(())
}




pub async fn get_data_by_timestamp(pool: &PgPool) -> anyhow::Result<Vec<BlockInfo>, AnyhowError> {
    let start_time = Utc::now() - chrono::Duration::minutes(30);

    let query_str = r#"
            SELECT b.block_hash, b.block_height, b.total_transaction, b.time, b.transaction_in_usd
            FROM blocks b
            ORDER BY time DESC
            LIMIT 10
        "#;

    let rows = sqlx::query(query_str).bind(start_time).fetch_all(pool).await?;

    let mut block_data = vec![];

    for row in rows {
        let block_hash: String = row.get("block_hash");
        let block_height: i64 = row.get("block_height");
        let total_transaction: i32 = row.get("total_transaction"); // Fetch as i32
        let total_transaction: u32 = total_transaction as u32; 
        // let time: NaiveDateTime = row.try_get("time")?;
        let block_time: DateTime<Utc> = row.try_get("time")?;
        let transaction_in_usd = row.get("transaction_in_usd");
        
       
       

        block_data.push(BlockInfo {
            block_hash,
            block_height,
            total_transaction,
            time: block_time,
            transaction_in_usd
        });
    }

    Ok(block_data)
}