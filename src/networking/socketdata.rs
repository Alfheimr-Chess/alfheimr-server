use serde::Serialize;
use crate::{
    logic::{
        color::PieceColor,
        piece::PieceList,
        board::GameBoard,
        move_gen::GameMove,
    }
};

/// Board information to send to the client
type BoardData = Vec<Vec<Option<(String, u8)>>>;

/// This struct is serialized to json and sent to the client
#[derive(Serialize)]
#[serde(tag = "action", content = "data", rename_all = "snake_case")]
pub enum SocketData {
    /// Inital message to client
    NewClient(NewClient),
    /// Clients message was invalid
    InvalidMessage(String),
    /// Player has made a move
    Move(Move),
    /// Player Won
    Winner(Winner),
    /// Something went wrong
    Error(String),
}

/// Data sent to clients when someone wins
#[derive(Serialize)]
pub struct Winner {
    /// Winner of the game
    winner: u8,
    /// Final board layout
    board: BoardData,
}

/// Data to send on piece move
#[derive(Serialize)]
pub struct Move {
    /// Id of the player whos turn it is
    turn: u8,
    /// Current board layout
    board: BoardData,
    /// A list of valid moves
    moves: Vec<GameMove>,
}

/// Message to new client
#[derive(Serialize)]
pub struct NewClient {
    /// Name of game
    name: String,
    /// Type of client
    client_type: ClientType,
    /// List of piecetypes
    pieces: Vec<(String, String)>,
}

/// Type of client
#[derive(Serialize)]
#[serde(tag = "type", content = "color", rename_all = "snake_case")]
pub enum ClientType {
    /// A player where the string is the color the player is controlling
    Player(u8),
    /// Spectates the game without influencing it
    Spectator,
}

/// Creates a `NewClient` object
pub fn new_client(client_type: ClientType, pieces: &PieceList, name: &str) -> NewClient {
    NewClient {
        name: String::from(name),
        client_type,
        // Generates list of pieces
        pieces: pieces.iter()
            .map(|(key, value)| {
                (key.clone(), value.name.clone())
            })
            .collect(),
    }
}

/// Generate `BoardData` from `GameBoard`
pub fn generate_boarddata(board: &GameBoard) -> BoardData {
        board.board.iter()
            .map(|y| {
                y.iter().map(|x| {
                    x.as_ref().map(|piece| (piece.symbol.clone(), piece.color as u8) )
                }).collect()
            })
            .collect()
}

/// Generates `SocketData::Move`
pub fn create_move(turn: PieceColor, board: &GameBoard, moves: Vec<GameMove>) -> SocketData {
    SocketData::Move(Move{
        moves,
        turn: turn as u8,
        board: generate_boarddata(board),
    })
}

impl SocketData {

    /// Generates `SocketData::Winner`
    pub fn winner(winner: PieceColor, board: &GameBoard) -> Self {
        SocketData::Winner(Winner{
            winner: winner as u8,
            board: generate_boarddata(board),
        })
    }

}

