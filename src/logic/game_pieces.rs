#![allow(dead_code)]
// Standard chess pieces
/// Standard chess King
pub const PIECE_KING: &str = "1*";
/// Standard chess Queen
pub const PIECE_QUEEN: &str = "n*";
/// Standard chess Knight
pub const PIECE_KNIGHT: &str = "~1/2";
/// Standard chess Rook
pub const PIECE_ROOK: &str = "n+";
/// Standard chess Bishop
pub const PIECE_BISHOP: &str = "nX";
/// Standard chess Pawn
pub const PIECE_PAWN: &str = "o1>,c1X>,oi2>";

// Some fairy chess pieces from terachess
pub const FAIRY_AMAZON: &str = "n*,~1/2";
pub const FAIRY_MARSHAL: &str = "n+,~1/2";
pub const FAIRY_CARDINAL: &str = "nX,~1/2";
pub const FAIRY_CENTAUR: &str = "1*,~1/2";
pub const FAIRY_ADMIRAL: &str = "n+,1*";
pub const FAIRY_MISSIONARY: &str = "nX,*1";



