use std::sync::Arc;

use axum::extract::State;

use crate::AppState;

pub async fn read_channel(state: State<Arc<AppState>>) -> &'static str {
    let pool = &state.pool;
    let result = sqlx::query("SELECT * FROM channel;")
        .execute(pool)
        .await
        .unwrap();

    println!("result: {:?}", result);
    "Hello, World!"
}

pub async fn create_channel(state: State<Arc<AppState>>) -> &'static str {
    let pool = &state.pool;
    let result = sqlx::query("INSERT INTO channel (title, project_id) VALUES ('test', 1);")
        .execute(pool)
        .await
        .unwrap();

    println!("result: {:?}", result);
    "Hello, World!"
}
