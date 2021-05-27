use super::Game;
use std::net::SocketAddr;

impl Game<'_> {

    /// Assigns a playertype to the newly connected client and sends initial data to the client. The
    /// procedure will also start the game if enough players are connected
    pub async fn new_connection(&mut self, addr: &SocketAddr) {
        info!("Connection established with {}", addr);
        // Finding available color
        let colors = self.rules.borrow().colors.clone();
        let new_color = colors.iter().find(|color| 
                self.players.iter().find(|(_, value)| Some(*color)==value.as_ref()).is_none());
        debug!("New color is: {:?}", new_color);
        // Sending initial data to client
        if let Ok(mut x) = self.clients.lock() {
            let client = x.get_mut(&addr).unwrap();
            match new_color {
                Some(color) => client.new_player(
                    *color,
                    &self.rules.borrow().pieces,
                    &self.rules.borrow().name).await,
                None => client.new_spectator(
                    &self.rules.borrow().pieces,
                    &self.rules.borrow().name).await,
            }
        }
        // Mapping address to color
        self.players.insert(*addr, new_color.map_or_else(|| None, |x| Some(*x)));
        debug!("{} clients connected", self.players.len());
        // Starting game if enough players are connected and the game hasn't started yet
        if !self.game_started && self.players.len() == self.rules.borrow().colors.len() {
            info!("Starting game");
            self.game_started = true;
            self.current_player = Some(self.rules.borrow().colors[0]);
            let msg = self.create_move(self.current_player.unwrap());
            self.send_msg(msg).await;
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
