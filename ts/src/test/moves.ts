type T2 = [
  // Double forward
  AssertMoveDoes<
    ParseFen<"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1">,
    "e2e4",
    ParseFen<"rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1">
  >,
  // Castle
  AssertMoveDoes<
    ParseFen<"rnbqkb1r/ppp2ppp/3p1n2/4p3/4P3/3B1N2/PPPP1PPP/RNBQK2R w KQkq - 0 1">,
    "e1g1",
    ParseFen<"rnbqkb1r/ppp2ppp/3p1n2/4p3/4P3/3B1N2/PPPP1PPP/RNBQ1RK1 b kq - 0 1">
  >,
  AssertMoveDoes<
    ParseFen<"r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1">,
    "e1e2",
    ParseFen<"r3k2r/8/8/8/8/8/4K3/R6R b kq - 0 1">
  >,
  AssertMoveDoes<
    ParseFen<"r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1">,
    "a1a8",
    ParseFen<"R3k2r/8/8/8/8/8/8/4K2R b Kk - 0 1">
  >,
  AssertMoveDoes<
    ParseFen<"r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1">,
    "h1h7",
    ParseFen<"r3k2r/7R/8/8/8/8/8/R3K3 b Qkq - 0 1">
  >,
  // EP
  AssertMoveDoes<
    ParseFen<"rnbqkbnr/p1p1pppp/8/1pPp4/8/PP1PPPPP/PP1PPPPP/RNBQKBNR w KQkq d6 0 1">,
    `c5d6`,
    ParseFen<"rnbqkbnr/p1p1pppp/3P4/1p6/8/PP1PPPPP/PP1PPPPP/RNBQKBNR b KQkq - 0 1">
  >
];
