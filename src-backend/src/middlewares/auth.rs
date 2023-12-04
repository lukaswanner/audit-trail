use axum::{
    extract::{Path, Request, State},
    http::{header::COOKIE, HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::AppState;
use jsonwebtoken::{decode, DecodingKey, Validation};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    project_id: i32,
    exp: usize,
}

fn extract_jwt_token(jar: &CookieJar) -> Option<&str> {
    match jar.get("jwt") {
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

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct ApiToken {
    project_id: i32,
}

// check if the token is in db
// return true if we have a valid token that matches
async fn key_is_valid(token: &str, project_title: String, state: AppState) -> bool {
    let query = "Select project_id from api_token a join project p on a.project_id = p.id where a.token = $1 and p.title = $2";

    let api_token = sqlx::query_as::<_, ApiToken>(query)
        .bind(token)
        .bind(project_title)
        .fetch_optional(&state.pool)
        .await
        .unwrap();

    match api_token {
        Some(_) => return true,
        _ => return false,
    }
}

async fn token_is_valid(token: &str) -> bool {
    let jwt_token = decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    );
    println!("{:?}", jwt_token);
    match jwt_token {
        Ok(_) => return true,
        _ => return false,
    }
}

pub async fn check_request_with_jwt_token(
    cookie: CookieJar,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    match extract_jwt_token(&cookie) {
        Some(token) if token_is_valid(token).await => {
            let response = next.run(request).await;
            return Ok(response);
        }

        _ => return Err(StatusCode::UNAUTHORIZED),
    }
}

// todo check if i can just use the body
pub async fn check_request_with_api_token(
    State(state): State<AppState>,
    Path(project_title): Path<String>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    match extract_api_token(&headers) {
        Some(token) if key_is_valid(token, project_title, state).await => {
            let response = next.run(request).await;
            return Ok(response);
        }

        _ => return Err(StatusCode::UNAUTHORIZED),
    }
}
