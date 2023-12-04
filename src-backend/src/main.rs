mod database;
mod middlewares;
mod routes;

use axum::{
    middleware::{self},
    routing::{delete, get, post},
    Router,
};

use middlewares::auth;
use routes::{api_token, channel, event, project, user, websocket};

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
        .route("/ws/events/:project_id", get(websocket::handler))
        // get routes
        .route("/channel/:project_id", get(channel::read_channel))
        .route("/user/:project_id", get(user::read_user))
        .route("/event/:project_id", get(event::read_event))
        // post routes
        .route("/channel/:project_id", post(channel::create_channel))
        .route("/user/:project_id", post(user::create_user))
        .route("/event/:project_id", post(event::create_event))
        // middleware
        .route_layer(middleware::from_fn_with_state(
            shared_state.clone(),
            auth::check_request,
        ))
        .route("/project", get(project::read_project))
        .route("/project", post(project::create_project))
        .route("/api-token", post(api_token::create_api_token))
        .route("/api-token", delete(api_token::delete_api_token))
        // state
        .with_state(shared_state);

    let host = "localhost";
    let port = "3000";
    let addr = format!("{}:{}", host, port);
    println!("address on: {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
