type Rank = "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8";
type CFile = "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H";
type Square = {
  rank: Rank;
  file: CFile;
};
type SS<S extends string> =
  S extends `${infer F extends CFile}${infer R extends Rank}`
    ? { rank: R; file: F }
    : never;

type OrderedFiles = ["A", "B", "C", "D", "E", "F", "G", "H"];
type OrderedRanks = ["1", "2", "3", "4", "5", "6", "7", "8"];

type Offset = -2 | -1 | 0 | 1 | 2;
type OffsetRank<R extends Rank, O extends Offset> = O extends -2 | -1 | 1 | 2
  ? {
      "1": { [-2]: never; [-1]: never; 1: "2"; 2: "3" };
      "2": { [-2]: never; [-1]: "1"; 1: "3"; 2: "4" };
      "3": { [-2]: "1"; [-1]: "2"; 1: "4"; 2: "5" };
      "4": { [-2]: "2"; [-1]: "3"; 1: "5"; 2: "6" };
      "5": { [-2]: "3"; [-1]: "4"; 1: "6"; 2: "7" };
      "6": { [-2]: "4"; [-1]: "5"; 1: "7"; 2: "8" };
      "7": { [-2]: "5"; [-1]: "6"; 1: "8"; 2: never };
      "8": { [-2]: "6"; [-1]: "7"; 1: never; 2: never };
    }[R][O]
  : R;
type OffsetFile<F extends CFile, O extends Offset> = O extends -2 | -1 | 1 | 2
  ? {
      A: { [-2]: never; [-1]: never; 1: "B"; 2: "C" };
      B: { [-2]: never; [-1]: "A"; 1: "C"; 2: "D" };
      C: { [-2]: "A"; [-1]: "B"; 1: "D"; 2: "E" };
      D: { [-2]: "B"; [-1]: "C"; 1: "E"; 2: "F" };
      E: { [-2]: "C"; [-1]: "D"; 1: "F"; 2: "G" };
      F: { [-2]: "D"; [-1]: "E"; 1: "G"; 2: "H" };
      G: { [-2]: "E"; [-1]: "F"; 1: "H"; 2: never };
      H: { [-2]: "F"; [-1]: "G"; 1: never; 2: never };
    }[F][O]
  : F;
type SquareOffset = {
  rank: Offset;
  file: Offset;
};
type OffsetSquare<S extends Square, O extends SquareOffset> = OffsetRank<
  S["rank"],
  O["rank"]
> extends never
  ? never
  : OffsetFile<S["file"], O["file"]> extends never
  ? never
  : {
      rank: OffsetRank<S["rank"], O["rank"]>;
      file: OffsetFile<S["file"], O["file"]>;
    };

type Color = "White" | "Black";
type Other<C extends Color> = C extends "White" ? "Black" : "White";
type Piece = "Pawn" | "Bishop" | "Knight" | "Rook" | "Queen" | "King";
type ColoredPiece = {
  color: Color;
  piece: Piece;
};

type PiecePositions = {
  [r in Rank]: { [f in CFile]: ColoredPiece | null };
};
type IdxP<P extends PiecePositions, S extends Square> = P[S["rank"]][S["file"]];
type UIdxP<P extends PiecePositions, S extends Square> = Unwrap<IdxP<P, S>>;
type State = {
  toMove: Color;
  pieces: PiecePositions;
  ep: Square | null;
  castle: {
    [c in Color]: {
      kingside: boolean;
      queenside: boolean;
    };
  };
};

type EmptyState = {
  toMove: "White";
  pieces: { [r in Rank]: { [f in CFile]: null } };
};
type StartState =
  ParseFen<"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1">;
