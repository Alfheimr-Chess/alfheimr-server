use crate::logic::{
    piece::GamePiece,
};
use rhai::Engine;

pub fn setup_methods(engine: &mut Engine) {
    engine.register_type::<GamePiece>()
        .register_get("color", GamePiece::get_color)
        .register_get("symbol", |piece: &mut GamePiece| piece.symbol.clone());
}

impl GamePiece {
    fn get_color(&mut self) -> i64 {
        self.color as i64
    }
}
