type T1 = [
  AssertOutcome<StartState, "Ongoing.">,
  AssertOutcome<
    ParseFen<"rnbqkbnr/ppppp2p/5p2/6pQ/8/4P3/PPPP1PPP/RNB1KBNR b KQkq - 0 1">,
    "Checkmate - White Wins!"
  >,
  AssertOutcome<ParseFen<"k7/8/1Q6/8/8/8/P7/8 b - - 0 1">, "Draw by stalemate.">
];
