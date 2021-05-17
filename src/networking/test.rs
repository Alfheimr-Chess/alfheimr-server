use super::socketdata;
use crate::logic::board::GameBoard;

#[test]
fn generate_boarddata() {
    let board = GameBoard::from_ffen("pppp/PZPP/4/ppkp").unwrap();
    let result = socketdata::generate_boarddata(&board);
    assert_eq!(result.len(), 4);
    assert_eq!(result[1].len(), 4);
    assert_eq!(result[1][1].as_ref().unwrap().0, "z");
    assert_eq!(result[2][1], None);
}

