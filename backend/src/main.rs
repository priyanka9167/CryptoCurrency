
use std::env;

use axum::{extract::State, http::HeaderName, routing::get, Json, Router};
use models::{ApiResponse, BitcoinData, AppState};
use reqwest::Method;
// use reqwest::Error;
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::{self, task, time::{self, sleep, Duration}};
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres, Row};
use dotenv::dotenv;
use tower_http::cors::{Any, CorsLayer};

use std::error::Error;
mod models;






#[tokio::main]
async fn main() {
    dotenv().expect("-->> No .env found \n");
    let pool = connect_n_get_db_pool().await.unwrap();
    // start_fetching_bitcoin_data(&pool).await;
    let shared_state = AppState {
        pg_pool:  pool
    };
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(vec![HeaderName::from_lowercase(b"content-type").unwrap()]);

    let app = Router::new()
        .route("/get_bitcoin", get(get_bitcoin))
        .layer(cors)
        .with_state(shared_state.clone());
        

    // run our app with hyper, listening globally on port 3000
    println!("Server started!!!");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn get_bitcoin(State(state): State<AppState>) -> Json<Value> {
    let latest_bitcoin_data = fetch_btc_data().await.unwrap();
    insert_bitcoin_data(&state.pg_pool, latest_bitcoin_data).await.unwrap();
    let inserted_data = get_inserted_bitcoin_data(&state.pg_pool).await.unwrap();
    Json(json!(inserted_data))
}

// async fn start_fetching_bitcoin_data(pool: &PgPool){
//     let mut interval = time::interval(Duration::from_secs(600));

//     loop {
//         interval.tick().await;
//         let pool_clone = pool.clone();
//         task::spawn(async move {
//             match fetch_btc_data().await {
//                 Ok(latest_bitcoin_data) => {
//                     if let Err(e) = insert_bitcoin_data(&pool_clone, latest_bitcoin_data).await {
//                         eprintln!("Failed to insert bitcoin data: {:?}", e);
//                     }
//                 }
//                 Err(e) => eprintln!("Failed to fetch bitcoin data: {:?}", e),
//             }
//         });
//     }
// }

// 1	BTC.main	846931	1717790346
async fn insert_bitcoin_data(pool: &PgPool, bitcoin_data: BitcoinData) -> anyhow::Result<()> {

    let query_str = r#"
        INSERT INTO bitcoin_data ( id, name, bitcoin_height, timestamp) 
        VALUES ( $1, $2, $3, $4 )
        ON CONFLICT(id) 
        DO UPDATE SET
        bitcoin_height = $3,
        timestamp = $4
    "#;
    
    let res  = sqlx::query(query_str)
    .bind(1)
    .bind(bitcoin_data.name)
    .bind(bitcoin_data.bitcoin_height)
    .bind(bitcoin_data.timestamp)
    // .bind(12345)
    .execute(pool)
    .await?;
    Ok(())
}

async fn get_inserted_bitcoin_data(pool: &PgPool) -> Result<BitcoinData, Box<dyn Error>>{
    let query_str = r#"
        SELECT name, bitcoin_height, timestamp FROM bitcoin_data WHERE id = 1;
    "#;
    let row = sqlx::query(query_str).fetch_one(pool).await?;

    let bitcoin_data = BitcoinData{
        name: row.get("name"),
        bitcoin_height: row.get("bitcoin_height"),
        timestamp: row.get("timestamp"),
    };
    Ok(bitcoin_data)



}

async fn connect_n_get_db_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    // Create a connection pool
    //  for MySQL/MariaDB, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await?;

    Ok(pool)
}

async fn fetch_btc_data() ->  anyhow::Result<BitcoinData> {
    let url = "https://api.blockcypher.com/v1/btc/main";
    let resp = reqwest::get(url).await?;
    let data: ApiResponse = resp.json().await?;

    println!("Bitcoin Data - {:?}", data);

    // Parse the time string into a timestamp value
    let timestamp = chrono::DateTime::parse_from_rfc3339(&data.time)
    .expect("Failed to parse timestamp")
    .timestamp();

    let new_bitcoin = BitcoinData {
        name: data.name,
        bitcoin_height: data.height as i32,
        timestamp:  timestamp as i32,
    };

    println!("{:?}",new_bitcoin);

    Ok(new_bitcoin)
}