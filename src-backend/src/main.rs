mod database;
mod middlewares;
mod routes;
mod session_state;

use argon2::password_hash::SaltString;
use axum::{
    middleware::{self},
    routing::{delete, get, post},
    Router,
};

use middlewares::auth;
use rand::rngs::OsRng;
use routes::{api_token, authorize, channel, event, project, user, websocket};

use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub salt: SaltString,
}

#[tokio::main]
async fn main() {
    let db = database::db::Database::new_localhost();
    let salt = SaltString::generate(&mut OsRng);

    let shared_state = AppState {
        pool: db.pool,
        salt,
    };

    // we have 2 routes, one for our website, one for our api
    // the api route gets checked via api-key and the website route gets checked via jwt
    let api_routes = Router::new()
        .route("/channel", post(channel::create_channel))
        .route("/user", post(user::create_user))
        .route("/event", post(event::create_event))
        .route_layer(middleware::from_fn_with_state(
            shared_state.clone(),
            auth::check_request_with_api_token,
        ));

    let app_routes = Router::new()
        // get
        .route("/channels", get(channel::read_channels))
        .route("/users", get(user::read_users))
        .route("/events", get(event::read_events))
        .route("/projects", get(project::read_projects))
        // post
        .route("/project", post(project::create_project))
        .route("/api-token", post(api_token::create_api_token))
        // delete
        .route("/api-token", delete(api_token::delete_api_token))
        .route_layer(middleware::from_fn_with_state(
            shared_state.clone(),
            auth::check_request_with_jwt_token,
        ));

    let websocket_routes = Router::new().route("/ws/events", get(websocket::handler));

    let login_routes = Router::new()
        .route("/login", post(authorize::login))
        .route("/register", post(authorize::register));

    let app = Router::new()
        .nest("/auth", login_routes)
        .nest("/api", api_routes)
        .nest("/app", app_routes)
        .nest("/", websocket_routes)
        .with_state(shared_state);

    let host = "localhost";
    let port = "3000";
    let addr = format!("{}:{}", host, port);
    println!("address on: {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
