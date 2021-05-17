#[macro_use] extern crate pest_derive;
#[macro_use] extern crate log;

/// Module that handles command line arguments
mod args;
/// Module with main error type for the server
mod error;
/// Handles connections with clients
mod networking;
/// General game logic
mod logic;
/// Generates rules for the game
mod rules;
/// Logging setup
mod logging;
/// Main part of game
mod game;

use tokio::{
    sync::mpsc,
};
use rules::RulesEnv;
use networking::{MsgData, Msg, ClientList};
use structopt::StructOpt;
use error::Error;
use game::Game;

/// Sets up the game
#[tokio::main]
async fn main() -> Result<(), Error> {
    let options = args::Arguments::from_args();
    logging::initialize_logging(&options);
    let rules = RulesEnv::new(&options.game)?;

    let (rx, clients) = networking::handle_connections(options.port).await;
    let clients_clone = clients.clone();
    ctrlc::set_handler(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        if let Ok(mut clients_unlocked) = clients_clone.lock() {
            info!("Server is shutting down");
            for client in clients_unlocked.values_mut() {
                rt.block_on(client.close("Server is shutting down"));
            }
            std::process::exit(1);
        }
    })
    .expect("Error setting Ctrl-C handler");
    debug!("Board: {:?}", rules.rules.board);
    game_loop(rx, clients, rules).await;
    Ok(())
}


/// Waits for messages from the client and handles the requests
async fn game_loop(mut rx: mpsc::Receiver<Msg>, clients: ClientList, rules: RulesEnv<'_>) {
    let mut game = Game::new(clients, rules);
    while let Some(msg) = rx.recv().await {
        match msg.data {
            MsgData::NewConnection => game.new_connection(&msg.addr).await,
            MsgData::ClosedConnection => game.closed_connection(&msg.addr),
            MsgData::Data(content) => game.handle_message(&content, &msg.addr).await,
        }
    }
}
