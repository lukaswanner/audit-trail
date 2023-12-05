use axum::{extract::State, Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{session_state::UserSession, AppState};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Channel {
    id: i32,
    title: String,
    project_title: String,
}

pub async fn read_channels(
    State(state): State<AppState>,
    Extension(session): Extension<UserSession>,
) -> Json<Vec<Channel>> {
    let pool = &state.pool;
    let result = sqlx::query_as::<_, Channel>(
        "SELECT c.id, c.title, p.title as project_title FROM channel c join project p on c.project_id = p.id WHERE p.account_id = $1;",
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
    #[serde(rename = "projectId")]
    project_id: i32,
}

pub async fn create_channel(
    State(state): State<AppState>,
    Json(payload): Json<CreateChannel>,
) -> &'static str {
    let pool = &state.pool;
    sqlx::query("INSERT INTO channel (title, project_id) VALUES ($1, $2);")
        .bind(payload.title)
        .bind(payload.project_id)
        .execute(pool)
        .await
        .unwrap();

    "Created channel"
}
