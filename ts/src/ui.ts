type PlayChess<S extends string, From extends State = StartState> = _PlayChess<
  From,
  GetNonemptyLines<Split<S>>
>;

type _PlayChess<St extends State, Ms extends string[]> = Ms extends [
  infer _M extends string,
  ...infer Rest extends string[]
]
  ? CurOutcome<St> extends "Ongoing."
    ? ParseMove<St, _M> extends infer M extends Move
      ? ApplyMove<St, M> extends infer NSt extends State
        ? _PlayChess<NSt, Rest>
        : never
      : `Invalid move: ${_M}`
    : RenderState<St>
  : RenderState<St>;

type X = ParseMove<StartState, "skcjn">;

type ParseMove<St extends State, M extends string> = NeverToNull<
  Moves<St> extends infer PM extends Move
    ? PM extends any // Split
      ? NameMoveSimple<St["pieces"], PM> extends FullUppercase<M>
        ? PM
        : never
      : never
    : never
>;

type NameMoveSimple<Ps extends PiecePositions, M extends Move> = M extends {
  start: Square;
  end: Square;
  piece: ColoredPiece;
  ep: Square | null;
}
  ? `${M["start"]["file"]}${M["start"]["rank"]}${M["end"]["file"]}${M["end"]["rank"]}${IdxP<
      Ps,
      M["start"]
    > extends M["piece"]
      ? ""
      : PromotionText<M["piece"]["piece"]>}`
  : M extends {
      kingStart: Square;
      rookStart: Square;
      kingEnd: Square;
      rookEnd: Square;
    }
  ? `${M["kingStart"]["file"]}${M["kingStart"]["rank"]}${M["kingEnd"]["file"]}${M["kingEnd"]["rank"]}`
  : never;

type PromotionText<P extends Piece> = {
  Pawn: "";
  Bishop: "B";
  Knight: "N";
  Rook: "R";
  Queen: "Q";
  King: "";
}[P];
