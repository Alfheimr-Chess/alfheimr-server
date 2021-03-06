/// Rhai methods for the board
mod board;
/// Rhai methods for `GamePiece`
mod piece;
/// Rhai functions for the game
mod game;

use crate::{
    Error,
    logic::{
        color::PieceColor,
        piece::{
            Piece,
            PieceList,
        },
        board::GameBoard,
    },
};
use std::{
    rc::Rc,
    cell::RefCell,
};
use rhai::{Engine, AST, Scope, FnPtr};

/// Shared reference to the current winner
pub type WinnerState = Rc<RefCell<Option<PieceColor>>>;

/// Rhai environment for rules
pub struct RulesEnv<'a> {
    /// User defined rules
    pub rules: SharedRules,
    /// Compiles rhai code
    pub ast: AST,
    /// Engine to run rhai code
    pub engine: Engine,
    /// Variables to be available when running user functions
    pub scope: Scope<'a>,
    /// State of the winner
    pub winner: WinnerState,
}

/// Stores game rules that are generated through rhai.
#[derive(Default, Clone)]
pub struct Rules {
    pub name: String,
    pub pieces: PieceList,
    pub board: Rc<RefCell<GameBoard>>,
    pub colors: Vec<PieceColor>,
}

/// Shared reference to rules.
pub type SharedRules = Rc<RefCell<Rules>>;

impl RulesEnv<'_> {

    /// Creates a new instance of a `RulesEnv` object from a lua script
    pub fn new(config: &str) -> Result<Self, Error> {
        // Setting up engine
        let mut engine = Engine::new();
        engine.register_type::<Rules>()
            .register_fn("rules", SharedRules::default)
            .register_fn("set_name", Rules::set_name)
            .register_fn("add_piece", Rules::add_piece)
            .register_fn("create_board", Rules::create_board)
            .register_fn("after_move", Rules::after_move)
            .register_fn("after_take", Rules::after_take)
            .register_fn("add_moves", Rules::add_moves)
            .register_fn("set_kingstatus", Rules::set_kingstatus);
        // Retrieving information from engine
        let ast = engine.compile_file(config.into())?;
        let rules = engine.eval_ast::<SharedRules>(&ast)?;
        // Creating game variables
        let winner = Rc::new(RefCell::new(None));
        // Adding game functions
        game::setup_functions(&mut engine, winner.clone());
        board::setup_methods(&mut engine, &rules.borrow().board);
        piece::setup_methods(&mut engine);
        // Return Self
        rules.borrow_mut().colors = vec![PieceColor::White, PieceColor::Black];
        return Ok(RulesEnv {
            ast,
            engine,
            rules: rules,
            scope: Scope::new(),
            winner: winner,
        });
    }
}

impl Rules {

    /// Adds a new piece from a parlett string to the rules
    fn add_piece(rules: Rc<RefCell<Rules>>, id: &str, name: &str, parlett: &str) {
        let piece = Piece::from_parlett(name, 0.0, parlett).unwrap();
        rules.borrow_mut().pieces.insert(id.to_string(), piece);
    }

    /// Sets name of ruleset
    fn set_name(rules: Rc<RefCell<Rules>>, name: &str) {
        rules.borrow_mut().name = String::from(name);
    }

    /// Creates a new board from a ffen string
    fn create_board(rules: Rc<RefCell<Rules>>, ffen: &str) {
        rules.borrow_mut().board = Rc::new(RefCell::new(GameBoard::from_ffen(ffen).unwrap()));
    }

    /// Adds a new function (`f`) to be run after a move to `piece`
    fn after_move(rules: Rc<RefCell<Rules>>, piece: &str, f: FnPtr) {
        rules.borrow_mut().pieces.get_mut(piece).unwrap().after_move = Some(f.fn_name().to_string());
    }

    fn after_take(rules: Rc<RefCell<Rules>>, piece: &str, f: FnPtr) {
        rules.borrow_mut().pieces.get_mut(piece).unwrap().after_take = Some(f.fn_name().to_string());
    }

    /// Adds additional moves to a piece
    fn add_moves(rules: Rc<RefCell<Rules>>, piece: &str, f: FnPtr) {
        rules.borrow_mut().pieces.get_mut(piece).unwrap().extra_moves = Some(f.fn_name().to_string());
    }

    /// Sets a piece to be treated as a king
    fn set_kingstatus(rules: Rc<RefCell<Rules>>, piece: &str, value: bool) {
        rules.borrow_mut().pieces.get_mut(piece).unwrap().kingstatus = value;
    }
}
