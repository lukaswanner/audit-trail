mod database;
mod middlewares;
mod routes;
mod session_state;

use argon2::password_hash::SaltString;
use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, ORIGIN},
        HeaderValue, Method,
    },
    middleware::{self},
    routing::{delete, get, post},
    Router,
};

use middlewares::auth;
use rand::rngs::OsRng;
use routes::{api_token, authorize, channel, event, insight, project, user, websocket};

use sqlx::PgPool;
use tower_http::cors::CorsLayer;

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
        .route("/event", post(event::create_event))
        .route("/channel", post(channel::create_channel_api))
        .route("/user", post(user::create_user))
        .route("/project", post(project::create_project_api))
        .route("/insight", post(insight::create_insight))
        .route_layer(middleware::from_fn_with_state(
            shared_state.clone(),
            auth::check_request_with_api_token,
        ));

    let app_routes = Router::new()
        // get
        .route("/channels", get(channel::read_channels))
        .route("/channels/:title", get(channel::read_channels_for_project))
        .route("/channel/:title", get(channel::read_channel))
        .route("/channel", post(channel::create_channel))
        .route("/users", get(user::read_users))
        .route("/user/:name", get(user::read_user))
        .route("/insight/:project_title/:name", get(insight::read_insight))
        .route("/insights/:project_title", get(insight::read_insights))
        .route("/events", get(event::read_events))
        .route(
            "/events/:channel_title",
            get(event::read_events_from_channel),
        )
        .route("/event/:id", get(event::read_event))
        .route("/project/:title", get(project::read_project))
        .route("/projects", get(project::read_projects))
        .route("/api-tokens", get(api_token::read_api_tokens))
        .route("/search", get(event::read_events_from_tag))
        // post
        .route("/project", post(project::create_project))
        .route("/api-token", post(api_token::create_api_token))
        // delete
        .route("/api-token", delete(api_token::delete_api_token))
        .route("/project-delete/:id", delete(project::delete_project))
        .route("/channel-delete/:id", delete(channel::delete_channel))
        .route_layer(middleware::from_fn_with_state(
            shared_state.clone(),
            auth::check_request_with_jwt_token,
        ));

    let websocket_routes = Router::new().route("/events", get(websocket::handler));

    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec![ORIGIN, AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(true)
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap());

    let login_routes = Router::new()
        .route("/login", post(authorize::login))
        .route("/register", post(authorize::register));

    let app = Router::new()
        .nest("/auth", login_routes)
        .nest("/api", api_routes)
        .nest("/app", app_routes)
        .nest("/ws", websocket_routes)
        .layer(cors)
        .with_state(shared_state);

    let host = "localhost";
    let port = "3000";
    let addr = format!("{}:{}", host, port);
    println!("address on: {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
