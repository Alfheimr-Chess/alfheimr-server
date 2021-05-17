use super::WinnerState;
use rhai::Engine;


/// Adds general functions to the game
pub fn setup_functions(engine: &mut Engine, winner: WinnerState) {
    engine.register_fn("set_winner", move |color: i64| {
        winner.replace(Some(color.into()));
    });
}
