mod database;
mod middlewares;
mod routes;

use axum::{
    middleware::{self},
    routing::{delete, get, post},
    Router,
};

use middlewares::auth;
use routes::{api_token, authorize, channel, event, project, user, websocket};

use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

#[tokio::main]
async fn main() {
    let db = database::db::Database::new_localhost();

    let shared_state = AppState { pool: db.pool };

    // we have 2 routes, one for our website, one for our api
    // the api route gets checked via api-key and the website route gets checked via jwt
    let authorize_routes = Router::new()
        .route("/api-key/:project_title", post(authorize::authorize))
        .route_layer(middleware::from_fn_with_state(
            shared_state.clone(),
            auth::check_request_with_api_token,
        ));

    let api_routes = Router::new()
        // use jwt for the rest
        .route("/channel", post(channel::create_channel))
        .route("/user", post(user::create_user))
        .route("/event", post(event::create_event))
        .route_layer(middleware::from_fn_with_state(
            shared_state.clone(),
            auth::check_request_with_jwt_token,
        ));

    let app_routes = Router::new()
        .route("/ws/events", get(websocket::handler))
        .route("/channels", get(channel::read_channels))
        .route("/users", get(user::read_users))
        .route("/events", get(event::read_events))
        .route("/projects", get(project::read_projects))
        .route("/project", post(project::create_project))
        .route("/api-token", post(api_token::create_api_token))
        .route("/api-token", delete(api_token::delete_api_token))
        .route_layer(middleware::from_fn_with_state(
            shared_state.clone(),
            auth::check_request_with_jwt_token,
        ));

    let app = Router::new()
        .nest("/authorize", authorize_routes)
        .nest("/api", api_routes)
        .nest("/", app_routes)
        .with_state(shared_state);

    let host = "localhost";
    let port = "3000";
    let addr = format!("{}:{}", host, port);
    println!("address on: {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
