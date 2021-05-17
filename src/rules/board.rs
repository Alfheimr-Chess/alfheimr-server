use std::{
    rc::Rc,
    cell::RefCell,
};
use crate::logic::{
    piece::GamePiece,
    board::GameBoard,
    move_gen::GameMove,
};
use rhai::{Engine, Dynamic, Array, EvalAltResult};

/// Shared reference to a GameBoard
type SharedBoard = Rc<RefCell<GameBoard>>;

/// Sets up methods for the gameboard and other structs
pub fn setup_methods(engine: &mut Engine, board: &SharedBoard) {
    let board_clone = board.clone();
    engine.register_type_with_name::<SharedBoard>("GameBoard")
        .register_fn("get_board", move || board_clone.clone())
        .register_fn("contains_piece", contains_piece)
        .register_result_fn("get_piece", get_piece)
        .register_fn("add_piece", add_piece)
        .register_fn("remove_piece", remove_piece);
    engine.register_type::<GameMove>()
        .register_get("points", get_points);
}

/// Checks if a position contains a `GamePiece`
fn contains_piece(sharedboard: SharedBoard, x: i64, y: i64) -> bool {
    let board = sharedboard.borrow();
    if x < 0 || x >= board.width as i64 || y < 0 || y >= board.height as i64 {
        return false;
    }
    board.board[y as usize][x as usize].is_some()
}

/// Returns `GamePiece` if it exists on the given position
fn get_piece(board: SharedBoard, x: i64, y: i64) -> Result<GamePiece, Box<EvalAltResult>> {
    board.borrow_mut().board[y as usize][x as usize].clone().ok_or("No piece".into())
}

/// Adds a piece to the board
fn add_piece(board: SharedBoard, color: i64, piece: String, x: i64, y: i64) {
    board.borrow_mut().board[y as usize][x as usize] = Some(GamePiece{
        symbol: piece,
        color: color.into(),
        has_moved: false,
    });
}

/// Removes a piece from the board
fn remove_piece(sharedboard: SharedBoard, x: i64, y: i64) {
    let mut board = sharedboard.borrow_mut();
    if !(x < 0 || x >= board.width as i64 || y < 0 || y >= board.height as i64) {
        board.board[y as usize][x as usize] = None;
    }
}

/// Returns an `Array` of the coordinates
fn get_points(m: &mut GameMove) -> Array {
    vec![
        Dynamic::from(m.from.0 as i64),
        Dynamic::from(m.from.1 as i64),
        Dynamic::from(m.to.0 as i64),
        Dynamic::from(m.to.1 as i64)
    ]
}
