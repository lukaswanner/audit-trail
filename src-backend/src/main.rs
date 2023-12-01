mod database;
mod routes;

use std::sync::Arc;

use axum::{
    extract::Request,
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
    Router, http::HeaderMap,
};
use routes::{channel, event, project, user};

use sqlx::PgPool;

pub struct AppState {
    pub pool: PgPool,
}

async fn auth(_headers: HeaderMap, request: Request, next: Next) -> Response {
    println!("Got a request: {:?}", request);

    let response = next.run(request).await;
    response
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
        // data
        .with_state(shared_state)
        .route_layer(middleware::from_fn(auth));

    let host = "localhost";
    let port = "3000";
    let addr = format!("{}:{}", host, port);
    println!("address on: {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
//curl requests

/*
* post project/
* curl -v -X POST -H "Content-Type: application/json" -d '{"title":"test"}' 0.0.0.0/project:3000
*/

/*
* post channel/
* curl -v -X POST -H "Content-Type: application/json" -d '{"title":"test", "project_id": 1}' 0.0.0.0/project:3000
*/

/*
* post user/
* curl -v -X POST -H "Content-Type: application/json" -d '{"name":"test"}'
*/

/* post event/
* curl -v -X POST -H "Content-Type: application/json" -d '{"icon":"😎", "title":"test", "channelId": 1, "userId": 1}' 0.0.0.0:3000/event
*/
