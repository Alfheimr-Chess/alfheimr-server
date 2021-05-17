use super::Game;
use std::net::SocketAddr;

impl Game<'_> {

    /// Assigns a playertype to the newly connected client and sends initial data to the client. The
    /// procedure will also start the game if enough players are connected
    pub async fn new_connection(&mut self, addr: &SocketAddr) {
        info!("Connection established with {}", addr);
        // Finding available color
        let new_color = self.rules.colors.iter()
            .find(|color| self.players.iter().find(|(_, value)| Some(*color)==value.as_ref()).is_none());
        debug!("New color is: {:?}", new_color);
        // Sending initial data to client
        if let Ok(mut x) = self.clients.lock() {
            let client = x.get_mut(&addr).unwrap();
            match new_color {
                Some(color) => client.new_player(
                    *color,
                    &self.rules.pieces,
                    &self.rules.name).await,
                None => client.new_spectator(
                    &self.rules.pieces,
                    &self.rules.name).await,
            }
        }
        // Mapping address to color
        self.players.insert(*addr, new_color.map_or_else(|| None, |x| Some(*x)));
        debug!("{} clients connected", self.players.len());
        // Starting game if enough players are connected and the game hasn't started yet
        if !self.game_started && self.players.len() == self.rules.colors.len() {
            info!("Starting game");
            self.game_started = true;
            self.current_player = Some(self.rules.colors[0]);
            self.send_msg(self.create_move(self.rules.colors[0])).await;
        // Sending board state to new client if the game is already started
        } else if self.game_started {
            self.send_msg(self.create_move(self.current_player.unwrap())).await;
        }
    }

    /// Removes client from player list when connection is closed
    pub fn closed_connection(&mut self, addr: &SocketAddr) {
        if let Some(color) = self.players.get(addr).unwrap() {
            info!("Player {} disconnected", *color as u8);
        } else {
            info!("Spectator disconnected");
        }
        self.players.remove(addr);
        if let Ok(mut x) = self.clients.lock() {
            x.remove(addr);
        }
        debug!("{} clients connected", self.players.len());
    }

}
