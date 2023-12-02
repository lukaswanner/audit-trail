mod auth;
mod database;
mod routes;

use axum::{
    middleware::{self},
    routing::{get, post},
    Router,
};

use routes::{channel, event, project, user, websocket};

use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

#[tokio::main]
async fn main() {
    let db = database::db::Database::new_localhost();

    let shared_state = AppState { pool: db.pool };

    let app = Router::new()
        // websocket routes
        .route("/ws", get(websocket::handler))
        // middleware
        .route_layer(middleware::from_fn_with_state(
            shared_state.clone(),
            auth::check_request,
        ))
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
        // state
        .with_state(shared_state);

    let host = "localhost";
    let port = "3000";
    let addr = format!("{}:{}", host, port);
    println!("address on: {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
