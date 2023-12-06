use crate::{session_state::UserSession, AppState};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
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

pub async fn create_api_token(
    State(state): State<AppState>,
    Extension(session): Extension<UserSession>,
    Json(payload): Json<ApiTokenCreatePayload>,
) -> Response {
    let api_token = ApiToken::generate_new(32);
    sqlx::query("INSERT INTO api_token (project_id, token) SELECT $1 AS project_id, $2 AS token WHERE EXISTS (SELECT 1 FROM project WHERE id = $1 and account_id = $3);")
        .bind(payload.project_id)
        .bind(&api_token.0)
        .bind(session.account_id)
        .execute(&state.pool)
        .await
        .unwrap();

    Json(api_token.0).into_response()
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ApiTokenDeletePayload {
    #[serde(rename = "apiToken")]
    pub token: String,
}

pub async fn delete_api_token(
    State(state): State<AppState>,
    Extension(session): Extension<UserSession>,
    Json(payload): Json<ApiTokenDeletePayload>,
) -> Response {
    let res = sqlx::query("DELETE FROM api_token WHERE EXISTS (SELECT 1 from project WHERE account_id = $2) AND token = $1;")
        .bind(payload.token)
        .bind(session.account_id)
        .execute(&state.pool)
        .await
        .unwrap();

    if res.rows_affected() == 0 {
        return StatusCode::NOT_FOUND.into_response();
    }

    StatusCode::NO_CONTENT.into_response()
}
