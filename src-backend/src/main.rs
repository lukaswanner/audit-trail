mod database;
mod middlewares;
mod notification;
mod routes;
mod session_state;

use std::sync::Arc;

use argon2::password_hash::SaltString;
use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, ORIGIN},
        HeaderValue, Method,
    },
    Router,
};
use dotenv::dotenv;

use futures::lock::Mutex;
use rand::rngs::OsRng;
use routes::server_routes;

use sqlx::PgPool;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub salt: SaltString,
    pub sms: Arc<Mutex<notification::sms::Sms>>,
}

#[tokio::main]
async fn main() {
    // load env variables first
    dotenv().ok();

    let db = database::db::Database::new_localhost();
    let sms = notification::sms::Sms::new();
    let salt = SaltString::generate(&mut OsRng);

    let shared_state = AppState {
        pool: db.pool,
        salt,
        sms: Arc::new(Mutex::new(sms)),
    };

    // we have 2 routes, one for our website, one for our api
    // the api route gets checked via api-key and the website route gets checked via jwt

    let cors = CorsLayer::new()
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers(vec![ORIGIN, AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(true)
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap());

    let app = Router::new()
        .nest("/auth", server_routes::login_routes(&shared_state))
        .nest("/api", server_routes::api_routes(&shared_state))
        .nest("/app", server_routes::app_routes(&shared_state))
        .nest("/ws", server_routes::websocket_routes(&shared_state))
        .layer(cors)
        .with_state(shared_state);

    let host = "localhost";
    let port = "3000";
    let addr = format!("{}:{}", host, port);
    println!("address on: {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
