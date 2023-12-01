use futures::{
    stream::{SplitSink, SplitStream, StreamExt},
    SinkExt,
};

use std::sync::Arc;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use axum_extra::{headers::UserAgent, TypedHeader};

use crate::AppState;

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

async fn write(mut sender: SplitSink<WebSocket, Message>) {
    let n_msg = 20;
    for i in 0..n_msg {
        if sender
            .send(Message::Text(format!("Server message {i} ...")))
            .await
            .is_err()
        {
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(300)).await
    }
}

async fn keep_alive(mut receiver: SplitStream<WebSocket>) {
    while let Some(Ok(_msg)) = receiver.next().await {}
}

async fn handle_socket(mut socket: WebSocket, _state: State<Arc<AppState>>) {
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        println!("Pinged...")
    } else {
        println!("Could not send ping!")
    }

    let (sender, receiver) = socket.split();

    let send_task = tokio::spawn(write(sender));
    let mut recv_task = tokio::spawn(keep_alive(receiver));

    // for later if i have a second task that checks db
    tokio::select! {
        _ = (&mut recv_task) => send_task.abort()
    }
}
