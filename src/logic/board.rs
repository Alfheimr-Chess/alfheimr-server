use pest::Parser;
use std::fmt;
use crate::{
    error::Error,
    logic::{
        piece::*,
        color::PieceColor,
        move_gen::GameMove,
    },
};

/// Struct for containing the board of a game, and the width and height
#[derive(Debug, Clone, Default)]
pub struct GameBoard {
    /// Horizontal size of board
    pub width: usize,
    /// Vertical size of board
    pub height: usize,
    /// Vectors of each row and cell in the board. First vector is vertical coordinates and second
    /// row is horizontal. The Option is None, if there isn't a piece in that position.
    pub board: Vec<Vec<Option<GamePiece>>>,
}

/// Simple struct for generating a gameboard from ffen-string.
/// Should only be used internally by ffen-parser
#[derive(Debug)]
pub struct GameBoardBuilder {
    pub current_x: usize,
    pub game_board: GameBoard,
}

impl GameBoardBuilder {
    /// Returns the build GameBoard
    pub fn get_board(self) -> GameBoard {
        self.game_board
    }

    /// Get a new, empty GameBoardBuilder
    pub fn new() -> GameBoardBuilder {
        GameBoardBuilder {
            current_x: 0,
            game_board: GameBoard::new(),
        }
    }

    /// Move the BoardBuilder to the next line of the board
    pub fn next_line(&mut self){
        self.current_x = 0;
        self.game_board.height += 1;
        self.game_board.board.push(vec![]);
    }

    /// Push `amount`, empty spaces to the board
    pub fn push_empty(&mut self, amount: usize) {
        for _ in 0..amount {
            self.current_x += 1;
            self.game_board.board[self.game_board.height-1].push(None);
        }
        if self.game_board.width < self.current_x {
            self.game_board.width = self.current_x;
        }
    }

    /// Push a piece to the board
    pub fn push_piece(&mut self, symbol: String) {
        self.current_x += 1;
        let col = if symbol == symbol.to_ascii_uppercase() {
            PieceColor::White
        } else {
            PieceColor::Black
        };
        self.game_board.board[self.game_board.height-1].push(Some(GamePiece{
            symbol: symbol.to_ascii_lowercase(),
            color: col,
            has_moved: false,
        }));
        if self.game_board.width < self.current_x {
            self.game_board.width = self.current_x;
        }
    }
}

/// Pest parser struct for ffen strings
#[derive(Parser,Debug)]
#[grammar = "parsers/ffen.pest"]
struct FFenParser;

impl GameBoard {

    /// Creates a `GameBoard` from a string with "Fairy Forsythe Edwards Notation" (ffen) syntax
    /// (https://www.chessvariants.com/programs.dir/ffen2htm.htm)
    pub fn from_ffen(ffen_raw_str: &str) -> Result<GameBoard, Error> {
        let ffen_str = ffen_raw_str.replace('\n', "").replace(' ', "");

        let ffen = match FFenParser::parse(Rule::ffen, &ffen_str) {
            Ok(mut x) => x.next().unwrap(),
            Err(_) => return Err(Error::FFenParse),
        };

        let mut board = GameBoardBuilder::new();

        use pest::iterators::Pair;

        fn parse(pair: Pair<Rule>, board: &mut GameBoardBuilder) {
            match pair.as_rule() {
                Rule::line |
                Rule::ffen => pair.into_inner().for_each(|p| parse(p, board)),
                Rule::newline => {
                    board.next_line();
                    parse(pair.into_inner().next().unwrap(), board)
                },
                Rule::fpiece => {
                    let piece = pair.into_inner().next().unwrap();
                    parse(piece, board);
                }
                Rule::long_piece | 
                Rule::piece => board.push_piece(pair.as_str().to_string()),
                Rule::empty => board.push_empty(pair.as_str().parse::<usize>().unwrap()),
            }
        }
        parse(ffen, &mut board);
        Ok(board.get_board())
    }

    /// Get a new, empty GameBoard
    pub fn new() -> GameBoard {
        GameBoard {
            width: 0,
            height: 1,
            board: vec![vec![]],
        }
    }

    /// Gets a list of positions of pieces matching a predicate (`pred`)
    pub fn get_positions_of(&self, pred: &dyn Fn(&GamePiece) -> bool) -> Vec<(usize, usize)> {
        let mut pieces = vec![];
        for y in 0..self.board.len() {
            for x in 0..self.board[y].len() {
                if let Some(piece) = &self.board[y][x] {
                    if pred(piece) {
                        pieces.push((x,y));
                    }
                }
            }
        }
        return pieces;
    }

    /// Moves a piece from one place to another and takes another piece if the new location is
    /// occupied by an opponents piece
    pub fn do_move(&mut self, mv: &GameMove) -> Result<bool, Error> {
        if mv.from.0 > self.width || mv.to.0 > self.width 
            || mv.from.1 > self.height || mv.to.1 > self.height {
            return Err(Error::InvalidMove);
        }
        let take = if self.board[mv.to.1][mv.to.0].is_some() {
            true
        } else {false};
        self.board[mv.to.1][mv.to.0] = self.board[mv.from.1][mv.from.0].take();
        match &mut self.board[mv.to.1][mv.to.0] {
            // Making move
            Some(x) => x.has_moved = true,
            None => return Err(Error::InvalidMove),
        }
        self.board[mv.from.1][mv.from.0] = None;

        Ok(take)
    }
}
impl fmt::Display for GameBoard {
    /// Simple text formatter for displaying a GameBoard in the terminal.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.board.len() {
            write!(f, "|")?;
            for x in 0..self.board[y].len() {
                if let Some(piece) = &self.board[y][x] {
                    if piece.color == PieceColor::White {
                        write!(f, "{}|", piece.symbol.to_ascii_uppercase())?;
                    } else {
                        write!(f, "{}|", piece.symbol)?;
                    }
                } else {
                    write!(f, " |")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
