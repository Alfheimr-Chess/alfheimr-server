/// Color of a players pieces
#[repr(u8)]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum PieceColor {
    White = 0,
    Black = 1,
    Yellow = 2,
}

impl From<i64> for PieceColor {
    fn from(orig: i64) -> PieceColor {
        match orig {
            0 => PieceColor::Black,
            1 => PieceColor::White,
            _ => PieceColor::Yellow,
        }
    }
}
