use crate::{
    session_state::{ApiSession, UserSession},
    AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Project {
    id: i32,
    project_title: String,
}

pub async fn read_project(
    State(state): State<AppState>,
    Path(title): Path<String>,
    Extension(session): Extension<UserSession>,
) -> Json<Option<Project>> {
    let query =
        "SELECT id, title as project_title FROM project WHERE account_id = $1 and lower(title) = lower($2)";
    let result = sqlx::query_as::<_, Project>(query)
        .bind(session.account_id)
        .bind(title)
        .fetch_optional(&state.pool)
        .await
        .unwrap();

    Json(result)
}

pub async fn read_projects(
    State(state): State<AppState>,
    Extension(session): Extension<UserSession>,
) -> Json<Vec<Project>> {
    let query = "SELECT id, title as project_title FROM project WHERE account_id = $1";
    let result = sqlx::query_as::<_, Project>(query)
        .bind(session.account_id)
        .fetch_all(&state.pool)
        .await
        .unwrap();

    Json(result)
}

#[derive(Deserialize)]
pub struct CreateProject {
    title: String,
}

pub async fn create_project(
    State(state): State<AppState>,
    Extension(session): Extension<UserSession>,
    Json(payload): Json<CreateProject>,
) -> StatusCode {
    let pool = &state.pool;
    let res = sqlx::query("INSERT INTO project (title, account_id) VALUES ($1, $2);")
        .bind(payload.title)
        .bind(session.account_id)
        .execute(pool)
        .await;

    match res {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::CONFLICT,
    }
}

pub async fn create_project_api(
    State(state): State<AppState>,
    Extension(session): Extension<ApiSession>,
    Json(payload): Json<CreateProject>,
) -> StatusCode {
    let pool = &state.pool;
    let res = sqlx::query("INSERT INTO project (title, account_id) VALUES ($1, $2);")
        .bind(payload.title)
        .bind(session.account_id)
        .execute(pool)
        .await;

    match res {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::CONFLICT,
    }
}
