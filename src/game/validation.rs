use super::{Game};
use crate::{
    networking::{receiveddata::ReceivedData},
    logic::{
        move_gen::generate_moves,
    }
};
use std::{net::SocketAddr};

impl Game<'_> {

    /// Validates a message from a client or returns error message
    pub fn validate_message(&mut self, content: &ReceivedData, addr: &SocketAddr) -> Result<(), String> {
        return Err(String::from(
                if !self.game_started {
                    "Game has not started yet"
                } else if self.players.get(addr).is_none() {
                    "Client has not connected"
                } else if !(self.current_player.unwrap() == self.players.get(addr).unwrap().unwrap()) {
                    "It is not you turn"
                } else if !self.is_valid_move(content, addr) {
                    "Move is not valid"
                } else { return Ok(()); }
                ));
    }

    /// Checks if the move given move is valid
    fn is_valid_move(&mut self, content: &ReceivedData, addr: &SocketAddr) -> bool {
        match content {
            ReceivedData::Move(given_move) => {
                generate_moves(self.players.get(addr).unwrap().unwrap(),
                &self.rules.borrow().pieces,
                &self.board.borrow(),
                Some((&self.ast, &self.engine)))
                    .iter()
                    .find(|valid_move| &given_move == valid_move).is_some()
            },
        }
    }

}
