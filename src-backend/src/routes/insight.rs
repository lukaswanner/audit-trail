use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use sqlx::prelude::FromRow;

use crate::{
    session_state::{ApiSession, UserSession},
    AppState,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Insight {
    id: i32,
    icon: String,
    title: String,
    value: String,
    #[serde(rename = "projectTitle")]
    project_title: String,
}

pub async fn read_insight(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Extension(session): Extension<UserSession>,
) -> Json<Option<Insight>> {
    let result = sqlx::query_as::<_, Insight>("SELECT i.id, i.icon, i.title, i.value, p.title as project_title FROM insight i join project p on i.project_id = p.id WHERE account_id = $1 and lower(i.title) = lower($2)")
        .bind(session.account_id)
        .bind(name)
        .fetch_optional(&state.pool)
        .await
        .unwrap();

    Json(result)
}

pub async fn read_insights(
    State(state): State<AppState>,
    Extension(session): Extension<UserSession>,
) -> Json<Vec<Insight>> {
    let result = sqlx::query_as::<_, Insight>("SELECT i.id, i.icon, i.title, i.value, p.title as project_title FROM insight i join project p on i.project_id = p.id WHERE account_id = $1")
        .bind(session.account_id)
        .fetch_all(&state.pool)
        .await
        .unwrap();

    Json(result)
}

#[derive(Serialize, Deserialize)]
pub struct CreateInsight {
    icon: String,
    title: String,
    value: String,
    #[serde(rename = "projectId")]
    project_id: i32,
}

pub async fn create_insight(
    State(state): State<AppState>,
    Extension(session): Extension<ApiSession>,
    Json(payload): Json<CreateInsight>,
) -> StatusCode {
    let pool = &state.pool;

    let res = sqlx::query(
        "INSERT INTO insight (icon, title, value, project_id) 
SELECT $1 AS icon, $2 AS title, $3 as value, $5 as project_id
WHERE EXISTS (SELECT 1 FROM project WHERE account_id = $4 and id = $5)",
    )
    .bind(payload.icon)
    .bind(payload.title)
    .bind(payload.value)
    .bind(session.account_id)
    .bind(session.project_id)
    .execute(pool)
    .await;

    match res {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::CONFLICT,
    }
}
