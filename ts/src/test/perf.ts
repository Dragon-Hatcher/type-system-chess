// These are very slow. You can rename this file to perf.txt to not run them.

type T0 = [
    // Start Position
    AssertCount<Moves<StartState>, 20>,
    // Perf positions from https://www.chessprogramming.org/Perft_Results
    AssertCount<Moves<ParseFen<"r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1">>, 48>,
    AssertCount<Moves<ParseFen<"8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1">>, 14>,
    AssertCount<Moves<ParseFen<"r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1">>, 6>,
    AssertCount<Moves<ParseFen<"rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8">>, 44>,
    AssertCount<Moves<ParseFen<"r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ">>, 46>,
    // EP
    AssertCount<Moves<ParseFen<"rnbqkbnr/p1p1pppp/8/1pPp4/8/PP1PPPPP/PP1PPPPP/RNBQKBNR w KQkq d6 0 1">>, 11>,
    //Castling
    AssertCount<Moves<ParseFen<"r3k2r/p3p2p/8/8/8/1R3R2/PPPPPPPP/RNBQKBNR b KQkq - 0 1">>, 14>,
];

