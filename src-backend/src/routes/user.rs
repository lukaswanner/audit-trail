use std::sync::Arc;

use axum::extract::State;

use crate::AppState;

pub async fn read_user(state: State<Arc<AppState>>) -> &'static str {
    let pool = &state.pool;
    let result = sqlx::query("SELECT * FROM event_user;")
        .execute(pool)
        .await
        .unwrap();

    println!("result: {:?}", result);
    "Hello, World!"
}

pub async fn create_user(state: State<Arc<AppState>>) -> &'static str {
    let pool = &state.pool;
    let result = sqlx::query("INSERT INTO users (name, properties) VALUES ('lukas', '{}');")
        .execute(pool)
        .await
        .unwrap();

    println!("result: {:?}", result);
    "Hello, World!"
}
