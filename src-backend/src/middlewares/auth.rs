use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{
    session_state::{ApiSession, UserSession},
    AppState,
};
use jsonwebtoken::{decode, DecodingKey, Validation};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    account_id: i32,
    exp: usize,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct ApiToken {
    account_id: i32,
    project_id: i32,
}

#[derive(Debug, FromRow, Deserialize)]
struct Account {
    id: i32,
}

fn extract_jwt_token(jar: &CookieJar) -> Option<&str> {
    match jar.get("__audit") {
        Some(cookie) => Some(cookie.value()),
        None => None,
    }
}

fn extract_api_token(headers: &HeaderMap) -> Option<&str> {
    match headers.get("api-key") {
        Some(token) => Some(token.to_str().unwrap()),
        None => None,
    }
}

// check if the token is in db
// return true if we have a valid token that matches
async fn key_is_valid(token: &str, state: AppState) -> Option<ApiToken> {
    let query = "Select project_id, p.account_id from api_token a join project p on a.project_id = p.id where a.token = $1";

    sqlx::query_as::<_, ApiToken>(query)
        .bind(token)
        .fetch_optional(&state.pool)
        .await
        .unwrap()
}

async fn token_is_valid(token: &str, state: AppState) -> Option<Account> {
    let jwt_token = decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    );

    let query = "Select id from account where id = $1";

    if let Ok(token) = jwt_token {
        let account = sqlx::query_as::<_, Account>(query)
            .bind(token.claims.account_id)
            .fetch_optional(&state.pool)
            .await;

        if let Ok(Some(account)) = account {
            return Some(account);
        }
    }
    None
}

pub async fn check_request_with_jwt_token(
    State(state): State<AppState>,
    cookie: CookieJar,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if let Some(token) = extract_jwt_token(&cookie) {
        if let Some(acc) = token_is_valid(token, state).await {
            request.extensions_mut().insert(UserSession::new(acc.id));
            let response = next.run(request).await;
            return Ok(response);
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

pub async fn check_request_with_api_token(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if let Some(token) = extract_api_token(&headers) {
        if let Some(acc) = key_is_valid(token, state).await {
            request
                .extensions_mut()
                .insert(ApiSession::new(acc.account_id, acc.project_id));
            let response = next.run(request).await;
            return Ok(response);
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}
