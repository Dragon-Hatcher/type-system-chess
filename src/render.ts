type RenderColoredPiece<P extends ColoredPiece | null> = P extends ColoredPiece
  ? {
      White: {
        Pawn: "P";
        Bishop: "B";
        Knight: "N";
        Rook: "R";
        Queen: "Q";
        King: "K";
      };
      Black: {
        Pawn: "p";
        Bishop: "b";
        Knight: "n";
        Rook: "r";
        Queen: "q";
        King: "k";
      };
    }[P["color"]][P["piece"]]
  : ".";
type RenderState<S extends State> = {
  0: "  A B C D E F G H  ";
  1: Concat3<"1", RenderRank<S["pieces"]["1"]>, " 1">;
  2: Concat3<"2", RenderRank<S["pieces"]["2"]>, " 2">;
  3: Concat3<"3", RenderRank<S["pieces"]["3"]>, " 3">;
  4: Concat3<"4", RenderRank<S["pieces"]["4"]>, " 4">;
  5: Concat3<"5", RenderRank<S["pieces"]["5"]>, " 5">;
  6: Concat3<"6", RenderRank<S["pieces"]["6"]>, " 6">;
  7: Concat3<"7", RenderRank<S["pieces"]["7"]>, " 7">;
  8: Concat3<"8", RenderRank<S["pieces"]["8"]>, " 8">;
  9: "  A B C D E F G H  ";
  toMove: S["toMove"];
};

type RenderRank<P extends PiecePositions[Rank]> = _RenderRank<P, OrderedFiles>;
type _RenderRank<
  P extends PiecePositions[Rank],
  F extends CFile[],
  A extends string = ""
> = F extends []
  ? A
  : _RenderRank<P, DropFirst<F>, `${A} ${RenderColoredPiece<P[F[0]]>}`>;

type RenderSquareSet<S extends Square> = {
  0: "  A B C D E F G H  ";
  1: Concat3<"1", RenderRankSquares<S, "1">, " 1">;
  2: Concat3<"2", RenderRankSquares<S, "2">, " 2">;
  3: Concat3<"3", RenderRankSquares<S, "3">, " 3">;
  4: Concat3<"4", RenderRankSquares<S, "4">, " 4">;
  5: Concat3<"5", RenderRankSquares<S, "5">, " 5">;
  6: Concat3<"6", RenderRankSquares<S, "6">, " 6">;
  7: Concat3<"7", RenderRankSquares<S, "7">, " 7">;
  8: Concat3<"8", RenderRankSquares<S, "8">, " 8">;
  9: "  A B C D E F G H  ";
};
type RenderRankSquares<S extends Square, R extends Rank> = _RenderRankSquares<
  S,
  R,
  OrderedFiles
>;
type _RenderRankSquares<
  S extends Square,
  R extends Rank,
  F extends CFile[],
  A extends string = ""
> = F extends []
  ? A
  : _RenderRankSquares<
      S,
      R,
      DropFirst<F>,
      `${A} ${{ rank: R; file: F[0] } extends S ? "#" : "."}`
    >;
