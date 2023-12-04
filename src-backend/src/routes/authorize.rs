use axum::{
    body::Body,
    extract::{Path, State},
    http::{Response, StatusCode},
};
use axum_extra::extract::cookie::Cookie;
use chrono::Duration;
use sqlx::prelude::FromRow;

use crate::AppState;

use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    project_id: i32,
    exp: usize,
}

#[derive(FromRow, Deserialize)]
struct Project {
    id: i32,
}

// if thigs function gets called, we know that the api-key is valid
pub async fn authorize(
    State(_state): State<AppState>,
    Path(project_title): Path<String>,
) -> Response<Body> {
    let query = "Select id from project where title = $1";

    let project = sqlx::query_as::<_, Project>(query)
        .bind(project_title)
        .fetch_one(&_state.pool)
        .await
        .unwrap();

    let my_claims = Claims {
        project_id: project.id,
        exp: (chrono::Utc::now() + Duration::days(1)).timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap();

    let mut cookie = Cookie::new("jwt", token);
    cookie.set_path("/");

    Response::builder()
        .header("Set-Cookie", cookie.to_string())
        .status(StatusCode::OK)
        .body(Body::empty())
        .unwrap()
}
