use crate::AppState;
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ApiTokenDb {
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
    Json(payload): Json<ApiTokenDb>,
) -> Response {
    let api_token = ApiToken::generate_new(32);
    sqlx::query("INSERT INTO api_token (project_id, token) VALUES ($1, $2 );")
        .bind(payload.project_id)
        .bind(&api_token.0)
        .execute(&state.pool)
        .await
        .unwrap();

    Json(api_token.0).into_response()
}
