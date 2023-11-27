use std::sync::Arc;

use crate::AppState;
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Event {
    id: i32,
    icon: String,
    title: String,
    channel_id: i32,
    user_id: i32,
}

pub async fn read_event(state: State<Arc<AppState>>) -> Response {
    let pool = &state.pool;
    let result = sqlx::query_as::<_, Event>("SELECT * FROM event;")
        .fetch_all(pool)
        .await
        .unwrap();

    format!("result: {:?}", result).into_response()
}

#[derive(Deserialize)]
pub struct CreateEvent {
    icon: String,
    title: String,
    #[serde(rename = "channelId")]
    channel_id: i32,
    #[serde(rename = "userId")]
    user_id: i32,
}

pub async fn create_event(
    state: State<Arc<AppState>>,
    Json(payload): Json<CreateEvent>,
) -> &'static str {
    let pool = &state.pool;
    let result = sqlx::query(
        "INSERT INTO event (icon, title, channel_id, user_id) VALUES ($1, $2, $3, $4);",
    )
    .bind(payload.icon)
    .bind(payload.title)
    .bind(payload.channel_id)
    .bind(payload.user_id)
    .execute(pool)
    .await
    .unwrap();

    println!("result: {:?}", result);
    "Created event"
}
