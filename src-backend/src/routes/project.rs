use crate::AppState;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Project {
    id: i32,
    title: String,
}

pub async fn read_project(State(state): State<AppState>) -> Json<Vec<Project>> {
    let pool = &state.pool;
    let result = sqlx::query_as::<_, Project>("SELECT * FROM project;")
        .fetch_all(pool)
        .await
        .unwrap();

    Json(result)
}

#[derive(Deserialize)]
pub struct CreateProject {
    title: String,
}

pub async fn create_project(
    State(state): State<AppState>,
    Json(payload): Json<CreateProject>,
) -> &'static str {
    let pool = &state.pool;
    sqlx::query("INSERT INTO project (title) VALUES ($1);")
        .bind(payload.title)
        .execute(pool)
        .await
        .unwrap();

    "Created project"
}
