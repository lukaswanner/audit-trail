use futures::{
    stream::{SplitSink, SplitStream, StreamExt},
    SinkExt,
};
use tokio::time::interval;

use std::time::Duration;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::Response,
};
use axum_extra::{headers::UserAgent, TypedHeader};

use crate::AppState;

use super::event::Event;

pub async fn handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<UserAgent>>,
    Path(project_id): Path<i32>,
    State(state): State<AppState>,
) -> Response {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };

    println!("`{user_agent}`  connected.");
    ws.on_upgrade(move |socket| handle_socket(socket, project_id, state))
}

async fn write(mut sender: SplitSink<WebSocket, Message>, project_id: i32, state: AppState) {
    let mut interval = interval(Duration::from_secs(5));

    let query_events =
        "select e.id, e.icon, e.title, c.title as channel_title, ev.name as user_name from event e left join channel c on e.channel_id = c.id join event_user ev on e.user_id = ev.id where c.project_id = $1";
    loop {
        interval.tick().await;
        let events = sqlx::query_as::<_, Event>(query_events)
            .bind(project_id)
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

async fn read(mut receiver: SplitStream<WebSocket>) {
    while let Some(message) = receiver.next().await {
        let message = match message {
            Ok(message) => message,
            Err(_) => return,
        };

        match message {
            Message::Text(text) => println!("Received: {}", text),
            Message::Close(_) => return,
            _ => (),
        }
    }
}

async fn handle_socket(mut socket: WebSocket, project_id: i32, state: AppState) {
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_err() {
        println!("Could not send ping!")
    }

    let (sender, receiver) = socket.split();

    let mut write_task = tokio::spawn(write(sender, project_id, state));
    let mut read_task = tokio::spawn(read(receiver));

    tokio::select! {
        _ = &mut write_task => read_task.abort(),
        _ = &mut read_task => write_task.abort(),
    }
}
