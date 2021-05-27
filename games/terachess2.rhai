
let rules = rules();

rules.set_name("TerraChess 2");

rules.add_piece("p", "Pawn", "o1-2>,c1X>");
rules.add_piece("k", "King", "1*,oi2*");
rules.add_piece("q", "Queen", "n*");
rules.add_piece("b", "Bishop", "nX");
rules.add_piece("n", "Knight", "~1/2");
rules.add_piece("r", "Rook", "n+");
rules.add_piece("a", "Amazon", "n*,~1/2");
rules.add_piece("m", "Marshal", "n+,~1/2");
rules.add_piece("c", "Cardinal", "nX,~1/2");
rules.add_piece("ce", "Centaur", "~1/2,1*");
rules.add_piece("ad", "Admiral", "n+,1*");
rules.add_piece("mi", "Missionary", "nX,1*");
rules.add_piece("ea", "Eagle", "1X.n+");
rules.add_piece("rh", "Rhinocerus", "1+.nX");
rules.add_piece("li", "Lion", "1*,~2+,~2/1,~2X");
rules.add_piece("ca", "Camel", "~1/3");
rules.add_piece("gi", "Giraffe", "~2/3");
rules.add_piece("bu", "Buffalo", "~1/2,~1/3,~2/3");
rules.add_piece("cn", "Cannon", "^n+");
rules.add_piece("cr", "Crocodile", "^nX");
rules.add_piece("ma", "Machine", "1+,~2+");
rules.add_piece("el", "Elephant", "1X,~2X");
rules.add_piece("pr", "Prince", "1*,o2>");
rules.add_piece("tr", "Troll", "~3*,c1X>,o1>");
rules.add_piece("so", "Sorceress", "^n*");
rules.add_piece("du", "Duchess", "1*,~2-3*");

rules.set_kingstatus("k", true);

rules.create_board(`(ad)(ce)(mi)mc(bu)(du)(so)(so)(du)(bu)cm(mi)(ce)(ad)/
                    (cn)(ca)(gi)(tr)(rh)(cr)(li)ka(li)(cr)(rh)(tr)(gi)(ca)(cn)/
                    (el)rnb(ma)(pr)(ea)qq(ea)(pr)(ma)bnr(el)/
                    pppppppppppppppp/
                    16/16/16/16/
                    16/16/16/16/
                    PPPPPPPPPPPPPPPP/
                    (EL)RNB(MA)(PR)(EA)QQ(EA)(PR)(MA)BNR(EL)/
                    (CN)(CA)(GI)(TR)(RH)(CR)(LI)KA(LI)(CR)(RH)(TR)(GI)(CA)(CN)/
                    (AD)(CE)(MI)MC(BU)(DU)(SO)(SO)(DU)(BU)CM(MI)(CE)(AD)`);
return rules;
