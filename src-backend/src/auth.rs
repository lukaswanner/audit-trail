use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::AppState;

fn extract_api_token(headers: &HeaderMap) -> Option<&str> {
    match headers.get("api-key") {
        Some(token) => Some(token.to_str().unwrap()),
        None => None,
    }
}

// placeholder for now until i implement proper auth
fn token_is_valid(_token: &str, _state: AppState) -> bool {
    true
}

pub async fn check_request(
    State(state): State<AppState>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    match extract_api_token(&headers) {
        Some(token) if token_is_valid(token, state) => {
            let response = next.run(request).await;
            return Ok(response);
        }
        _ => return Err(StatusCode::UNAUTHORIZED),
    }
}
