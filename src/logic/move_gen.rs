use crate::logic::board::GameBoard;
use crate::logic::piece::{GamePiece, PieceList};
use crate::logic::movement::*;
use crate::logic::color::PieceColor;
use serde::{Serialize, Deserialize};
use rhai::{Engine, AST, Scope, Dynamic, serde::from_dynamic};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, PartialOrd, Ord)]
pub struct GameMove {
    pub from: (usize, usize),
    pub to: (usize, usize),
}

impl GameMove {
    pub fn new(fx: usize, fy: usize, tx: usize, ty: usize) -> GameMove {
        GameMove {
            from: (fx,fy),
            to: (tx,ty),
        }
    }
}

pub fn generate_moves(color: PieceColor,
                      pieces: &PieceList,
                      board: &GameBoard,
                      rhai_env: Option<(&AST, &Engine)>,
                     ) -> Vec<GameMove> {
    let mut moves: Vec<GameMove> = vec![];
    for y in 0..board.height {
        for x in 0..board.width {
            if let Some(piece) = &board.board[y][x] {
                if piece.color == color {
                    gen_moves_for_piece(&color,&x,&y,&piece,pieces,board, &mut moves);
                    let piece_type = pieces.get(&piece.symbol).unwrap();
                    // Adding extra moves from rhai
                    if let Some((ast, engine)) = rhai_env {
                        let mut scope = Scope::new();
                        if let Some(extra_moves) = &piece_type.extra_moves {
                            let e: Vec<Dynamic> =
                                engine.call_fn(&mut scope, ast, &extra_moves, (x, y))
                                .unwrap();
                            e.iter()
                                .map(|m| {
                                    let a: [i64; 2] = from_dynamic(m).unwrap();
                                    GameMove::new(x, y, a[0] as usize, a[1] as usize)
                                })
                                .for_each(|m| moves.push(m));
                        }
                    }
                }
            }
        }
    }
    moves.sort();
    moves.dedup();
    return moves;
}

pub fn gen_moves_for_piece( color: &PieceColor,
                            x: &usize, 
                            y: &usize,
                            piece: &GamePiece,
                            pieces: &PieceList,
                            board: &GameBoard, 
                            moves: &mut Vec<GameMove>
                           ) {
    let movement_rules = &pieces[&piece.symbol].moves;

    for move_rule in movement_rules {
        for direction in &move_rule.direction {
            let mut dir_moves = gen_moves_from_dir(color, x, y, &move_rule, direction.get_coords(color, &move_rule.distance), board);
            moves.append(&mut dir_moves);
        }
    }
}
pub fn gen_moves_for_piece_with_rule( color: &PieceColor,
                            x: &usize, 
                            y: &usize,
                            move_rule: &Movement,
                            board: &GameBoard, 
                            moves: &mut Vec<GameMove>) {

    for direction in &move_rule.direction {
        let mut dir_moves = gen_moves_from_dir(color, x, y, &move_rule, direction.get_coords(color, &move_rule.distance), board);
        moves.append(&mut dir_moves);
    }
}


fn gen_moves_from_dir(color: &PieceColor, x: &usize, y: &usize, move_rule: &Movement, dirs: Vec<(i32, i32)>, board: &GameBoard)-> Vec<GameMove> {
    if move_rule.initial && board.board[*y][*x].as_ref().unwrap().has_moved {
        return vec![];
    }
    let mut moves = vec![];
    for dir in dirs {
        let mut mx = *x;
        let mut my = *y;
        let mut d_moved = 0;
        let mut has_leaped = false;
        'inner: loop {
            d_moved += 1;
            mx = (mx as i32 + dir.0) as usize;
            my = (my as i32 + dir.1) as usize;
            if mx < board.width && my < board.height {
                if move_rule.distance.correct(&d_moved) {
                    moves.push(GameMove::new(*x,*y,mx,my));
                    if let Some(end_piece) = &board.board[my][mx] {
                        if move_rule.nocapture || end_piece.color == *color || (move_rule.locust && !has_leaped) {
                            moves.pop();
                        }
                        if !move_rule.leaper && (!move_rule.locust || has_leaped) { 
                            break 'inner;
                        }
                        if move_rule.nocapture || end_piece.color == *color || (move_rule.locust && !has_leaped) {
                            has_leaped = true;
                        }
                        
                    }
                    else if move_rule.capture || (move_rule.locust && has_leaped){
                        moves.pop();
                    }
                } else {
                    if let Some(_) = &board.board[my][mx] {
                        if !move_rule.leaper && (!move_rule.locust || has_leaped) {
                            break 'inner;
                        }
                    }
                }
            } else {
                break 'inner;
            }
        }
    }
    if let Some(then) = &move_rule.then {
        let mut true_moves = vec![];
        for mov in moves {
            true_moves.push(mov.clone());
            gen_moves_for_piece_with_rule(color, &mov.to.0, &mov.to.1, then, board, &mut true_moves);
        }
        return true_moves.into_iter().map(|mut t| {t.from = (*x,*y); t}).collect();
    }
    return moves;
}

pub fn is_checked(p_color: PieceColor, dx: &usize, dy: &usize, pieces: &PieceList, board: &GameBoard) -> bool {
    let mut moves: Vec<GameMove> = vec![];
    let color = match p_color {
        PieceColor::White => PieceColor::Black,
        _ => PieceColor::White
    };
    for y in 0..board.height {
        for x in 0..board.width {
            if let Some(piece) = &board.board[y][x] {
                if piece.color == color {
                    gen_moves_for_piece(&color,&x,&y,&piece,pieces,board, &mut moves);
                    for mov in &moves {
                        if mov.to.0 == *dx && mov.to.1 == *dy {
                            return true;
                        }
                    }
                    moves.clear();
                }
            }
        }
    }
    false
}
pub fn get_checkers(p_color: PieceColor, dx: &usize, dy: &usize, pieces: &PieceList, board: &GameBoard) -> Vec<(usize,usize)> {
    let mut moves: Vec<GameMove> = vec![];
    let mut checkers: Vec<(usize,usize)> = vec![];
    let color = match p_color {
        PieceColor::White => PieceColor::Black,
        _ => PieceColor::White
    };
    for y in 0..board.height {
        for x in 0..board.width {
            if let Some(piece) = &board.board[y][x] {
                if piece.color == color {
                    gen_moves_for_piece(&color,&x,&y,&piece,pieces,board, &mut moves);
                    'piece: for mov in &moves {
                        if mov.to.0 == *dx && mov.to.1 == *dy {
                            checkers.push((mov.from.0, mov.from.1));
                            break 'piece;
                        }
                    }
                    moves.clear();
                }
            }
        }
    }
    checkers
}
