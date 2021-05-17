use pest::Parser;
use crate::error::Error;
use crate::logic::color::PieceColor;

/// A struct describing a movement rule for a piece
#[derive(Debug, Clone)]
pub struct Movement {
    pub distance: Distance,
    pub direction: Vec<Direction>,
    pub then: Option<Box<Movement>>,
    pub repeat: Option<Distance>,
    pub initial: bool,
    pub capture: bool,
    pub nocapture: bool,
    pub leaper: bool,
    pub locust: bool,
}

/// Enum describing the different direction a piece can move.
#[derive(Debug, Clone)]
pub enum Direction {
    All,
    Orthogonal,
    OrthogonalForward,
    OrthogonalBackward,
    OrthogonalSideways,
    Diagonal,
    DiagonalForward,
    DiagonalBackward,
    Hippogonal,
    Movement(Box<Movement>)
}

/// Enum describing different options for a distance a piece 
/// can travel. `N` being any distance, `Set` being a specific
/// number of squares, `Range` being a range between `min` and 
/// `max` and `hippogonal` moving like a knight, going first `m`
/// squares in one orthogonal direction, and then `n` the the 
/// other.
#[derive(Debug, Clone)]
pub enum Distance {
    N,
    Set(usize),
    Range {
        min: usize,
        max: usize,
    },
    Hippogonal {
        m: usize,
        n: usize,
    }
}

impl Distance {
    pub fn correct(&self, len: &usize) -> bool {
        match self {
            Distance::N => true,
            Distance::Set(x) => len == x,
            Distance::Range{min, max} => len >= min && len <= max,
            Distance::Hippogonal{m: _,n: _} => *len == 1,
        }
    }    
}


/// Pest parser to Parlet
#[derive(Parser, Debug)]
#[grammar = "parsers/parlett.pest"] // relativ til src/
struct ParlettParser;

impl Movement {
    /// Create a single `Movement` to piece from parlett string, using
    /// a pest parser.
    pub fn from_parlett(parlett_str: &str) -> Result<Movement, Error> {

        let parlett = match ParlettParser::parse(Rule::parlett, parlett_str) {
            Ok(mut x) => x.next().unwrap(),
            Err(_) => {
                error!("Failed to parse parlett: {}", parlett_str);
                return Err(Error::ParlettParse)
            },
        };
        let mut movement = Movement::new();

        use pest::iterators::Pair;

        fn parse(pair: Pair<Rule>, movem: &mut Movement){
            match pair.as_rule() {
                Rule::movement |
                Rule::smove |
                Rule::parlett => pair.into_inner().for_each(|p| parse(p, movem)),
                Rule::then => {
                    let mut mov = Movement::new();
                    let p = pair.into_inner().next().unwrap();
                    parse(p, &mut mov);
                    movem.then = Some(Box::new(mov));
                },
                Rule::group => {
                    let mut smove = Movement::new();
                    let p = pair.into_inner().next().unwrap();
                    parse(p, &mut smove);
                    movem.direction.push(Direction::Movement(Box::new(smove)));
                }
                Rule::property => {
                    match pair.as_str() {
                        "i" => movem.initial = true,
                        "c" => movem.capture = true,
                        "o" => movem.nocapture = true,
                        "&" => movem.repeat = Some(Distance::N),
                        _ => unreachable!(),
                    }
                },
                Rule::special => {
                    match pair.as_str() {
                        "~" => movem.leaper = true,
                        "^" => movem.locust = true,
                        _ => unreachable!(),
                    }
                },
                Rule::range => {
                    let mut splt = pair.as_str().split('-');
                    movem.distance = Distance::Range {
                        min: splt.next().unwrap().parse::<usize>().unwrap(),
                        max: splt.next().unwrap().parse::<usize>().unwrap()
                    }
                }
                Rule::distance => {
                    match pair.as_str() {
                        "n" => movem.distance = Distance::N,
                        _ => movem.distance = Distance::Set(pair.as_str().parse::<usize>().unwrap()),
                    }
                },
                Rule::hippogonal => {
                    let mut splt = pair.as_str().split('/');
                    movem.direction.push(Direction::Hippogonal);
                    movem.distance = Distance::Hippogonal {
                        m: splt.next().unwrap().parse::<usize>().unwrap(),
                        n: splt.next().unwrap().parse::<usize>().unwrap()
                    } 
                }
                Rule::direction => movem.direction.push(Direction::from_str(pair.as_str())),
                Rule::directions => pair.into_inner().for_each(|x| parse(x, movem)),
            }
        } 

        parse(parlett, &mut movement);
        Ok(movement)
    }

    /// Get a new `Movement` with no direction
    pub fn new() -> Movement {
        Movement {
                distance: Distance::N,
                direction: vec![],
                then: None,
                repeat: None,
                initial: false,
                capture: false,
                nocapture: false,
                leaper: false,
                locust: false,
        }
    }
}


fn if_white(color: PieceColor, a: Vec<(i32, i32)>, b: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    if color == PieceColor::White {a} else {b}
}

impl Direction {
    /// Get the corrosponding `Direction` to a parlett direction
    pub fn from_str(dir: &str) -> Direction {
        match dir {
            "*" => Direction::All,
            "+" => Direction::Orthogonal,
            ">" => Direction::OrthogonalForward,
            "<" => Direction::OrthogonalBackward,
            "=" => Direction::OrthogonalSideways,
            "X" => Direction::Diagonal,
            "X>" => Direction::DiagonalForward,
            "X<" => Direction::DiagonalBackward,
            _ => unreachable!(),
        }
    }

    /// Get coordinate direction from direction
    pub fn get_coords(&self, color: &PieceColor, distance: &Distance) -> Vec<(i32,i32)> {
        match self {
            Direction::All => vec![(1,0), (-1, 0), (0,1), (0,-1), (1,1), (-1,1), (-1,-1), (1,-1)],
            Direction::Diagonal => vec![(1,1), (-1,1), (-1,-1), (1,-1)],
            Direction::DiagonalForward => if_white(*color, vec![(1,-1), (-1,-1)], vec![(1,1), (-1,1)]),
            Direction::DiagonalBackward => if_white(*color, vec![(1,1), (-1,1)], vec![(1,-1), (-1,-1)]),
            Direction::Orthogonal => vec![(1,0), (-1, 0), (0,1), (0,-1)],
            Direction::OrthogonalSideways => vec![(1,0), (0,1)],
            Direction::OrthogonalForward => if_white(*color, vec![(0,-1)], vec![(0,1)]),
            Direction::OrthogonalBackward => if_white(*color, vec![(0,1)], vec![(0,-1)]),
            Direction::Hippogonal => {
                match distance {
                    Distance::Hippogonal{m,n} =>
                        vec![(*m as i32, *n as i32),(*m as i32, -(*n as i32)),
                        (-(*m as i32), *n as i32),(-(*m as i32), -(*n as i32)),
                        (*n as i32, *m as i32),(*n as i32, -(*m as i32)),
                        (-(*n as i32), *m as i32),(-(*n as i32), -(*m as i32))
                        ],
                        _ => unreachable!(),
                }
            }
            _ => unimplemented!()
        }
    }
}
