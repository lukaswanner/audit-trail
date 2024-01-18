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
    #[serde(rename = "projectTitle")]
    project_title: String,
}

#[derive(Deserialize)]
pub struct CreateChannelApi {
    title: String,
}

#[derive(Deserialize)]
pub struct CreateChannelPayload {
    title: String,
    #[serde(rename = "projectId")]
    project_id: i32,
}

#[derive(Deserialize)]
pub struct UpdateChannelPayload {
    id: i32,
    title: String,
    #[serde(rename = "projectId")]
    project_id: i32,
}

pub async fn read_channel(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Extension(session): Extension<UserSession>,
) -> Json<Option<Channel>> {
    let pool = &state.pool;
    let result = sqlx::query_as::<_, Channel>(
        "SELECT c.id, c.title, p.title as project_title FROM channel c join project p on c.project_id = p.id WHERE p.account_id = $1 and c.id = $2",
    )
    .bind(session.account_id)
    .bind(id)
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

pub async fn read_channels_for_project(
    State(state): State<AppState>,
    Path(project_id): Path<i32>,
    Extension(session): Extension<UserSession>,
) -> Json<Vec<Channel>> {
    let pool = &state.pool;
    let result = sqlx::query_as::<_, Channel>(
                "SELECT c.id, c.title, p.title as project_title FROM channel c join project p on c.project_id = p.id WHERE p.account_id = $1 and p.id = $2",
            )
        .bind(session.account_id)
        .bind(project_id)
        .fetch_all(pool)
        .await
    .unwrap();

    Json(result)
}

pub async fn create_channel(
    State(state): State<AppState>,
    Extension(session): Extension<UserSession>,
    Json(payload): Json<CreateChannelPayload>,
) -> StatusCode {
    let pool = &state.pool;
    let res = sqlx::query("INSERT INTO channel (title, project_id) SELECT $1 AS title, $2 AS project_id WHERE EXISTS (SELECT 1 FROM project WHERE account_id = $3 and id = $2)")
        .bind(payload.title)
        .bind(payload.project_id)
        .bind(session.account_id)
        .execute(pool)
        .await;

    match res {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::CONFLICT,
    }
}

pub async fn update_channel(
    State(state): State<AppState>,
    Extension(session): Extension<UserSession>,
    Json(payload): Json<UpdateChannelPayload>,
) -> StatusCode {
    let pool = &state.pool;
    let query = r#"
    UPDATE channel 
        SET title = $1, project_id = $2  
    WHERE 
        id = $3 AND 
    EXISTS (
        SELECT 1 FROM project p
        WHERE 
            p.account_id = $4 and p.id = $2
        )"#;
    let res = sqlx::query(query)
        .bind(payload.title)
        .bind(payload.project_id)
        .bind(payload.id)
        .bind(session.account_id)
        .execute(pool)
        .await;

    match res {
        Ok(row) => {
            if row.rows_affected() == 0 {
                return StatusCode::NOT_MODIFIED;
            }
            StatusCode::OK
        }
        Err(_) => StatusCode::NOT_MODIFIED,
    }
}

pub async fn create_channel_api(
    State(state): State<AppState>,
    Extension(session): Extension<ApiSession>,
    Json(payload): Json<CreateChannelApi>,
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

pub async fn delete_channel(
    State(state): State<AppState>,
    Extension(session): Extension<UserSession>,
    Path(id): Path<i32>,
) -> StatusCode {
    let res = sqlx::query("DELETE FROM channel WHERE EXISTS (SELECT 1 from project WHERE account_id = $1) AND id = $2")
        .bind(session.account_id)
        .bind(id)
        .execute(&state.pool)
        .await
        .unwrap();

    if res.rows_affected() == 0 {
        return StatusCode::NOT_FOUND;
    }

    StatusCode::NO_CONTENT
}
