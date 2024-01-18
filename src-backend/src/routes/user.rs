use axum::{extract::State, http::StatusCode, Extension, Json};
use serde::Serialize;
use sqlx::prelude::FromRow;

use crate::{session_state::UserSession, AppState};

#[derive(FromRow, Serialize)]
pub struct User {
    pub email: String,
}

pub async fn read_user(
    State(state): State<AppState>,
    Extension(session): Extension<UserSession>,
) -> Json<Option<User>> {
    let query = r#"
        SELECT 
            a.email
        FROM 
            account a
        WHERE 
            a.id = $1
    "#;

    let result = sqlx::query_as::<_, User>(query)
        .bind(session.account_id)
        .fetch_optional(&state.pool)
        .await;

    match result {
        Ok(user) => Json(user),
        Err(_) => Json(None),
    }
}

pub async fn delete_user(
    State(state): State<AppState>,
    Extension(session): Extension<UserSession>,
) -> StatusCode {
    let query = r#"
        DELETE FROM 
            account a
        WHERE 
            a.id = $1
    "#;

    let result = sqlx::query(query)
        .bind(session.account_id)
        .fetch_optional(&state.pool)
        .await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
