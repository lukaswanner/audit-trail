use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use serde_json::{json, Value};
use sqlx::prelude::FromRow;

use crate::{
    session_state::{ApiSession, UserSession},
    AppState,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    id: i32,
    name: String,
    project_title: String,
    properties: sqlx::types::Json<HashMap<String, String>>,
}

pub async fn read_user(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Extension(session): Extension<UserSession>,
) -> Json<Option<User>> {
    let result = sqlx::query_as::<_, User>("SELECT ev.id, ev.name, p.title as project_title, ev.properties FROM event_user ev join project p on ev.project_id = p.id WHERE account_id = $1 and lower(name) = lower($2)")
        .bind(session.account_id)
        .bind(name)
        .fetch_optional(&state.pool)
        .await
        .unwrap();

    Json(result)
}

pub async fn read_users(
    State(state): State<AppState>,
    Extension(session): Extension<UserSession>,
) -> Json<Vec<User>> {
    let result = sqlx::query_as::<_, User>("SELECT ev.id, ev.name, p.title as project_title, ev.properties FROM event_user ev join project p on ev.project_id = p.id WHERE account_id = $1")
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
    Extension(session): Extension<ApiSession>,
    Json(payload): Json<CreateUser>,
) -> StatusCode {
    let pool = &state.pool;

    let props = json!(payload.properties);
    let res = sqlx::query(
        "INSERT INTO event_user (name,project_id, properties) 
SELECT $1 AS name, $2 AS project_id, $3 as properties
WHERE EXISTS (SELECT 1 FROM project WHERE account_id = $4 and id = $5)",
    )
    .bind(payload.name)
    .bind(payload.project_id)
    .bind(props)
    .bind(session.account_id)
    .bind(session.project_id)
    .execute(pool)
    .await;

    match res {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::CONFLICT,
    }
}
