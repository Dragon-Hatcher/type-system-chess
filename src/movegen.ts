type ExcludeOccupied<
  S extends Square,
  OurC extends Color,
  P extends PiecePositions
> = S extends Square ? (IdxP<P, S> extends { color: OurC } ? never : S) : never;

type KingMoveSqs<S extends Square, P extends PiecePositions> = IdxP<
  P,
  S
> extends { color: infer OurC extends Color }
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

type KnightMoveSqs<S extends Square, P extends PiecePositions> = IdxP<
  P,
  S
> extends {
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
> = IdxP<P, S> extends { color: infer OurC extends Color }
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
  IdxP<P, S> extends { color: infer TargetC extends Color }
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

type Forward<
  C extends Color,
  FOff extends Offset = 0,
  Double extends boolean = false
> = C extends "White"
  ? { rank: Double extends false ? 1 : 2; file: FOff }
  : { rank: Double extends false ? -1 : -2; file: FOff };
type StartRank<C extends Color> = C extends "White" ? "2" : "7";
type PawnMoveSqs<S extends Square, P extends PiecePositions> = IdxP<
  P,
  S
> extends { color: infer OurC extends Color }
  ? // Forward 1.
    | (IdxP<P, OffsetSquare<S, Forward<OurC>>> extends null
          ? OffsetSquare<S, Forward<OurC>>
          : never)
      // Forward 2
      | (S["rank"] extends StartRank<OurC>
          ? OffsetSquare<S, Forward<OurC, 0, true>>
          : never)
      // Captures
      | (IdxP<P, OffsetSquare<S, Forward<OurC, -1>>> extends {
          color: Other<OurC>;
        }
          ? OffsetSquare<S, Forward<OurC, -1>>
          : never)
      | (IdxP<P, OffsetSquare<S, Forward<OurC, 1>>> extends {
          color: Other<OurC>;
        }
          ? OffsetSquare<S, Forward<OurC, 1>>
          : never)
  : // TODO: EP + Promotions
    never;
type PawnAttackSqs<S extends Square, P extends PiecePositions> = IdxP<
  P,
  S
> extends { color: infer OurC extends Color }
  ? OffsetSquare<S, Forward<OurC, -1>> | OffsetSquare<S, Forward<OurC, 1>>
  : never;

type Move = {
  start: Square;
  end: Square;
};

type MoveSqsToMoves<
  Start extends Square,
  Ends extends Square
> = Ends extends Square ? { start: Start; end: Square } : never;

type MovesForSq<S extends Square, Ps extends PiecePositions> = IdxP<
  Ps,
  S
> extends {
  piece: infer P extends Piece;
}
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
    : P extends "Pawn"
    ? MoveSqsToMoves<S, PawnMoveSqs<S, Ps>>
    : // TODO: Castle + Pawn moves
      never
  : never;

type AttacksForSq<S extends Square, Ps extends PiecePositions> = IdxP<
  Ps,
  S
> extends {
  piece: infer P extends Piece;
}
  ? P extends "Bishop"
    ? BishopMoveSqs<S, Ps>
    : P extends "Knight"
    ? KnightMoveSqs<S, Ps>
    : P extends "Rook"
    ? RookMoveSqs<S, Ps>
    : P extends "Queen"
    ? QueenMoveSqs<S, Ps>
    : P extends "King"
    ? KingMoveSqs<S, Ps>
    : P extends "Pawn"
    ? PawnAttackSqs<S, Ps>
    : never
  : never;

type SqsAttackedByC<
  Ps extends PiecePositions,
  C extends Color
> = _SqsAttackedByC<Ps, C, Rank, CFile>;
type _SqsAttackedByC<
  Ps extends PiecePositions,
  C extends Color,
  R extends Rank,
  F extends CFile
> = R extends any
  ? F extends any
    ? IdxP<Ps, { rank: R; file: F }> extends { color: C }
      ? AttacksForSq<{ rank: R; file: F }, Ps>
      : never
    : never
  : never;

type ColorChecked<
  Ps extends PiecePositions,
  C extends Color
> = true extends SquareIsCheck<Ps, C, Rank, CFile, SqsAttackedByC<Ps, Other<C>>>
  ? true
  : false;
type SquareIsCheck<
  Ps extends PiecePositions,
  C extends Color,
  R extends Rank,
  F extends CFile,
  Attacked extends Square
> = R extends any
  ? F extends any
    ? { rank: R; file: F } extends Attacked
      ? IdxP<Ps, { rank: R; file: F }> extends { color: Other<C> }
        ? true
        : never
      : never
    : never
  : never;

type ApplyMove<S extends State, M extends Move> = {
  toMove: Other<S["toMove"]>;
  pieces: ApplyMoveToPieces<S["pieces"], M>;
};

type ApplyMoveToPieces<Ps extends PiecePositions, M extends Move> = {
  [R in Rank]: { [F in CFile]: ApplyMoveToSq<Ps, { rank: R; file: F }, M> };
};

type ApplyMoveToSq<
  Ps extends PiecePositions,
  S extends Square,
  M extends Move
> = S extends M["start"]
  ? null
  : S extends M["end"]
  ? IdxP<Ps, M["start"]>
  : IdxP<Ps, S>;
