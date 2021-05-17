use serde::Deserialize;
use crate::logic::move_gen::GameMove;

/// Data received from clients
#[derive(Deserialize, Debug)]
#[serde(tag = "action", content = "data", rename_all = "snake_case")]
pub enum ReceivedData {
    Move(GameMove),
}
