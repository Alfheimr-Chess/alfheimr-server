let rules = rules();
rules.add_piece("k", "King", "1>,1<");
rules.add_piece("n", "Knight", "~2>,~2<");
rules.add_piece("r", "Rook", "n<,n>");

rules.set_kingstatus("k", true);

rules.create_board("k/n/r/1/1/R/N/K");
return rules;
