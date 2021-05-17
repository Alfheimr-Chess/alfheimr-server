use std::{
    borrow::Cow,
};
use futures::{
    stream::SplitSink,
    sink::SinkExt
};
use tokio_tungstenite::{
    WebSocketStream,
    tungstenite::{
        self,
        protocol::{
            CloseFrame,
            frame::coding::CloseCode
        },
    },
};
use super::socketdata::*;
use crate::logic::{
    piece::PieceList,
    color::PieceColor,
};

/// Shortcut for writing the outgoing connection
type OutgoingConnection = SplitSink<WebSocketStream<tokio::net::TcpStream>, tungstenite::Message>;

/// Stores information about a connection to a client
pub struct Client {
    /// Used to send data to the client
    outgoing: OutgoingConnection,
}

impl Client {

    /// Creates new instance of client
    pub fn new(outgoing: OutgoingConnection) -> Self {
        Self {
            outgoing,
        }
    }

    /// Send data through the websocket connection to the client
    pub async fn send(&mut self, s: String) {
        // TODO: Handle errors
        match self.outgoing.send(tungstenite::Message::Text(s)).await {
            Ok(_) => (),
            Err(e) => error!("Failed to send message to client: {}", e),
        }
    }

    /// Sends socketdata struct to client
    pub async fn send_socket(&mut self, socketdata: &SocketData) {
        self.send(serde_json::to_string(socketdata).unwrap()).await
    }

    /// Close the websocket connection to the client
    pub async fn close(&mut self, reason: &'static str) {
        self.outgoing.send(tungstenite::Message::Close(Some(
                    CloseFrame {
                        code: CloseCode::Away,
                        reason: Cow::from(reason),
                    }
                    ))).await.ok();
    }

    /// Sends new player msg to client
    pub async fn new_player(&mut self, color: PieceColor, pieces: &PieceList, name: &str) {
        self.send_socket(&SocketData::NewClient(
                new_client(ClientType::Player(color as u8), pieces, name)
            )).await;
    }

    /// Send new spectator msg to client
    pub async fn new_spectator(&mut self, pieces: &PieceList, name: &str) {
        self.send_socket(&SocketData::NewClient(
                new_client(ClientType::Spectator, pieces, name)
            )).await;
    }
}
