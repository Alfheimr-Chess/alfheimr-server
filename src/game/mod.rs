/// Handles new and closed connections
mod connection;
/// Validates user actions
mod validation;

use crate::{
    Error,
    rules::{self, RulesEnv, SharedRules},
    networking::{
        self,
        receiveddata::ReceivedData,
        socketdata::SocketData,
        ClientList,
    },
    logic::{
        board::GameBoard,
        color::PieceColor,
        piece::GamePiece,
        move_gen::{GameMove, generate_moves, is_checked},
    },
};
use std::{
    net::SocketAddr,
    collections::HashMap,
    rc::Rc,
    cell::RefCell,
};
use rhai::{Engine, AST, Scope};

/// Type for mapping client addresses to player colors
type PlayerList = HashMap<SocketAddr, Option<PieceColor>>;

/// Struct for storing the global state of the game
pub struct Game<'a> {
    players: PlayerList,
    clients: ClientList,
    board: Rc<RefCell<GameBoard>>,
    game_started: bool,
    current_player: Option<PieceColor>,
    rules: SharedRules,
    engine: Engine,
    ast: AST,
    scope: Scope<'a>,
    winner: rules::WinnerState,
}

/// Stores which players a message should be sent to
enum SelectedPlayers {
    All,
    List(Vec<SocketAddr>),
}

/// Stores message and which players it should be sent to
struct PlayerMessage {
    players: SelectedPlayers,
    message: networking::socketdata::SocketData,
}

impl<'a> Game<'a> {

    /// Creates a new game
    pub fn new(clients: ClientList, env: RulesEnv<'a>) -> Game<'a> {
        info!("Game: {}", env.rules.borrow().name);
        let board = env.rules.borrow().board.clone();
        Self {
            clients,
            board,
            players: HashMap::new(),
            game_started: false,
            current_player: None,
            rules: env.rules,
            engine: env.engine,
            ast: env.ast,
            scope: env.scope,
            winner: env.winner,
        }
    }

    /// Handles new messages from clients
    pub async fn handle_message(&mut self, content: &ReceivedData, addr: &SocketAddr) {
        let player_msg = match self.evaluate_message(content, addr) {
            Ok(msg) => msg,
            Err(e) => {
                error!("{}", e);
                PlayerMessage::all_players(SocketData::Error(e.to_string()))
            },
        };
        self.send_msg(player_msg).await;
    }

    /// Evaluates message from a client and returns a response
    fn evaluate_message(&mut self, content: &ReceivedData, addr: &SocketAddr) -> Result<PlayerMessage, Error> {
        match content {
            ReceivedData::Move(_) => self.evaluate_move(content, addr),
        }
    }

    /// Evaluates a move from a player
    fn evaluate_move(&mut self, content: &ReceivedData, addr: &SocketAddr) -> Result<PlayerMessage, Error> {
        // Validates message
        if let Err(err_msg) = self.validate_message(content, addr) {
            return Ok(PlayerMessage::single_player(*addr, SocketData::InvalidMessage(err_msg)));
        }
        debug!("Message validated");
        let ReceivedData::Move(gamemove) = content;
        info!("{:?} made move from {:?} to {:?}", self.current_player.unwrap(), gamemove.from, gamemove.to);
        self.do_move(gamemove)?;
        let colors = &self.rules.borrow().colors;
        let next_player = colors[(self.current_player.unwrap() as usize + 1) % colors.len()];
        self.current_player = Some(next_player);
        // Checking for winners
        if let Some(winner) = self.find_winner().or(*self.winner.borrow())  {
            info!("Winner: {:?}", winner);
            return Ok(PlayerMessage::all_players(SocketData::winner(winner, &self.board.borrow())));
        }
        return Ok(self.create_move(next_player));
    }

    /// Sends message to specied players
    async fn send_msg(&mut self, msg: PlayerMessage) {
        if let Ok(mut x) = self.clients.lock() {
            match msg.players {
                SelectedPlayers::All => {
                    for i in x.values_mut() {
                        i.send_socket(&msg.message).await;
                    }
                },
                SelectedPlayers::List(players) => {
                    for player in players {
                        x.get_mut(&player).unwrap().send_socket(&msg.message).await;
                    }
                }
            }
        }
    }

    /// Makes move and runs rhai functions
    fn do_move(&mut self, gamemove: &GameMove) -> Result<(), Error> {
        // Finds piece that is moved
        let from = gamemove.from;
        let move_piece_symbol = &self.board.borrow().board[from.1][from.0].as_ref().unwrap().symbol.clone();
        let move_piece = &self.rules.borrow().pieces[move_piece_symbol];
        // Does move
        let take = self.board.borrow_mut().do_move(&gamemove)?;
        // Runs events
        if let Some(after_move) = &move_piece.after_move {
            let _: () = self.engine.call_fn(&mut self.scope,
                                            &self.ast,
                                            &after_move,
                                            (gamemove.clone(), )).unwrap();
        }
        if take {
            if let Some(after_take) = &move_piece.after_take {
                let _: () = self.engine.call_fn(&mut self.scope,
                                                &self.ast,
                                                &after_take,
                                                (gamemove.clone(), )).unwrap();
            }
        }
        Ok(())
    }

    /// Finds a winner if it exists
    fn find_winner(&self) -> Option<PieceColor> {
        let king_pieces = self.board.borrow().get_positions_of(&|piece: &GamePiece| {
            piece.color == self.current_player.unwrap() && self.rules.borrow().pieces[&piece.symbol].kingstatus
        });
        let opp_color = self.rules.borrow().colors[((self.current_player? as usize + 1) % self.rules.borrow().colors.len()) as usize];
        for (x,y) in king_pieces {
            if is_checked(self.current_player?, &x, &y, &self.rules.borrow().pieces, &self.board.borrow()) {
                debug!("King, {:?}, is checked", (x,y));
                let valid_moves: Vec<GameMove> = generate_moves(self.current_player?, &self.rules.borrow().pieces, &self.board.borrow(), Some((&self.ast, &self.engine)))
                    .into_iter().filter(|mov| self.king_checked_predicate(mov)).collect();
                if valid_moves.len() > 0 {
                    debug!("King can be saved: {:?}", valid_moves);
                    return None;
                }
                debug!("King has no legal moves!");
                return Some(opp_color);
            }
        }
        return None;
    }

    /// Creates move data with board state
    fn create_move(&self, turn: PieceColor) -> PlayerMessage {
        let valid_moves = generate_moves(turn, &self.rules.borrow().pieces, &self.board.borrow(), Some((&self.ast, &self.engine)))
            .into_iter().filter(|mov| self.king_checked_predicate(mov)).collect();
        let data = networking::socketdata::create_move(turn, &self.board.borrow(), valid_moves);
        return PlayerMessage::all_players(data);
    }

    /// Predicate used to check if king is checked after move
    fn king_checked_predicate(&self, mov: &GameMove) -> bool {
        let mut nboard = self.board.borrow().clone();
        nboard.do_move(&mov);
        let king_pieces = nboard.get_positions_of(&|piece: &GamePiece| {
            piece.color == self.current_player.unwrap() && self.rules.borrow().pieces[&piece.symbol].kingstatus
        });
        for (kx, ky) in &king_pieces {
            if is_checked(self.current_player.unwrap(), kx, ky, &self.rules.borrow().pieces, &nboard) {
                return false;
            }
        }
        return true;
    }
}

impl PlayerMessage {

    /// Create `PlayerMessage` for all players
    fn all_players(msg: SocketData) -> Self {
        PlayerMessage{
            players: SelectedPlayers::All,
            message: msg,
        }
    }

    /// Create `PlayerMessage` for a single player
    fn single_player(player: SocketAddr, msg: SocketData) -> Self {
        PlayerMessage {
            players: SelectedPlayers::List(vec![player]),
            message: msg,
        }
    }

}
