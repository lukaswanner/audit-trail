use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use serde_json::{json, Value};
use sqlx::prelude::FromRow;

use crate::{
    session_state::{ApiSession, UserSession},
    AppState,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Actor {
    id: i32,
    name: String,
    #[serde(rename = "projectTitle")]
    project_title: String,
    properties: sqlx::types::Json<HashMap<String, String>>,
}

pub async fn read_actor(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Extension(session): Extension<UserSession>,
) -> Json<Option<Actor>> {
    let result = sqlx::query_as::<_, Actor>("SELECT a.id, a.name, p.title as project_title, a.properties FROM actor a join project p on a.project_id = p.id WHERE account_id = $1 and a.id = $2")
        .bind(session.account_id)
        .bind(id)
        .fetch_optional(&state.pool)
        .await
        .unwrap();

    Json(result)
}

pub async fn read_actors(
    State(state): State<AppState>,
    Extension(session): Extension<UserSession>,
) -> Json<Vec<Actor>> {
    let result = sqlx::query_as::<_, Actor>("SELECT a.id, a.name, p.title as project_title, a.properties FROM actor a join project p on a.project_id = p.id WHERE account_id = $1")
        .bind(session.account_id)
        .fetch_all(&state.pool)
        .await
        .unwrap();

    Json(result)
}

#[derive(Serialize, Deserialize)]
pub struct CreateActor {
    name: String,
    properties: HashMap<String, Value>,
}

pub async fn create_actor(
    State(state): State<AppState>,
    Extension(session): Extension<ApiSession>,
    Json(payload): Json<CreateActor>,
) -> StatusCode {
    let pool = &state.pool;

    let props = json!(payload.properties);
    let res = sqlx::query(
        "INSERT INTO actor (name,project_id, properties) 
SELECT $1 AS name, $2 AS project_id, $3 as properties
WHERE EXISTS (SELECT 1 FROM project WHERE account_id = $4 and id = $2)",
    )
    .bind(payload.name)
    .bind(session.project_id)
    .bind(props)
    .bind(session.account_id)
    .execute(pool)
    .await;

    match res {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::CONFLICT,
    }
}
