use futures::{
    lock::Mutex,
    stream::{SplitSink, SplitStream, StreamExt},
    SinkExt,
};
use tokio::time::interval;

use std::{str::FromStr, sync::Arc, time::Duration};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
    Extension,
};

use crate::{session_state::UserSession, AppState};

#[derive(sqlx::FromRow, serde::Serialize)]
struct Count {
    count: i64,
}

// "channel_id:event_id"
#[derive(Debug)]
struct ChannelMessage {
    channel_id: i32,
    event_id: i32,
}

impl FromStr for ChannelMessage {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_message = s.split_once(":");

        match raw_message {
            Some((channel_id, event_id)) => Ok(ChannelMessage {
                channel_id: channel_id.parse().unwrap(),
                event_id: event_id.parse().unwrap(),
            }),
            None => Err(()),
        }
    }
}

pub async fn handler(
    ws: WebSocketUpgrade,
    Extension(session): Extension<UserSession>,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, session, state))
}

async fn write(
    mut sender: SplitSink<WebSocket, Message>,
    channel_arc: Arc<Mutex<i32>>,
    event_arc: Arc<Mutex<i32>>,
    session: UserSession,
    state: AppState,
) {
    let mut interval = interval(Duration::from_secs(5));

    let query_events = r#"
    SELECT 
        COUNT(e.id) as count
    FROM event e
    JOIN channel c ON e.channel_id = c.id
    JOIN project p ON c.project_id = p.id
    WHERE 
        p.account_id = $1
        AND c.id = $2
        AND e.id > $3
    "#;

    loop {
        interval.tick().await;
        let channel_id = channel_arc.lock().await;
        let event_id = event_arc.lock().await;
        if *channel_id == -1 {
            continue;
        }
        if *event_id == -1 {
            continue;
        }
        let count = sqlx::query_as::<_, Count>(query_events)
            .bind(session.account_id)
            .bind(*channel_id)
            .bind(*event_id)
            .fetch_one(&state.pool)
            .await
            .unwrap();

        if sender
            .send(Message::Text(count.count.to_string()))
            .await
            .is_err()
        {
            return;
        }
    }
}

async fn read(
    mut receiver: SplitStream<WebSocket>,
    channel_arc: Arc<Mutex<i32>>,
    event_arc: Arc<Mutex<i32>>,
) {
    while let Some(message) = receiver.next().await {
        let message = match message {
            Ok(message) => message,
            Err(_) => return,
        };

        match message {
            Message::Text(text) => {
                let channel_message = match text.parse::<ChannelMessage>() {
                    Ok(channel_message) => channel_message,
                    Err(_) => return,
                };

                let mut current_id = channel_arc.lock().await;
                if *current_id != channel_message.channel_id {
                    *current_id = channel_message.channel_id;
                }
                let mut current_event_id = event_arc.lock().await;
                if *current_event_id != channel_message.event_id {
                    *current_event_id = channel_message.event_id;
                }
            }
            Message::Close(_) => return,
            _ => (),
        }
    }
}

async fn handle_socket(socket: WebSocket, session: UserSession, state: AppState) {
    let (sender, receiver) = socket.split();
    let last_event_id: Arc<Mutex<i32>> = Arc::new(Mutex::new(-1));
    let current_channel: Arc<Mutex<i32>> = Arc::new(Mutex::new(-1));

    let mut read_task = tokio::spawn(read(
        receiver,
        current_channel.clone(),
        last_event_id.clone(),
    ));
    let mut write_task = tokio::spawn(write(
        sender,
        current_channel.clone(),
        last_event_id.clone(),
        session,
        state,
    ));

    tokio::select! {
        _ = &mut write_task => read_task.abort(),
        _ = &mut read_task => write_task.abort(),
    }
}
