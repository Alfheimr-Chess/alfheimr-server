use simplelog::LevelFilter;
use structopt::StructOpt;

#[derive(StructOpt)]
/// A modifiable chess game
pub struct Arguments {
    /// Port to host server on
    #[structopt(short, long, default_value="1126")]
    pub port: u16,
    /// Lua file for game
    #[structopt(short, long, default_value="./game.rhai")]
    pub game: String,
    /// Hide startup banner
    #[structopt(long)]
    pub no_startup_banner: bool,
    /// Loglevel
    #[structopt(short, long, default_value="info")]
    pub loglevel: LevelFilter,
}
