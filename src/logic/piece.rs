use crate::error::Error;
use crate::logic::movement::Movement;
use crate::logic::color::PieceColor;
use std::{
    collections::HashMap,
};

/// A struct for describing a general piece with any number
/// of movement rules.
#[derive(Debug, Clone)]
pub struct Piece {
    pub name: String,
    pub value: f32,
    pub moves: Vec<Movement>,
    pub after_move: Option<String>,
    pub after_take: Option<String>,
    pub extra_moves: Option<String>,
    pub kingstatus: bool,
}

#[derive(Debug, Clone)]
pub struct GamePiece {
    pub symbol: String,
    pub color: PieceColor,
    pub has_moved: bool,
}

impl Piece {
    /// Create a piece, from a parlett string with any number
    /// of movement rules.
    pub fn from_parlett(p_name: &str, value: f32, parlett: &str) -> Result<Piece, Error> {
        let split = parlett.split(',');
        let name = p_name.to_string();
        // TODO: Remove unwrap
        let moves = split.map(|x| Movement::from_parlett(x).unwrap()).collect::<Vec<Movement>>();

        Ok(Piece {
            name,
            value,
            moves,
            after_move: None,
            after_take: None,
            extra_moves: None,
            kingstatus: false,
        })
    }
}

pub type PieceList = HashMap<String, Piece>;
