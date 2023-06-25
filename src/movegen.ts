type ExcludeOccupied<
  S extends Square,
  OurC extends Color,
  P extends PiecePositions
> = S extends Square
  ? P[S["rank"]][S["file"]] extends { color: OurC }
    ? never
    : S
  : never;

type KingMoveSqs<
  S extends Square,
  P extends PiecePositions
> = P[S["rank"]][S["file"]] extends { color: infer OurC extends Color }
  ? ExcludeOccupied<
      | OffsetSquare<S, { rank: -1; file: -1 }>
      | OffsetSquare<S, { rank: -1; file: 0 }>
      | OffsetSquare<S, { rank: -1; file: 1 }>
      | OffsetSquare<S, { rank: 0; file: -1 }>
      | OffsetSquare<S, { rank: 0; file: 1 }>
      | OffsetSquare<S, { rank: 1; file: -1 }>
      | OffsetSquare<S, { rank: 1; file: 0 }>
      | OffsetSquare<S, { rank: 1; file: 1 }>,
      OurC,
      P
    >
  : never;

type KnightMoveSqs<
  S extends Square,
  P extends PiecePositions
> = P[S["rank"]][S["file"]] extends {
  color: infer OurC extends Color;
}
  ? ExcludeOccupied<
      | OffsetSquare<S, { rank: -2; file: -1 }>
      | OffsetSquare<S, { rank: -2; file: 1 }>
      | OffsetSquare<S, { rank: 2; file: -1 }>
      | OffsetSquare<S, { rank: 2; file: 1 }>
      | OffsetSquare<S, { rank: -1; file: -2 }>
      | OffsetSquare<S, { rank: 1; file: -2 }>
      | OffsetSquare<S, { rank: -1; file: 2 }>
      | OffsetSquare<S, { rank: 1; file: 2 }>,
      OurC,
      P
    >
  : never;

type Cast<
  S extends Square,
  O extends SquareOffset,
  P extends PiecePositions
> = P[S["rank"]][S["file"]] extends { color: infer OurC extends Color }
  ? _Cast<OffsetSquare<S, O>, O, P, OurC>
  : never;
type _Cast<
  S extends Square,
  O extends SquareOffset,
  P extends PiecePositions,
  OurC extends Color,
  A extends Square = never
> = [S] extends [never]
  ? // We hit the edge of the board.
    A
  : // the square exists
  P[S["rank"]][S["file"]] extends { color: infer TargetC extends Color }
  ? // Check if this is our piece or opps.
    TargetC extends OurC
    ? A
    : A | S
  : // Continue casting the ray as there was no piece
    _Cast<OffsetSquare<S, O>, O, P, OurC, A | S>;

type RookMoveSqs<S extends Square, P extends PiecePositions> =
  | Cast<S, { rank: 1; file: 0 }, P>
  | Cast<S, { rank: -1; file: 0 }, P>
  | Cast<S, { rank: 0; file: 1 }, P>
  | Cast<S, { rank: 0; file: -1 }, P>;

type BishopMoveSqs<S extends Square, P extends PiecePositions> =
  | Cast<S, { rank: 1; file: 1 }, P>
  | Cast<S, { rank: 1; file: -1 }, P>
  | Cast<S, { rank: -1; file: 1 }, P>
  | Cast<S, { rank: -1; file: -1 }, P>;

type QueenMoveSqs<S extends Square, P extends PiecePositions> =
  | RookMoveSqs<S, P>
  | BishopMoveSqs<S, P>;

type Move = {
  start: Square;
  end: Square;
};

type MoveSqsToMoves<
  Start extends Square,
  Ends extends Square
> = Ends extends Square ? { start: Start; end: Square } : never;

type Moves<
  S extends Square,
  Ps extends PiecePositions
> = Ps[S["rank"]][S["file"]] extends { piece: infer P extends Piece }
  ? P extends "Bishop"
    ? MoveSqsToMoves<S, BishopMoveSqs<S, Ps>>
    : P extends "Knight"
    ? MoveSqsToMoves<S, KnightMoveSqs<S, Ps>>
    : P extends "Rook"
    ? MoveSqsToMoves<S, RookMoveSqs<S, Ps>>
    : P extends "Queen"
    ? MoveSqsToMoves<S, QueenMoveSqs<S, Ps>>
    : P extends "King"
    ? MoveSqsToMoves<S, KingMoveSqs<S, Ps>>
    // TODO: Castle + Pawn moves
    : never
  : never;
