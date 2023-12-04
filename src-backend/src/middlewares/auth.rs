use axum::{
    extract::{Path, Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::AppState;

fn extract_api_token(headers: &HeaderMap) -> Option<&str> {
    match headers.get("api-key") {
        Some(token) => Some(token.to_str().unwrap()),
        None => None,
    }
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct ApiToken {
    project_id: i32,
}

// check if the token is in db
// return true if we have a valid token that matches
async fn token_is_valid(token: &str, project_id: i32, state: AppState) -> bool {
    let query = "Select project_id from api_token where token = $1 and project_id = $2";

    let api_token = sqlx::query_as::<_, ApiToken>(query)
        .bind(token)
        .bind(project_id)
        .fetch_optional(&state.pool)
        .await
        .unwrap();

    match api_token {
        Some(_) => return true,
        _ => return false,
    }
}

pub async fn check_request(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(project_id): Path<i32>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    match extract_api_token(&headers) {
        Some(token) if token_is_valid(token, project_id, state).await => {
            let response = next.run(request).await;
            return Ok(response);
        }

        _ => return Err(StatusCode::UNAUTHORIZED),
    }
}