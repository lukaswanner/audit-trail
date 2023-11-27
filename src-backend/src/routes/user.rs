use std::sync::Arc;

use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};

use crate::AppState;
use serde::{Deserialize, Serialize};

pub async fn read_user(state: State<Arc<AppState>>) -> Response {
    let pool = &state.pool;
    let result = sqlx::query("SELECT * FROM event_user;")
        .execute(pool)
        .await
        .unwrap();

    format!("result: {:?}", result).into_response()
}

#[derive(Serialize, Deserialize)]
struct Properties {
    #[serde(default)]
    email: Option<String>,
    #[serde(default)]
    phone: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateUser {
    name: String,
    properties: Properties,
}

pub async fn create_user(
    state: State<Arc<AppState>>,
    Json(payload): Json<CreateUser>,
) -> &'static str {
    let pool = &state.pool;

    let properties = serde_json::to_string(&payload.properties).unwrap();
    let result = sqlx::query("INSERT INTO event_user (name,properties) VALUES ($1,$2);")
        .bind(payload.name)
        .bind(properties)
        .execute(pool)
        .await
        .unwrap();

    println!("result: {:?}", result);
    "Created user"
}
