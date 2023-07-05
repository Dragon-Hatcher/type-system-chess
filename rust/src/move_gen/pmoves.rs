use crate::board_rep::{
    board::{
        idx::{IdxBoard, RunIdxBoard},
        BoardTy, CellEn, Empty, Filled,
    },
    color::{Black, ColorEn, White},
    piece::{Bishop, ColoredPiece, King, Knight, Pawn, PieceEn, Queen, Rook},
    square::{AllSqs, SquareTy},
};

use super::{
    bishop::{BishopMoveSqs, RunBishopMoveSqs},
    king::{KingMoveSqs, RunKingMoveSqs},
    knight::{KnightMoveSqs, RunKnightMoveSqs},
    list::{MLNil, MoveListTy, SLCons, SLNil, SquareListTy, ConcatML, RunConcatML},
    ml_from_sl::{PMoveLFromSqs, RunPMoveLFromSqs},
    pawn::{PawnMoves, RunPawnMoves},
    queen::{QueenMoveSqs, RunQueenMoveSqs},
    rook::{RookMoveSqs, RunRookMoveSqs},
};

pub(crate) trait RunPMovesForSq<B: BoardTy, MoverC: ColorEn>: SquareTy {
    type Output: MoveListTy;
}
pub(crate) type PMovesForSq<S, B, MoverC> = <S as RunPMovesForSq<B, MoverC>>::Output;

impl<S: SquareTy, B: BoardTy, MoverC: ColorEn> RunPMovesForSq<B, MoverC> for S
where
    B: RunIdxBoard<S>,
    S: RunPMovesForTypeAtSq<B, MoverC, IdxBoard<B, S>>,
{
    type Output = <S as RunPMovesForTypeAtSq<B, MoverC, IdxBoard<B, S>>>::Output;
}

pub(crate) trait RunPMovesForTypeAtSq<B: BoardTy, MoverC: ColorEn, Type: CellEn>:
    SquareTy
{
    type Output: MoveListTy;
}

impl<S: SquareTy, B: BoardTy, MoverC: ColorEn> RunPMovesForTypeAtSq<B, MoverC, Empty> for S {
    type Output = MLNil;
}
impl<S: SquareTy, B: BoardTy, P: PieceEn>
    RunPMovesForTypeAtSq<B, White, Filled<ColoredPiece<P, Black>>> for S
{
    type Output = MLNil;
}
impl<S: SquareTy, B: BoardTy, P: PieceEn>
    RunPMovesForTypeAtSq<B, Black, Filled<ColoredPiece<P, White>>> for S
{
    type Output = MLNil;
}
impl<S: SquareTy, B: BoardTy, C: ColorEn>
    RunPMovesForTypeAtSq<B, C, Filled<ColoredPiece<Knight, C>>> for S
where
    S: RunKnightMoveSqs,
    KnightMoveSqs<S>: RunPMoveLFromSqs<B, S, C, Knight>,
{
    type Output = PMoveLFromSqs<KnightMoveSqs<S>, B, S, C, Knight>;
}
impl<S: SquareTy, B: BoardTy, C: ColorEn> RunPMovesForTypeAtSq<B, C, Filled<ColoredPiece<King, C>>>
    for S
where
    S: RunKingMoveSqs,
    KingMoveSqs<S>: RunPMoveLFromSqs<B, S, C, King>,
{
    type Output = PMoveLFromSqs<KingMoveSqs<S>, B, S, C, King>;
}
impl<S: SquareTy, B: BoardTy, C: ColorEn> RunPMovesForTypeAtSq<B, C, Filled<ColoredPiece<Rook, C>>>
    for S
where
    S: RunRookMoveSqs<B>,
    RookMoveSqs<S, B>: RunPMoveLFromSqs<B, S, C, Rook>,
{
    type Output = PMoveLFromSqs<RookMoveSqs<S, B>, B, S, C, Rook>;
}
impl<S: SquareTy, B: BoardTy, C: ColorEn>
    RunPMovesForTypeAtSq<B, C, Filled<ColoredPiece<Bishop, C>>> for S
where
    S: RunBishopMoveSqs<B>,
    BishopMoveSqs<S, B>: RunPMoveLFromSqs<B, S, C, Bishop>,
{
    type Output = PMoveLFromSqs<BishopMoveSqs<S, B>, B, S, C, Bishop>;
}
impl<S: SquareTy, B: BoardTy, C: ColorEn> RunPMovesForTypeAtSq<B, C, Filled<ColoredPiece<Queen, C>>>
    for S
where
    S: RunQueenMoveSqs<B>,
    QueenMoveSqs<S, B>: RunPMoveLFromSqs<B, S, C, Queen>,
{
    type Output = PMoveLFromSqs<QueenMoveSqs<S, B>, B, S, C, Queen>;
}
impl<S: SquareTy, B: BoardTy, C: ColorEn> RunPMovesForTypeAtSq<B, C, Filled<ColoredPiece<Pawn, C>>>
    for S
where
    S: RunPawnMoves<B>,
{
    type Output = PawnMoves<S, B>;
}

pub(crate) trait RunPMoves<MoverC: ColorEn>: BoardTy {
    type Output: MoveListTy;
}
pub(crate) type PMoves<B, MoverC> = <B as RunPMoves<MoverC>>::Output;

impl<B: BoardTy, MoverC: ColorEn> RunPMoves<MoverC> for B
where
    B: RunPMovesForSqL<AllSqs, MoverC>,
{
    type Output = <B as RunPMovesForSqL<AllSqs, MoverC>>::Output;
}

pub(crate) trait RunPMovesForSqL<L: SquareListTy, MoverC: ColorEn>: BoardTy {
    type Output: MoveListTy;
}
pub(crate) type PMovesForSqL<B, L, MoverC> = <B as RunPMovesForSqL<L, MoverC>>::Output;

impl<B: BoardTy, MoverC: ColorEn> RunPMovesForSqL<SLNil, MoverC> for B {
    type Output = MLNil;
}
impl<B: BoardTy, MoverC: ColorEn, S: SquareTy, Next: SquareListTy>
    RunPMovesForSqL<SLCons<S, Next>, MoverC> for B
where
    S: RunPMovesForSq<B, MoverC>,
    B: RunPMovesForSqL<Next, MoverC>,
    PMovesForSq<S, B, MoverC>: RunConcatML<PMovesForSqL<B, Next, MoverC>>
{
    type Output = ConcatML<PMovesForSq<S, B, MoverC>, PMovesForSqL<B, Next, MoverC>> ;
}
