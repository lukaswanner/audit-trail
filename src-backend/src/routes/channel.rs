use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::AppState;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Channel {
    id: i32,
    title: String,
    project_id: i32,
}

pub async fn read_channels(State(state): State<AppState>) -> Json<Vec<Channel>> {
    let pool = &state.pool;
    let result = sqlx::query_as::<_, Channel>("SELECT * FROM channel;")
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
