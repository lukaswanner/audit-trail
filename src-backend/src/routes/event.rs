use std::sync::Arc;

use axum::{extract::State, Json};

use crate::AppState;

pub async fn read_event(state: State<Arc<AppState>>) -> &'static str {
    let pool = &state.pool;
    let result = sqlx::query("SELECT * FROM event;")
        .execute(pool)
        .await
        .unwrap();

    println!("result: {:?}", result);
    "Hello, World!"
}

#[derive(serde::Deserialize)]
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
    let result = sqlx::query("INSERT INTO event (icon, title, channel_id, user_id) VALUES ($1, $2, $3, $4);")
        .bind(payload.icon)
        .bind(payload.title)
        .bind(payload.channel_id)
        .bind(payload.user_id)
        .execute(pool)
        .await
        .unwrap();

    println!("result: {:?}", result);
    "Hello, World!"
}
