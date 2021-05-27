import "modules/default" as df;

let rules = rules();

rules.set_name("All Pawns");
rules.add_piece("p", "Pawn", "o1>,oi2>,c1X>");
rules.add_piece("k", "Pawn", "1*");
rules.add_piece("q", "Pawn", "n*");
rules.add_piece("b", "Pawn", "nX");
rules.add_piece("n", "Pawn", "~1/2");
rules.add_piece("r", "Pawn", "n+");
rules.set_kingstatus("k", true);
df::set_default_board(rules);

return rules;
