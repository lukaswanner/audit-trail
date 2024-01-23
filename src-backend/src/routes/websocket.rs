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

use super::event::Event;

struct ChannelMessage {
    channel_id: i32,
}

impl FromStr for ChannelMessage {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_channel_id = s.split_once(":").map(|(_, id)| id);

        match raw_channel_id {
            Some(channel_id) => Ok(ChannelMessage {
                channel_id: channel_id.parse().unwrap(),
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
    session: UserSession,
    state: AppState,
) {
    let mut interval = interval(Duration::from_secs(5));

    let query_events = r#"SELECT 
        e.id,
        e.icon,
        e.title,
        c.title AS channel_title,
        a.name AS actor_name,
        p.id as project_id,
        e.ts,
        COALESCE(
            (
                SELECT
                    JSONB_AGG(json_build_object(t.key, t.value))
                FROM 
                    tag_event te
                JOIN tag t ON te.tag_id = t.id
                WHERE 
                    te.event_id = e.id
            )
        , '[]'::jsonb) AS tags
    FROM event e
    JOIN channel c ON e.channel_id = c.id
    JOIN actor a ON e.actor_id = a.id
    JOIN project p ON c.project_id = p.id
    WHERE 
        p.account_id = $1
        AND c.id = $2
    GROUP BY e.id, c.title, a.name, p.id
    ORDER BY 
            e.ts DESC
    "#;

    loop {
        interval.tick().await;
        let channel_id = channel_arc.lock().await;
        if *channel_id == -1 {
            continue;
        }
        let events = sqlx::query_as::<_, Event>(query_events)
            .bind(session.account_id)
            .bind(*channel_id)
            .fetch_all(&state.pool)
            .await
            .unwrap();

        if sender
            .send(Message::Text(serde_json::to_string(&events).unwrap()))
            .await
            .is_err()
        {
            return;
        }
    }
}

async fn read(mut receiver: SplitStream<WebSocket>, current_channel_id: Arc<Mutex<i32>>) {
    while let Some(message) = receiver.next().await {
        println!("Received message: {:?}", message);
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

                println!("Channel ID: {}", channel_message.channel_id);
                let mut current_id = current_channel_id.lock().await;
                if *current_id != channel_message.channel_id {
                    *current_id = channel_message.channel_id;
                }
            }
            Message::Close(_) => return,
            _ => (),
        }
    }
}

async fn handle_socket(mut socket: WebSocket, session: UserSession, state: AppState) {
    println!("New WebSocket connection");
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_err() {
        println!("Could not send ping!")
    }

    let (sender, receiver) = socket.split();
    let current_channel_id: Arc<Mutex<i32>> = Arc::new(Mutex::new(-1));

    let mut read_task = tokio::spawn(read(receiver, current_channel_id.clone()));
    let mut write_task = tokio::spawn(write(sender, current_channel_id.clone(), session, state));

    tokio::select! {
        _ = &mut write_task => read_task.abort(),
        _ = &mut read_task => write_task.abort(),
    }
}
