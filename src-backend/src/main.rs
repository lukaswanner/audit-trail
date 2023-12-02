mod auth;
mod database;
mod routes;

use std::sync::Arc;

use axum::{
    middleware::{self},
    routing::{get, post},
    Router,
};

use routes::{channel, event, project, user, websocket};

use sqlx::PgPool;

pub struct AppState {
    pub pool: PgPool,
}

#[tokio::main]
async fn main() {
    let db = database::db::Database::new_localhost();

    let shared_state = Arc::new(AppState { pool: db.pool });

    let app = Router::new()
        // get routes
        .route("/channel", get(channel::read_channel))
        .route("/project", get(project::read_project))
        .route("/user", get(user::read_user))
        .route("/event", get(event::read_event))
        // post routes
        .route("/channel", post(channel::create_channel))
        .route("/project", post(project::create_project))
        .route("/user", post(user::create_user))
        .route("/event", post(event::create_event))
        // websocket routes
        .route("/ws", get(websocket::handler))
        // data
        .with_state(shared_state)
        .route_layer(middleware::from_fn(auth::check_request));

    let host = "localhost";
    let port = "3000";
    let addr = format!("{}:{}", host, port);
    println!("address on: {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
