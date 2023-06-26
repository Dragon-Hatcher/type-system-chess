type ParseFen<Fen extends string> =
  Fen extends `${infer FenPieces} ${infer ToMove extends
    | "w"
    | "b"} ${infer _Castling} ${infer _EP} ${infer _HalfMove} ${infer _FullMove}`
    ? ParseFenPieces<FenPieces> extends PiecePositions
      ? {
          toMove: ToMove extends "w" ? "White" : "Black";
          pieces: ParseFenPieces<FenPieces>;
        }
      : null
    : null;

type ParseFenPieces<FPieces extends string> =
  FPieces extends `${infer R8}/${infer R7}/${infer R6}/${infer R5}/${infer R4}/${infer R3}/${infer R2}/${infer R1}`
    ? ParseFenRank<R1> extends PiecePositions[Rank]
      ? ParseFenRank<R2> extends PiecePositions[Rank]
        ? ParseFenRank<R3> extends PiecePositions[Rank]
          ? ParseFenRank<R4> extends PiecePositions[Rank]
            ? ParseFenRank<R5> extends PiecePositions[Rank]
              ? ParseFenRank<R6> extends PiecePositions[Rank]
                ? ParseFenRank<R7> extends PiecePositions[Rank]
                  ? ParseFenRank<R8> extends PiecePositions[Rank]
                    ? {
                        "1": ParseFenRank<R1>;
                        "2": ParseFenRank<R2>;
                        "3": ParseFenRank<R3>;
                        "4": ParseFenRank<R4>;
                        "5": ParseFenRank<R5>;
                        "6": ParseFenRank<R6>;
                        "7": ParseFenRank<R7>;
                        "8": ParseFenRank<R8>;
                      }
                    : null
                  : null
                : null
              : null
            : null
          : null
        : null
      : null
    : null;

type ParseFenRank<
  FRank extends string,
  F extends CFile = "A",
  A extends object = {}
> = FRank extends ""
  ? [F] extends [never]
    ? A
    : null
  : ParseFenPiece<FirstChar<FRank>> extends infer P extends ColoredPiece | null
  ? ParseFenRank<DropFirstChar<FRank>, OffsetFile<F, 1>, A & { [f in F]: P }>
  : FirstChar<FRank> extends infer R extends Rank
  ? ParseFenRank<
      // The case of 1 is handled by ParseFenPiece
      `${OffsetRank<R, -1>}${DropFirstChar<FRank>}`,
      OffsetFile<F, 1>,
      A & { [f in F]: null }
    >
  : null;

type FenPieceTable = {
  P: { color: "White"; piece: "Pawn" };
  B: { color: "White"; piece: "Bishop" };
  N: { color: "White"; piece: "Knight" };
  R: { color: "White"; piece: "Rook" };
  Q: { color: "White"; piece: "Queen" };
  K: { color: "White"; piece: "King" };
  p: { color: "Black"; piece: "Pawn" };
  b: { color: "Black"; piece: "Bishop" };
  n: { color: "Black"; piece: "Knight" };
  r: { color: "Black"; piece: "Rook" };
  q: { color: "Black"; piece: "Queen" };
  k: { color: "Black"; piece: "King" };
  "1": null;
};
type ParseFenPiece<Char extends string> = Char extends keyof FenPieceTable
  ? FenPieceTable[Char]
  : undefined;
