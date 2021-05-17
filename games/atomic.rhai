let rules = rules();
rules.add_piece("p", "Pawn", "o1>,oi2>,c1X>");
rules.add_piece("k", "King", "1*");
rules.add_piece("q", "Queen", "n*");
rules.add_piece("b", "Bishop", "nX");
rules.add_piece("n", "Knight", "~1/2");
rules.add_piece("r", "Rook", "n+");
rules.set_kingstatus("k", true);
rules.create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");

for i in ["p", "k", "q", "b", "n", "r"] {
	rules.after_take(i, |m| {
		let board = get_board();
		let p = m.points;
        let color = board.get_piece(p[2], p[3]).color;
        board.remove_piece(p[2], p[3]);
		for i in range(0,3) {
			for j in range(0,3) {
				let x = p[2]-1+i;
				let y = p[3]-1+j;
				if board.contains_piece(x, y) && board.get_piece(x, y).symbol != "p" {
                    if board.get_piece(x, y).symbol == "k" {
                        set_winner(color);
                    }
					board.remove_piece(x, y);
				}
			}
		}
	});
}

return rules;
