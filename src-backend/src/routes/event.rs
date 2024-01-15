use crate::{
    session_state::{ApiSession, UserSession},
    AppState,
};
use ::chrono::{DateTime, Utc};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Event {
    id: i32,
    icon: String,
    title: String,
    #[serde(rename = "channelTitle")]
    channel_title: String,
    #[serde(rename = "actorName")]
    actor_name: String,
    #[serde(rename = "projectId")]
    project_id: i32,
    ts: DateTime<Utc>,
    tags: Value,
}

pub async fn read_event(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Extension(session): Extension<UserSession>,
) -> Json<Option<Event>> {
    let pool = &state.pool;
    let result = sqlx::query_as::<_, Event>(
        r#"
    SELECT 
        e.id,
        e.icon,
        e.title,
        c.title as channel_title,
        a.name as actor_name,
        p.id as project_id,
        e.ts,
        COALESCE(
            (
                SELECT
                    JSONB_AGG(json_build_object(t.key, t.value))
                FROM 
                    tag_event te
                JOIN tag t ON te.tag_id = t.id
                WHERE 
                    te.event_id = e.id
            )
        , '[]'::jsonb) AS tags
    FROM 
        event e 
    JOIN channel c on e.channel_id = c.id 
    JOIN actor a on e.actor_id = a.id 
    JOIN project p on c.project_id = p.id 
    WHERE 
        p.account_id = $1 and e.id = $2 
    GROUP BY 
        e.id, c.title, a.name, p.id"#,
    )
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
    let result = sqlx::query_as::<_, Event>(
        r#"
    SELECT 
        e.id,
        e.icon,
        e.title,
        c.title as channel_title,
        a.name as actor_name,
        p.id as project_id,
        e.ts,
        COALESCE(
            (
                SELECT
                    JSONB_AGG(json_build_object(t.key, t.value))
                FROM 
                    tag_event te
                JOIN tag t ON te.tag_id = t.id
                WHERE 
                    te.event_id = e.id
            )
        , '[]'::jsonb) AS tags
    FROM 
        event e 
    JOIN channel c on e.channel_id = c.id 
    JOIN actor a on e.actor_id = a.id 
    JOIN project p on c.project_id = p.id 
    where 
        p.account_id = $1 
    GROUP BY
        e.id, c.title, a.name,p.id"#,
    )
    .bind(session.account_id)
    .fetch_all(pool)
    .await
    .unwrap();

    Json(result)
}

#[derive(Deserialize, Debug)]
pub struct TagSearch {
    key: String,
    value: String,
}

pub async fn read_events_from_tag(
    State(state): State<AppState>,
    search_query: Query<TagSearch>,
    Extension(session): Extension<UserSession>,
) -> Json<Vec<Event>> {
    let pool = &state.pool;
    let result = sqlx::query_as::<_, Event>(
        r#"
    SELECT 
        e.id,
        e.icon,
        e.title,
        c.title AS channel_title,
        a.name AS actor_name,
        p.id as project_id,
        e.ts,
        Jsonb_agg(Json_build_object(t.KEY, t.value)) AS tags
    FROM 
        event e
       JOIN channel c ON e.channel_id = c.id
       JOIN actor a ON e.actor_id = a.id
       JOIN project p ON c.project_id = p.id
       JOIN tag_event te ON e.id = te.event_id
       JOIN tag t ON te.tag_id = t.id
    WHERE  
        p.account_id = $1
    GROUP BY e.id, c.title, a.name, p.id
    HAVING
        MAX(CASE WHEN t.key = $2 AND t.value = $3 THEN 1 ELSE 0 END) = 1
"#,
    )
    .bind(session.account_id)
    .bind(&search_query.key)
    .bind(&search_query.value)
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
    let result = sqlx::query_as::<_, Event>(
        r#"
    SELECT 
        e.id,
        e.icon,
        e.title,
        c.title AS channel_title,
        a.name AS actor_name,
        p.id as project_id,
        e.ts,
        COALESCE(
            (
                SELECT
                    JSONB_AGG(json_build_object(t.key, t.value))
                FROM 
                    tag_event te
                JOIN tag t ON te.tag_id = t.id
                WHERE 
                    te.event_id = e.id
            )
        , '[]'::jsonb) AS tags
    FROM event e
    JOIN channel c ON e.channel_id = c.id
    JOIN actor a ON e.actor_id = a.id
    JOIN project p ON c.project_id = p.id
    WHERE 
        p.account_id = $1
        AND c.title = $2
    GROUP BY e.id, c.title, a.name, p.id"#,
    )
    .bind(session.account_id)
    .bind(channel_title)
    .fetch_all(pool)
    .await
    .unwrap();

    Json(result)
}

pub async fn read_events_from_actor(
    State(state): State<AppState>,
    Path(actor_id): Path<i32>,
    Extension(session): Extension<UserSession>,
) -> Json<Vec<Event>> {
    let pool = &state.pool;
    let result = sqlx::query_as::<_, Event>(
        r#"
    SELECT 
        e.id,
        e.icon,
        e.title,
        c.title AS channel_title,
        a.name AS actor_name,
        p.id as project_id,
        e.ts,
        COALESCE(
            (
                SELECT
                    JSONB_AGG(json_build_object(t.key, t.value))
                FROM 
                    tag_event te
                JOIN tag t ON te.tag_id = t.id
                WHERE 
                    te.event_id = e.id
            )
        , '[]'::jsonb) AS tags
    FROM event e
    JOIN channel c ON e.channel_id = c.id
    JOIN actor a ON e.actor_id = a.id
    JOIN project p ON c.project_id = p.id
    WHERE 
        p.account_id = $1
        AND e.actor_id = $2
    GROUP BY e.id, c.title, a.name, p.id"#,
    )
    .bind(session.account_id)
    .bind(actor_id)
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
    pub channel_id: i32,
    #[serde(rename = "actorId")]
    pub actor_id: i32,
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

async fn payload_is_valid(session: &ApiSession, payload: &CreateEvent, state: &AppState) -> bool {
    let query = "SELECT a.id as actor_id, ch.id as channel_id FROM project p join channel ch on p.id = ch.project_id join actor a on p.id = a.project_id WHERE p.id = $1 and a.id = $2 and ch.id = $3";

    let valid = sqlx::query(query)
        .bind(session.project_id)
        .bind(payload.actor_id)
        .bind(payload.channel_id)
        .fetch_optional(&state.pool)
        .await;

    matches!(valid, Ok(Some(_)))
}

pub async fn create_event(
    State(state): State<AppState>,
    Extension(session): Extension<ApiSession>,
    Json(payload): Json<CreateEvent>,
) -> StatusCode {
    let pool = &state.pool;

    if !payload_is_valid(&session, &payload, &state).await {
        return StatusCode::UNAUTHORIZED;
    }

    let event_query = "INSERT INTO event (icon, title, channel_id, actor_id)
    SELECT $1 AS title, $2 AS icon, $3 as channel_id, $4 as actor_id
    WHERE EXISTS (SELECT 1 FROM project WHERE account_id = $5 and id = $6) returning id";
    let event_res = sqlx::query_as::<_, CreateEventResponse>(event_query)
        .bind(payload.icon)
        .bind(payload.title)
        .bind(payload.channel_id)
        .bind(payload.actor_id)
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

    if !tag_res.is_empty() {
        let tag_event_query = "INSERT INTO tag_event(tag_id, event_id) select tag_id, $1 from unnest($2::int[]) as tag_id";

        let tag_ids: Vec<i32> = tag_res.iter().map(|t| t.id).collect();
        sqlx::query(tag_event_query)
            .bind(event_res.id)
            .bind(tag_ids)
            .execute(pool)
            .await
            .unwrap();
    }

    StatusCode::CREATED
}
