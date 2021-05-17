/// A representation of a chess, or chess-like, piece
pub mod piece;
/// A representation of a rectangular chessboard of any size
pub mod board;
/// Descriptions for how a piece can move
pub mod movement;
/// Movement generation from movement descriptors
pub mod move_gen;
pub mod color;
#[cfg(test)]
mod test;
