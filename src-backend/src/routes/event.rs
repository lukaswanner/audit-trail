use crate::{
    session_state::{ApiSession, UserSession},
    AppState,
};
use ::chrono::{DateTime, Utc};
use axum::{
    extract::{Path, State},
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Event {
    id: i32,
    icon: String,
    title: String,
    #[serde(rename = "channelTitle")]
    channel_title: String,
    #[serde(rename = "userName")]
    user_name: String,
    ts: DateTime<Utc>,
}

pub async fn read_event(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Extension(session): Extension<UserSession>,
) -> Json<Option<Event>> {
    let pool = &state.pool;
    let result = sqlx::query_as::<_, Event>("SELECT e.id, e.icon, e.title, c.title as channel_title, u.name as user_name, e.ts FROM event e JOIN channel c on e.channel_id = c.id JOIN event_user u on e.user_id = u.id JOIN project p on c.project_id = p.id where p.account_id = $1 and e.id = $2")
        .bind(session.account_id)
        .bind(id)
        .fetch_optional(pool)
        .await
        .unwrap();

    Json(result)
}

pub async fn read_events(
    State(state): State<AppState>,
    Extension(session): Extension<UserSession>,
) -> Json<Vec<Event>> {
    let pool = &state.pool;
    let result = sqlx::query_as::<_, Event>("SELECT e.id, e.icon, e.title, c.title as channel_title, u.name as user_name, e.ts FROM event e JOIN channel c on e.channel_id = c.id JOIN event_user u on e.user_id = u.id JOIN project p on c.project_id = p.id where p.account_id = $1")
        .bind(session.account_id)
        .fetch_all(pool)
        .await
        .unwrap();

    Json(result)
}

pub async fn read_events_from_channel(
    State(state): State<AppState>,
    Path(channel_title): Path<String>,
    Extension(session): Extension<UserSession>,
) -> Json<Vec<Event>> {
    let pool = &state.pool;
    let result = sqlx::query_as::<_, Event>("SELECT e.id, e.icon, e.title, c.title as channel_title, u.name as user_name, e.ts FROM event e JOIN channel c on e.channel_id = c.id JOIN event_user u on e.user_id = u.id JOIN project p on c.project_id = p.id where p.account_id = $1 and c.title = $2")
            .bind(session.account_id)
            .bind(channel_title)
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
    Extension(session): Extension<ApiSession>,
    Json(payload): Json<CreateEvent>,
) -> &'static str {
    let pool = &state.pool;
    let query = "INSERT INTO event (icon, title, channel_id, user_id)
    SELECT $1 AS title, $2 AS icon, $3 as channel_id, $4 as user_id
    WHERE EXISTS (SELECT 1 FROM project WHERE account_id = $5 and id = $6)";
    sqlx::query(query)
        .bind(payload.icon)
        .bind(payload.title)
        .bind(payload.channel_id)
        .bind(payload.user_id)
        .bind(session.account_id)
        .bind(session.project_id)
        .execute(pool)
        .await
        .unwrap();

    "Created event"
}
