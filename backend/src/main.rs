

use std::time::Duration;

use axum::{ http::HeaderName, routing::get,  Router};
use handlers::get_bitcoin;
use models::AppState;
// use reqwest::Error;
use dotenv::dotenv;
use reqwest::Method;
use sqlx::{PgPool, Pool};
use tokio::time::interval;
use tower_http::cors::{Any, CorsLayer};

mod models;

mod db;
mod handlers;
mod repository;
mod blockchain_apis;

#[tokio::main]
async fn main() {
    dotenv().expect("-->> No .env found \n");
    let pool = db::connect_n_get_db_pool().await.unwrap();


    // let latest_blocks = blockchain_apis::fetch_btc_data().await.unwrap();

    // repository::insert_blocks(&pool, latest_blocks).await.unwrap();

    // start_fetching_bitcoin_data(&pool).await;
    

    let shared_state = AppState { pg_pool: pool.clone() };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(vec![HeaderName::from_lowercase(b"content-type").unwrap()]);

        let app = Router::new()
        .route("/get_bitcoin", get(handlers::get_bitcoin))
        .route("/get_rate", get(handlers::fetch_rate))
        .layer(cors)
        .with_state(shared_state.clone());

    // run our app with hyper, listening globally on port 3000
    println!("Server started!!!");
    

    

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // Spawn a new task to fetch Bitcoin data
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(10));
        loop {
            interval.tick().await;
            start_fetching(&pool).await;
            // Handle the fetched data as needed
        }
    });

    axum::serve(listener, app).await.unwrap();
}

async fn start_fetching(pool: &PgPool){
    println!("Started fetching!!!");
    let mut interval = interval(Duration::from_secs(10));
    loop{
        interval.tick().await;
        // let latest_bitcoin_data = fetch_btc_data().await;
        let latest_blocks = blockchain_apis::fetch_btc_data().await.unwrap();

        repository::insert_blocks(&pool, latest_blocks).await.unwrap();
    }
}

