use std::collections::HashMap;

use axum::{extract::State, Json};
use serde_json::{json, Value};
use sqlx::prelude::FromRow;

use crate::AppState;
use serde::{Deserialize, Serialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    id: i32,
    name: String,
    properties: sqlx::types::Json<HashMap<String, String>>,
}

pub async fn read_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let pool = &state.pool;
    let result = sqlx::query_as::<_, User>("SELECT * FROM event_user;")
        .fetch_all(pool)
        .await
        .unwrap();

    Json(result)
}

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    name: String,
    #[serde(rename = "projectId")]
    project_id: i32,
    properties: HashMap<String, Value>,
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> &'static str {
    let pool = &state.pool;

    let props = json!(payload.properties);
    sqlx::query("INSERT INTO event_user (name,project_id, properties) VALUES ($1,$2,$3);")
        .bind(payload.name)
        .bind(payload.project_id)
        .bind(props)
        .execute(pool)
        .await
        .unwrap();

    "Created user"
}
