type ExcludeOccupied<
  S extends Square,
  OurC extends Color,
  P extends PiecePositions
> = S extends Square ? (IdxP<P, S> extends { color: OurC } ? never : S) : never;

type KingPMoveSqs<S extends Square, P extends PiecePositions> = IdxP<
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

type CastlingRank<C extends Color> = C extends "White" ? "1" : "8";
type CRSquare<C extends Color, F extends CFile> = {
  rank: CastlingRank<C>;
  file: F;
};
type CastlingMoves<S extends Square, St extends State> = IdxP<
  St["pieces"],
  S
> extends { color: infer OurC extends Color }
  ? _Kingside<S, St, OurC> | _Queenside<S, St, OurC>
  : never;
type _Kingside<
  S extends Square,
  St extends State,
  OurC extends Color
> = St["castle"][OurC]["kingside"] extends true
  ? IdxP<St["pieces"], CRSquare<OurC, "F"> | CRSquare<OurC, "G">> extends null
    ? true extends SquareIsAttacked<
        CastlingRank<OurC>,
        "E" | "F" | "G",
        SqsAttackedByC<St["pieces"], Other<OurC>>
      >
      ? never
      : {
          kingStart: S;
          rookStart: CRSquare<OurC, "H">;
          kingEnd: CRSquare<OurC, "G">;
          rookEnd: CRSquare<OurC, "F">;
        }
    : never
  : never;
type _Queenside<
  S extends Square,
  St extends State,
  OurC extends Color
> = St["castle"][OurC]["kingside"] extends true
  ? IdxP<
      St["pieces"],
      CRSquare<OurC, "B"> | CRSquare<OurC, "C"> | CRSquare<OurC, "D">
    > extends null
    ? true extends SquareIsAttacked<
        CastlingRank<OurC>,
        "E" | "D" | "C",
        SqsAttackedByC<St["pieces"], Other<OurC>>
      >
      ? never
      : {
          kingStart: S;
          rookStart: CRSquare<OurC, "A">;
          kingEnd: CRSquare<OurC, "C">;
          rookEnd: CRSquare<OurC, "D">;
        }
    : never
  : never;

type KnightPMoveSqs<S extends Square, P extends PiecePositions> = IdxP<
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

type RookPMoveSqs<S extends Square, P extends PiecePositions> =
  | Cast<S, { rank: 1; file: 0 }, P>
  | Cast<S, { rank: -1; file: 0 }, P>
  | Cast<S, { rank: 0; file: 1 }, P>
  | Cast<S, { rank: 0; file: -1 }, P>;

type BishopPMoveSqs<S extends Square, P extends PiecePositions> =
  | Cast<S, { rank: 1; file: 1 }, P>
  | Cast<S, { rank: 1; file: -1 }, P>
  | Cast<S, { rank: -1; file: 1 }, P>
  | Cast<S, { rank: -1; file: -1 }, P>;

type QueenPMoveSqs<S extends Square, P extends PiecePositions> =
  | RookPMoveSqs<S, P>
  | BishopPMoveSqs<S, P>;

type Forward<
  C extends Color,
  FOff extends Offset = 0,
  Double extends boolean = false
> = C extends "White"
  ? { rank: Double extends false ? 1 : 2; file: FOff }
  : { rank: Double extends false ? -1 : -2; file: FOff };
type StartRank<C extends Color> = C extends "White" ? "2" : "7";
type LastRank<C extends Color> = C extends "White" ? "8" : "1";
type MakeMoves<
  S extends Square,
  E extends Square,
  C extends Color,
  P extends Piece
> = P extends any
  ? { start: S; end: E; piece: { color: C; piece: P }; ep: null }
  : never;
type PawnPMoveSqs<S extends Square, P extends PiecePositions> = IdxP<
  P,
  S
> extends { color: infer OurC extends Color }
  ? // Forward 1.
    | (S["rank"] extends LastRank<OurC>
          ? never
          : IdxP<P, OffsetSquare<S, Forward<OurC>>> extends null
          ? OffsetSquare<S, Forward<OurC>>
          : never)
      // Forward 2
      | (S["rank"] extends StartRank<OurC>
          ? IdxP<P, OffsetSquare<S, Forward<OurC>>> extends null
            ? IdxP<P, OffsetSquare<S, Forward<OurC, 0, true>>> extends null
              ? OffsetSquare<S, Forward<OurC, 0, true>>
              : never
            : never
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
  : // TODO: EP
    never;
type PawnAttackSqs<S extends Square, P extends PiecePositions> = IdxP<
  P,
  S
> extends { color: infer OurC extends Color }
  ? OffsetSquare<S, Forward<OurC, -1>> | OffsetSquare<S, Forward<OurC, 1>>
  : never;
type PawnEpMoves<
  S extends Square,
  P extends PiecePositions,
  EP extends Square | null
> = IdxP<P, S> extends { color: infer OurC extends Color }
  ? EP extends Square
    ?
        | (IdxP<P, OffsetSquare<S, Forward<OurC, -1>>> extends null
            ? EP extends OffsetSquare<S, Forward<OurC, -1>>
              ? {
                  start: S;
                  end: OffsetSquare<S, Forward<OurC, -1>>;
                  piece: { piece: "Pawn"; color: OurC };
                  ep: EP;
                }
              : never
            : never)
        | (IdxP<P, OffsetSquare<S, Forward<OurC, 1>>> extends null
            ? EP extends OffsetSquare<S, Forward<OurC, 1>>
              ? {
                  start: S;
                  end: OffsetSquare<S, Forward<OurC, 1>>;
                  piece: { piece: "Pawn"; color: OurC };
                  ep: EP;
                }
              : never
            : never)
    : never
  : never;

type Move =
  | {
      start: Square;
      end: Square;
      piece: ColoredPiece;
      ep: Square | null;
    }
  | {
      kingStart: Square;
      rookStart: Square;
      kingEnd: Square;
      rookEnd: Square;
    };

type MoveSqsToMoves<
  Ps extends PiecePositions,
  Start extends Square,
  Ends extends Square
> = Ends extends Square
  ? Ends["rank"] extends "1" | "8"
    ? IdxP<Ps, Start> extends { piece: "Pawn"; color: infer C extends Color }
      ? MakeMoves<Start, Ends, C, "Bishop" | "Knight" | "Rook" | "Queen">
      : { start: Start; end: Ends; piece: UIdxP<Ps, Start>; ep: null }
    : { start: Start; end: Ends; piece: UIdxP<Ps, Start>; ep: null }
  : never;

type PMovesForSqC<
  S extends Square,
  C extends Color,
  Ps extends PiecePositions,
  St extends State
> = IdxP<Ps, S> extends {
  piece: infer P extends Piece;
  color: C;
}
  ? P extends "Bishop"
    ? MoveSqsToMoves<Ps, S, BishopPMoveSqs<S, Ps>>
    : P extends "Knight"
    ? MoveSqsToMoves<Ps, S, KnightPMoveSqs<S, Ps>>
    : P extends "Rook"
    ? MoveSqsToMoves<Ps, S, RookPMoveSqs<S, Ps>>
    : P extends "Queen"
    ? MoveSqsToMoves<Ps, S, QueenPMoveSqs<S, Ps>>
    : P extends "King"
    ? MoveSqsToMoves<Ps, S, KingPMoveSqs<S, Ps>> | CastlingMoves<S, St>
    : P extends "Pawn"
    ? MoveSqsToMoves<Ps, S, PawnPMoveSqs<S, Ps>> | PawnEpMoves<S, Ps, St["ep"]>
    : // TODO: Castle
      never
  : never;

type PMovesForC<S extends State, C extends Color> = _PMovesForC<
  S,
  C,
  Rank,
  CFile
>;
type _PMovesForC<
  S extends State,
  C extends Color,
  R extends Rank,
  F extends CFile
> = R extends any
  ? F extends any
    ? PMovesForSqC<{ rank: R; file: F }, C, S["pieces"], S>
    : never
  : never;

type Moves<S extends State> = _Moves<S, PMovesForC<S, S["toMove"]>>;
type _Moves<S extends State, PMs extends Move> = PMs extends any
  ? IsCheck<ApplyMove<S, PMs>["pieces"], S["toMove"]> extends false
    ? PMs
    : never
  : never;

type AttacksForSq<S extends Square, Ps extends PiecePositions> = IdxP<
  Ps,
  S
> extends {
  piece: infer P extends Piece;
}
  ? P extends "Bishop"
    ? BishopPMoveSqs<S, Ps>
    : P extends "Knight"
    ? KnightPMoveSqs<S, Ps>
    : P extends "Rook"
    ? RookPMoveSqs<S, Ps>
    : P extends "Queen"
    ? QueenPMoveSqs<S, Ps>
    : P extends "King"
    ? KingPMoveSqs<S, Ps>
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
      ? IdxP<Ps, { rank: R; file: F }> extends { piece: "King"; color: C }
        ? true
        : never
      : never
    : never
  : never;
type SquareIsAttacked<
  R extends Rank,
  F extends CFile,
  Attacked extends Square
> = R extends any
  ? F extends any
    ? { rank: R; file: F } extends Attacked
      ? true
      : never
    : never
  : never;

type IsCheck<Ps extends PiecePositions, C extends Color> = ColorChecked<Ps, C>;

type ApplyMove<S extends State, M extends Move> = {
  toMove: Other<S["toMove"]>;
  pieces: ApplyMoveToPieces<S["pieces"], M>;
  ep: ApplyMoveToEp<M>;
  castle: ApplyMoveToCastle<S["castle"], M>;
};

type ApplyMoveToPieces<Ps extends PiecePositions, M extends Move> = {
  [R in Rank]: { [F in CFile]: ApplyMoveToSq<Ps, { rank: R; file: F }, M> };
};

type ApplyMoveToSq<
  Ps extends PiecePositions,
  S extends Square,
  M extends Move
> = M extends {
  start: Square;
  end: Square;
  piece: ColoredPiece;
  ep: Square | null;
}
  ? S extends M["start"]
    ? null
    : S extends M["end"]
    ? M["piece"]
    : IdxP<Ps, S>
  : M extends {
      kingStart: Square;
      rookStart: Square;
      kingEnd: Square;
      rookEnd: Square;
    }
  ? S extends M["rookStart"] | M["kingStart"]
    ? null
    : S extends M["kingEnd"]
    ? IdxP<Ps, M["kingStart"]>
    : S extends M["rookEnd"]
    ? IdxP<Ps, M["rookStart"]>
    : IdxP<Ps, S>
  : never;

type ApplyMoveToEp<M extends Move> = M extends {
  start: infer Start extends { rank: "2" | "7"; file: CFile };
  end: { rank: "4" | "5" };
  piece: {
    piece: "Pawn";
    color: infer C extends Color;
  };
}
  ? OffsetSquare<Start, Forward<C>>
  : null;

type ApplyMoveToCastle<S extends State["castle"], M extends Move> = {
  White: {
    kingside: _ApplyMoveToCastle<M, S["White"]["kingside"], SS<"H1">>;
    queenside: _ApplyMoveToCastle<M, S["White"]["queenside"], SS<"A1">>;
  };
  Black: {
    kingside: _ApplyMoveToCastle<M, S["Black"]["kingside"], SS<"H8">>;
    queenside: _ApplyMoveToCastle<M, S["Black"]["queenside"], SS<"A8">>;
  };
};
type _ApplyMoveToCastle<
  M extends Move,
  Initial,
  RookSquare extends Square
> = M extends { piece: { piece: "King" } } | { start: RookSquare }
  ? false
  : Initial;
