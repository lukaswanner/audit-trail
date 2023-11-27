mod database;
mod routes;

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use routes::{channel, event, project, user};

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
        // data
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
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
