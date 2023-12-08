use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{
    session_state::{ApiSession, UserSession},
    AppState,
};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Channel {
    id: i32,
    title: String,
    project_title: String,
}

pub async fn read_channel(
    State(state): State<AppState>,
    Path(title): Path<String>,
    Extension(session): Extension<UserSession>,
) -> Json<Option<Channel>> {
    let pool = &state.pool;
    let result = sqlx::query_as::<_, Channel>(
        "SELECT c.id, c.title, p.title as project_title FROM channel c join project p on c.project_id = p.id WHERE p.account_id = $1 and lower(c.title) = lower($2)",
    )
    .bind(session.account_id)
    .bind(title)
    .fetch_optional(pool)
    .await
    .unwrap();

    Json(result)
}

pub async fn read_channels(
    State(state): State<AppState>,
    Extension(session): Extension<UserSession>,
) -> Json<Vec<Channel>> {
    let pool = &state.pool;
    let result = sqlx::query_as::<_, Channel>(
        "SELECT c.id, c.title, p.title as project_title FROM channel c join project p on c.project_id = p.id WHERE p.account_id = $1",
    )
    .bind(session.account_id)
    .fetch_all(pool)
    .await
    .unwrap();

    Json(result)
}

#[derive(Deserialize)]
pub struct CreateChannel {
    title: String,
}

pub async fn create_channel(
    State(state): State<AppState>,
    Extension(session): Extension<ApiSession>,
    Json(payload): Json<CreateChannel>,
) -> StatusCode {
    let pool = &state.pool;
    let res = sqlx::query("INSERT INTO channel (title, project_id) SELECT $1 AS title, $2 AS project_id WHERE EXISTS (SELECT 1 FROM project WHERE account_id = $3 and id = $2)")
        .bind(payload.title)
        .bind(session.project_id)
        .bind(session.account_id)
        .execute(pool)
        .await;

    match res {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::CONFLICT,
    }
}
