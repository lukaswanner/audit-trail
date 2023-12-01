use futures::{
    stream::{SplitSink, StreamExt},
    SinkExt,
};
use tokio::time::interval;

use std::{sync::Arc, time::Duration};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use axum_extra::{headers::UserAgent, TypedHeader};

use crate::AppState;

use super::event::Event;

pub async fn handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<UserAgent>>,
    state: State<Arc<AppState>>,
) -> Response {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };

    println!("`{user_agent}`  connected.");
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn write(mut sender: SplitSink<WebSocket, Message>, state: State<Arc<AppState>>) {
    let mut interval = interval(Duration::from_secs(5));
    let query = "SELECT * FROM event;";
    loop {
        interval.tick().await;
        let events = sqlx::query_as::<_, Event>(query)
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

async fn handle_socket(mut socket: WebSocket, state: State<Arc<AppState>>) {
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        println!("Pinged...")
    } else {
        println!("Could not send ping!")
    }

    let (sender, _) = socket.split();

    tokio::spawn(write(sender, state));
}
