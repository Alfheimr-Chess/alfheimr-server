import "modules/default" as df;

let rules = rules();
rules.set_name("Test");

df::add_default_pieces(rules);
df::set_default_board(rules);

return rules;
