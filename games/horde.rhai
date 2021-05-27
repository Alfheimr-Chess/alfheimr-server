import "modules/default" as df;

let rules = rules();

rules.set_name("Horde");
df::add_default_pieces(rules);

rules.create_board("rnbqkbnr/pppppppp/8/1PP2PP1/PPPPPPPP/PPPPPPPP/PPPPPPPP/PPPPPPPP");

return rules;
