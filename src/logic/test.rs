use crate::logic::*;
use crate::logic::move_gen::*;
use crate::logic::color::PieceColor;
use crate::Error;

/// Test to ensure pawns cant leap when they move 2 forward
#[test]
fn pawn_not_leaper() {
    let pieces = helper_get_standard_pieces();
    let board = board::GameBoard::from_ffen("4/pppp/PPPP/4").unwrap();

    let moves = generate_moves(PieceColor::White, &pieces, &board, None);
    assert_eq!(moves.len(), 6);
}

/// Test depth 1 of movegen
#[test]
fn perft_depth_1_moves() {
    let pieces = helper_get_standard_pieces();
    let chess_board = board::GameBoard::from_ffen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap();

    let moves = generate_moves(PieceColor::White, &pieces, &chess_board, None);
    assert_eq!(moves.len(), 20);
}

/// Test depth 2 of movegen
#[test]
fn perft_depth_2_moves() {
    let pieces = helper_get_standard_pieces();
    let board = board::GameBoard::from_ffen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap();

    assert_eq!(perft(2, PieceColor::White, &pieces, &board), 400);
}

/// Test depth 3 of movegen
#[test]
fn perft_depth_3_moves() {
    let pieces = helper_get_standard_pieces();
    let board = board::GameBoard::from_ffen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap();

    assert_eq!(perft(3, PieceColor::White, &pieces, &board), 8902);
}

fn perft(depth: u32, color: PieceColor, pieces: &piece::PieceList, board: &board::GameBoard) -> u32 {
    if depth == 0 {
        return 1;
    }
    let opp_color = match color {
        PieceColor::White => PieceColor::Black,
        PieceColor::Black => PieceColor::White,
        _ => unreachable!(),
    };
    let moves = generate_moves(color, &pieces, &board, None);
    let mut n_pos = 0;

    for mov in moves {
        let mut nboard = board.clone();
        assert!(!nboard.do_move(&mov).is_err());
        let king = get_position_of_piece(&nboard, color, "k".to_string()).unwrap();
        if !is_checked(color, &king.0, &king.1, pieces, &nboard) {
            n_pos += perft(depth - 1, opp_color, pieces, &nboard);
        }
    }

    return n_pos;
}

#[test]
fn diagonal_movement() {
    let pieces = helper_get_standard_pieces();
    let board = board::GameBoard::from_ffen("8/2b5/8/8/8/1B4B1/8/8").unwrap();
    let moves = generate_moves(PieceColor::White, &pieces, &board, None);
    assert_eq!(moves.len(), 17);
}

#[test]
fn orthogonal_movegen() {
    let pieces = helper_get_standard_pieces();
    let mut board = board::GameBoard::from_ffen("nnnn/4/4/NNNN").unwrap();

    let moves = generate_moves(PieceColor::White, &pieces, &board, None);
    assert_eq!(board.do_move(&moves[0]).is_ok(), true);

    assert!(board.board[1][1].as_ref().unwrap().has_moved);        
}

/// Test move from position with no piece
#[test]
fn invalid_move() {
    let mut board = board::GameBoard::from_ffen("nnnn/4/4/NNNN").unwrap();
    assert_eq!(board.do_move(&move_gen::GameMove{ from: (0,2), to: (1,1)}).is_ok(), false);
} 
#[test]
fn get_checkers_fn() {
    let pieces = helper_get_standard_pieces();
    // Setup board for test
    let board = board::GameBoard::from_ffen("nnRn/1N1R/2N1/4").unwrap();

    assert_eq!(get_checkers(PieceColor::Black, &3,&0,&pieces, &board), [(2, 0), (1, 1), (3, 1), (2, 2)]);
}

#[test]
fn distance_correct_set() {
    let mut pieces = piece::PieceList::new();
    pieces.insert("1".to_string(), piece::Piece::from_parlett("", 0.0, "1>").unwrap());
    pieces.insert("5".to_string(), piece::Piece::from_parlett("", 0.0, "5>").unwrap());
    pieces.insert("7".to_string(), piece::Piece::from_parlett("", 0.0, "7>").unwrap());
    pieces.insert("999".to_string(), piece::Piece::from_parlett("", 0.0, "999>").unwrap());

    assert!(pieces["1"].moves[0].distance.correct(&1));
    assert!(pieces["5"].moves[0].distance.correct(&5));
    assert!(pieces["7"].moves[0].distance.correct(&7));
    assert!(pieces["999"].moves[0].distance.correct(&999));
}

#[test]
fn distance_correct_range() {
    let mut pieces = piece::PieceList::new();
    pieces.insert("r1-5".to_string(), piece::Piece::from_parlett("", 0.0, "1-5>").unwrap());
    pieces.insert("r10-100".to_string(), piece::Piece::from_parlett("", 0.0, "10-100>").unwrap());
    pieces.insert("r20-40".to_string(), piece::Piece::from_parlett("", 0.0, "20-40>").unwrap());
    pieces.insert("r9-9".to_string(), piece::Piece::from_parlett("", 0.0, "9-9>").unwrap());

    for i in 1..6 {
        assert!(pieces["r1-5"].moves[0].distance.correct(&i));
    }
    for i in 10..101 {
        assert!(pieces["r10-100"].moves[0].distance.correct(&i));
    }
    for i in 20..40 {
        assert!(pieces["r20-40"].moves[0].distance.correct(&i));
    }
    for i in 9..10 {
        assert!(pieces["r9-9"].moves[0].distance.correct(&i));
    }
}

/// Get the first found position of a piece
fn get_position_of_piece(board: &board::GameBoard, color: PieceColor, symbol: String) -> Result<(usize, usize), Error> {
    for y in 0..board.board.len() {
        for x in 0..board.board[y].len() {
            if let Some(piece) = &board.board[y][x] {
                if piece.symbol == symbol && piece.color == color {
                    return Ok((x,y));
                }
            }
        }
    }
    Err(Error::InvalidMove)
}

/// Returns a list of standard pieces
fn helper_get_standard_pieces() -> piece::PieceList {
    let mut pieces = piece::PieceList::new();
    pieces.insert("n".to_string(), piece::Piece::from_parlett("Knight", 3.0, "~1/2").unwrap());
    pieces.insert("r".to_string(), piece::Piece::from_parlett("Rook", 5.0, "n+").unwrap());
    pieces.insert("p".to_string(), piece::Piece::from_parlett("Pawn", 1.0, "o1>,oi2>,c1X>").unwrap());
    pieces.insert("b".to_string(), piece::Piece::from_parlett("Bishop", 3.0, "nX").unwrap());
    pieces.insert("q".to_string(), piece::Piece::from_parlett("Queen", 8.0, "n*").unwrap());
    pieces.insert("k".to_string(), piece::Piece::from_parlett("King", 100.0, "1*").unwrap());
    return pieces;
}
