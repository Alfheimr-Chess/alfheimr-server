
fn add_default_pieces(rules) {
	rules.add_piece("p", "Pawn", "o1>,oi2>,c1X>");
	rules.add_piece("k", "King", "1*");
	rules.add_piece("q", "Queen", "n*");
	rules.add_piece("b", "Bishop", "nX");
	rules.add_piece("n", "Knight", "~1/2");
	rules.add_piece("r", "Rook", "n+");
	rules.set_kingstatus("k", true);
}

fn set_default_board(rules) {
	rules.create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
}

