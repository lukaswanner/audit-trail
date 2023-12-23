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
use serde_json::Value;
use sqlx::{prelude::FromRow, QueryBuilder};

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
    tags: Value,
}

pub async fn read_event(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Extension(session): Extension<UserSession>,
) -> Json<Option<Event>> {
    let pool = &state.pool;
    let result = sqlx::query_as::<_, Event>("SELECT e.id, e.icon, e.title, c.title as channel_title, u.name as user_name, e.ts, JSONB_AGG(json_build_object(t.key, t.value)) as tags FROM event e JOIN channel c on e.channel_id = c.id JOIN event_user u on e.user_id = u.id JOIN project p on c.project_id = p.id join tag_event te on e.id = te.event_id join tag t on te.tag_id = t.id where p.account_id = $1 and e.id = $2 group by e.id, c.title, u.name")
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
    let result = sqlx::query_as::<_, Event>("SELECT e.id, e.icon, e.title, c.title as channel_title, u.name as user_name, e.ts, JSONB_AGG(json_build_object(t.key, t.value)) as tags FROM event e JOIN channel c on e.channel_id = c.id JOIN event_user u on e.user_id = u.id JOIN project p on c.project_id = p.id join tag_event te on e.id = te.event_id join tag t on te.tag_id = t.id where p.account_id = $1 group by e.id, c.title, u.name")
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
    let result = sqlx::query_as::<_, Event>("SELECT e.id, e.icon, e.title, c.title as channel_title, u.name as user_name, e.ts, JSONB_AGG(json_build_object(t.key, t.value)) as tags FROM event e JOIN channel c on e.channel_id = c.id JOIN event_user u on e.user_id = u.id JOIN project p on c.project_id = p.id join tag_event te on e.id = te.event_id join tag t on te.tag_id = t.id where p.account_id = $1 and c.title = $2 group by e.id, c.title, u.name")
            .bind(session.account_id)
            .bind(channel_title)
            .fetch_all(pool)
            .await
        .unwrap();

    Json(result)
}

#[derive(Deserialize)]
pub struct CreateTag {
    key: String,
    value: String,
}

#[derive(Deserialize)]
pub struct CreateEvent {
    icon: String,
    title: String,
    #[serde(rename = "channelId")]
    channel_id: i32,
    #[serde(rename = "userId")]
    user_id: i32,
    tags: Vec<CreateTag>,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct CreateEventResponse {
    id: i32,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct TagResponse {
    id: i32,
}

pub async fn create_event(
    State(state): State<AppState>,
    Extension(session): Extension<ApiSession>,
    Json(payload): Json<CreateEvent>,
) -> &'static str {
    let pool = &state.pool;
    let event_query = "INSERT INTO event (icon, title, channel_id, user_id)
    SELECT $1 AS title, $2 AS icon, $3 as channel_id, $4 as user_id
    WHERE EXISTS (SELECT 1 FROM project WHERE account_id = $5 and id = $6) returning id";
    let event_res = sqlx::query_as::<_, CreateEventResponse>(event_query)
        .bind(payload.icon)
        .bind(payload.title)
        .bind(payload.channel_id)
        .bind(payload.user_id)
        .bind(session.account_id)
        .bind(session.project_id)
        .fetch_one(pool)
        .await
        .unwrap();

    let mut tag_query =
        "INSERT INTO tag(key, value) select * from unnest($1::text[], $2::text[]) on CONFLICT DO NOTHING";

    let keys = payload
        .tags
        .iter()
        .map(|t| t.key.as_str())
        .collect::<Vec<&str>>();
    let values = payload
        .tags
        .iter()
        .map(|t| t.value.as_str())
        .collect::<Vec<&str>>();

    sqlx::query(tag_query)
        .bind(&keys)
        .bind(&values)
        .execute(pool)
        .await
        .unwrap();

    tag_query = "SELECT id from tag where key = Any($1) and value = Any($2)";

    let tag_res = sqlx::query_as::<_, TagResponse>(tag_query)
        .bind(keys)
        .bind(values)
        .fetch_all(pool)
        .await
        .unwrap();

    if tag_res.len() > 0 {
        let tag_event_query = "INSERT INTO tag_event(tag_id, event_id) select tag_id, $1 from unnest($2::int[]) as tag_id";

        let tag_ids: Vec<i32> = tag_res.iter().map(|t| t.id).collect();
        sqlx::query(tag_event_query)
            .bind(event_res.id)
            .bind(tag_ids)
            .execute(pool)
            .await
            .unwrap();
    }
    "Created event"
}
