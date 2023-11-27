use std::sync::Arc;

use axum::extract::State;

use crate::AppState;

pub async fn read_project(state: State<Arc<AppState>>) -> &'static str {
    let pool = &state.pool;
    let result = sqlx::query("SELECT * FROM project;")
        .execute(pool)
        .await
        .unwrap();

    println!("result: {:?}", result);
    "Hello, World!"
}

pub async fn create_project(state: State<Arc<AppState>>) -> &'static str {
    let pool = &state.pool;
    let result = sqlx::query("INSERT INTO project (title) VALUES ('test');")
        .execute(pool)
        .await
        .unwrap();

    println!("result: {:?}", result);
    "Hello, World!"
}
