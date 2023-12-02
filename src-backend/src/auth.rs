use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

fn extract_api_token(headers: &HeaderMap) -> Option<&str> {
    match headers.get("api-key") {
        Some(token) => Some(token.to_str().unwrap()),
        None => None,
    }
}

// placeholder for now until i implement proper auth
fn token_is_valid(_token: &str) -> bool {
    true
}

pub async fn check_request(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    match extract_api_token(&headers) {
        Some(token) if token_is_valid(token) => {
            let response = next.run(request).await;
            Ok(response)
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}
