import "modules/default" as df;

let rules = rules();
rules.set_name("Atomic Chess");

df::add_default_pieces(rules);
df::set_default_board(rules);

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
