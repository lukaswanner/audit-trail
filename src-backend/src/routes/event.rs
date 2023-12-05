use crate::{session_state::UserSession, AppState};
use axum::{extract::State, Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Event {
    id: i32,
    icon: String,
    title: String,
    channel_title: String,
    user_name: String,
}

pub async fn read_events(
    State(state): State<AppState>,
    Extension(session): Extension<UserSession>,
) -> Json<Vec<Event>> {
    let pool = &state.pool;
    let result = sqlx::query_as::<_, Event>("SELECT e.id, e.icon, e.title, c.title as channel_title, u.name as user_name FROM event e JOIN channel c on e.channel_id = c.id JOIN event_user u on e.user_id = u.id JOIN project p on c.project_id = p.id where p.account_id = $1;")
        .bind(session.account_id)
        .fetch_all(pool)
        .await
        .unwrap();

    Json(result)
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
    State(state): State<AppState>,
    Json(payload): Json<CreateEvent>,
) -> &'static str {
    let pool = &state.pool;
    sqlx::query("INSERT INTO event (icon, title, channel_id, user_id) VALUES ($1, $2, $3, $4);")
        .bind(payload.icon)
        .bind(payload.title)
        .bind(payload.channel_id)
        .bind(payload.user_id)
        .execute(pool)
        .await
        .unwrap();

    "Created event"
}
