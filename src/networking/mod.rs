/// Sending data to a client
mod client;
/// Struct for storing data to send to clients
pub mod socketdata;
/// Structs for storing data received from clients
pub mod receiveddata;
/// Tests for networking
#[cfg(test)]
mod test;

use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
    collections::HashMap,
};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc,
};
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::{TryStreamExt, StreamExt};
pub use client::Client;

/// Message that is send to the main thread
pub struct Msg {
    pub addr: SocketAddr,
    pub data: MsgData,
}

/// Data to send to the main thread
pub enum MsgData {
    /// New connection from a client
    NewConnection,
    /// The client has send data to the server
    Data(receiveddata::ReceivedData),
    /// The client has closed the connection to the server
    ClosedConnection,
}

/// Type used for storing a list for clients
pub type ClientList = Arc<Mutex<HashMap<SocketAddr, Client>>>;

/// Handles connections to clients
pub async fn handle_connections(port: u16) -> (mpsc::Receiver<Msg>, ClientList) {
    let clients = Arc::new(Mutex::new(HashMap::new()));
    let (tx, rx) = mpsc::channel(32);
    tokio::spawn(listen_for_connections(tx, clients.clone(), port));
    return (rx, clients);
}

/// Listens for new connections and starts a new listener thread
async fn listen_for_connections(tx: mpsc::Sender<Msg>, clients: ClientList, port: u16) {
    let try_socket = TcpListener::bind(format!("0.0.0.0:{}", port)).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening for clients on port {}", port);
    while let Ok((stream, addr)) = listener.accept().await {
        let tx_clone = tx.clone();
        tokio::spawn(create_websocket(tx_clone, stream, addr, clients.clone()));
    }
}

/// Creates a new websocket connection and listening for new messages.
/// Messages reveived through the websocket is send throught the sender, `tx`.
async fn create_websocket(tx: mpsc::Sender<Msg>, stream: TcpStream, addr: SocketAddr, clients: ClientList) {
    debug!("Incomming connection from: {}", addr);
    // TODO: Handle error on websocket handshake
    // Creating websocket connection
    let ws_stream = match tokio_tungstenite::accept_async(stream).await {
        Ok(s) => s,
        Err(_) => {
            error!("Error during the websocket handshake occurred");
            return;
        }
    };
    let (outgoing, mut incoming) = ws_stream.split();
    // Adding client to clientlist
    if let Ok(mut x) = clients.lock() {
        x.insert(addr, Client::new(outgoing));
    }
    // Informing main thread of the new connection
    send_msg(&tx, MsgData::NewConnection, addr).await;
    // Listening for new messages on websocket
    while let Ok(msg) = incoming.try_next().await {
        if let Some(data) = msg {
            debug!("Message received from {}", addr);
            if let Message::Close(_) = data {
                break;
            }
            match parse_msg(&data) {
                Some(parsed_msg) => send_msg(&tx, parsed_msg, addr).await,
                None => {
                    if data.len() != 0 {
                        warn!("Received invalid message from {}", addr);
                    }
                },
            }
        }
    }
    debug!("Connection closed with {:?}", addr);
    send_msg(&tx, MsgData::ClosedConnection, addr).await;
}

/// Parses message received from client
fn parse_msg(msg: &tokio_tungstenite::tungstenite::Message) -> Option<MsgData> {
    Some(MsgData::Data(serde_json::from_str(msg.to_text().ok()?).ok()?))
}

/// Sends a message to the main thread
async fn send_msg(tx: &mpsc::Sender<Msg>, data: MsgData, addr: SocketAddr) {
    let result = tx.send(Msg {
        addr,
        data,
    }).await;
    match result {
        Ok(_) => (),
        Err(_) => error!("Could not send message to main thread"),
    }
}
