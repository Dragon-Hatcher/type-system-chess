// NOTE: If performance is too bad rename the files in /test
// from *.ts -> *.txt to prevent them from being checked.

// Render a game state using `RenderState`. You can hover over
// the display to view the type.
type __DISPLAY__1 = RenderState<StartState>;

// Create a `State` from a FEN using the `ParseFen` type.
type __DISPLAY__2 = RenderState<
  ParseFen<"rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2">
>;

// Now you can play a game of chess. Write alternating moves
// using long algebraic notation. (Start square then end square.
// For promotions add b n r or q to the end. For castling
// write the movement of the king. This is equivalent notation
// to the UCI protocol.) The display shows the state of the game
// after all moves have been applied.
type __DISPLAY__3 = PlayChess<// Promotion
`
  f2f4
  a7a6
  f4f5
  a6a5
  f5f6
  a5a4
  f6g7
  a4a3
  g7h8q
`>;
// Fools mate
// `
//   f2f3
//   e7e6
//   g2g4
//   d8h4
// `

// Optionally start from a specific position
// ,ParseFen<"rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2">
