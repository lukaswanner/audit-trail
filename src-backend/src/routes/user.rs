use std::collections::HashMap;

use axum::{extract::State, Extension, Json};
use serde_json::{json, Value};
use sqlx::prelude::FromRow;

use crate::{session_state::UserSession, AppState};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    id: i32,
    name: String,
    project_title: String,
    properties: sqlx::types::Json<HashMap<String, String>>,
}

pub async fn read_users(
    State(state): State<AppState>,
    Extension(session): Extension<UserSession>,
) -> Json<Vec<User>> {
    let result = sqlx::query_as::<_, User>("SELECT ev.id, ev.name, p.title as project_title, ev.properties FROM event_user ev join project p on ev.project_id = p.id WHERE account_id = $1; ")
        .bind(session.account_id)
        .fetch_all(&state.pool)
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
