use axum::{extract::State, Json};
use serde_json::{json, Value};
use sqlx::{PgPool, Error as SqlxError};
use anyhow::Error;

use crate::repository::get_data_by_timestamp;

use super::{repository, models::AppState};







pub async fn get_bitcoin(State(state): State<AppState>) -> Json<Value> {
    let inserted_data = get_data_by_timestamp(&state.pg_pool).await.unwrap();
    Json(json!(inserted_data))
}