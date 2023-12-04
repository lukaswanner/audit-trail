use crate::AppState;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Project {
    id: i32,
    account_id: i32,
    title: String,
}

pub async fn read_projects(State(state): State<AppState>) -> Json<Vec<Project>> {
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
    account_id: i32,
}

pub async fn create_project(
    State(state): State<AppState>,
    Json(payload): Json<CreateProject>,
) -> &'static str {
    let pool = &state.pool;
    sqlx::query("INSERT INTO project (title, account_id) VALUES ($1, $2);")
        .bind(payload.title)
        .bind(payload.account_id)
        .execute(pool)
        .await
        .unwrap();

    "Created project"
}
