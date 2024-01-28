use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use sqlx::prelude::FromRow;

use crate::{session_state::UserSession, AppState};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NotificationUser {
    id: i32,
    name: String,
    #[serde(rename = "channelId")]
    channel_id: i32,
    #[serde(rename = "channelTitle")]
    channel_title: String,
    #[serde(rename = "phoneNumber")]
    phone_number: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateNotificationUser {
    name: String,
    #[serde(rename = "channelId")]
    channel_id: i32,
    #[serde(rename = "phoneNumber")]
    phone_number: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateNotificationUser {
    name: String,
    #[serde(rename = "channelId")]
    channel_id: i32,
    #[serde(rename = "phoneNumber")]
    phone_number: String,
}

pub async fn read_notification_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Extension(session): Extension<UserSession>,
) -> Json<Option<NotificationUser>> {
    let result = sqlx::query_as::<_, NotificationUser>(
        r#"
    SELECT 
        n.id,
        n.name,
        n.channel_id,
        c.title as channel_title,
        n.phone_number
    FROM 
        notification_user n 
    JOIN channel c on n.channel_id = c.id
    JOIN project p on c.project_id = p.id 
    WHERE 
        p.account_id = $1 and n.id = $2"#,
    )
    .bind(session.account_id)
    .bind(id)
    .fetch_optional(&state.pool)
    .await
    .unwrap();

    Json(result)
}

pub async fn read_notification_users(
    State(state): State<AppState>,
    Extension(session): Extension<UserSession>,
) -> Json<Vec<NotificationUser>> {
    let result = sqlx::query_as::<_, NotificationUser>(
        r#"
    SELECT 
        n.id,
        n.name,
        n.channel_id,
        c.title as channel_title,
        n.phone_number
    FROM 
        notification_user n 
    JOIN channel c on n.channel_id = c.id
    JOIN project p on c.project_id = p.id 
    WHERE 
        p.account_id = $1"#,
    )
    .bind(session.account_id)
    .fetch_all(&state.pool)
    .await
    .unwrap();

    Json(result)
}

pub async fn read_notification_users_for_channel(
    State(state): State<AppState>,
    Path(channel_id): Path<i32>,
    Extension(session): Extension<UserSession>,
) -> Json<Vec<NotificationUser>> {
    let result = sqlx::query_as::<_, NotificationUser>(
        r#"
    SELECT 
        n.id,
        n.name,
        n.channel_id,
        c.title as channel_title,
        n.phone_number
    FROM 
        notification_user n 
    JOIN channel c on n.channel_id = c.id
    JOIN project p on c.project_id = p.id 
    WHERE 
        p.account_id = $1
        AND n.channel_id = $2
        "#,
    )
    .bind(session.account_id)
    .bind(channel_id)
    .fetch_all(&state.pool)
    .await
    .unwrap();

    Json(result)
}

pub async fn create_new_notification_user(
    State(state): State<AppState>,
    Extension(session): Extension<UserSession>,
    Json(payload): Json<CreateNotificationUser>,
) -> StatusCode {
    let result = sqlx::query_as::<_, NotificationUser>(
        r#"
    INSERT INTO 
        notification_user (name,channel_id, phone_number)
    SELECT $1 AS name, $2 AS channel_id, $3 as phone_number
    WHERE EXISTS
    (
        SELECT 1 
            FROM channel c 
        JOIN project p on c.project_id = p.id 
        WHERE
            p.account_id = $4 AND 
            c.id = $2
    )
            "#,
    )
    .bind(payload.name)
    .bind(payload.channel_id)
    .bind(payload.phone_number)
    .bind(session.account_id)
    .fetch_all(&state.pool)
    .await;

    match result {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}
