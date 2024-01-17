use crate::{session_state::UserSession, AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use chrono::{DateTime, Utc};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ApiTokenCreatePayload {
    #[serde(rename = "projectId")]
    pub project_id: i32,
}

pub struct ApiToken(String);

impl ApiToken {
    pub fn generate_new(length: usize) -> ApiToken {
        let token: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();

        ApiToken(token)
    }
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct ApiTokenResponse {
    id: i32,
    token: String,
    #[serde(rename = "projectTitle")]
    title: String,
    #[serde(rename = "createdAt")]
    created_at: DateTime<Utc>,
}

pub async fn read_api_tokens(
    State(state): State<AppState>,
    Extension(session): Extension<UserSession>,
) -> Json<Vec<ApiTokenResponse>> {
    let res = sqlx::query_as::<_,ApiTokenResponse>("SELECT a.id, a.token, p.title, a.created_at FROM api_token a JOIN project p on a.project_id = p.id WHERE p.account_id = $1")
        .bind(session.account_id)
        .fetch_all(&state.pool)
        .await
        .unwrap();

    Json(res)
}

pub async fn create_api_token(
    State(state): State<AppState>,
    Extension(session): Extension<UserSession>,
    Json(payload): Json<ApiTokenCreatePayload>,
) -> Response {
    let api_token = ApiToken::generate_new(32);
    sqlx::query("INSERT INTO api_token (project_id, token) SELECT $1 AS project_id, $2 AS token WHERE EXISTS (SELECT 1 FROM project WHERE id = $1 and account_id = $3)")
        .bind(payload.project_id)
        .bind(&api_token.0)
        .bind(session.account_id)
        .execute(&state.pool)
        .await
        .unwrap();

    Json(api_token.0).into_response()
}

pub async fn delete_api_token(
    State(state): State<AppState>,
    Extension(session): Extension<UserSession>,
    Path(id): Path<i32>,
) -> Response {
    let res = sqlx::query("DELETE FROM api_token WHERE EXISTS (SELECT 1 from project WHERE account_id = $2) AND id = $1")
        .bind(id)
        .bind(session.account_id)
        .execute(&state.pool)
        .await
        .unwrap();

    if res.rows_affected() == 0 {
        return StatusCode::NOT_FOUND.into_response();
    }

    StatusCode::NO_CONTENT.into_response()
}
