use self::{
    knight::{KnightMoveSqs, RunKnightMoveSqs},
    list::{AppendMaybeMove, MLNil, MoveListTy, RunAppendMaybeMove, SLCons, SLNil, SquareListTy},
};
use crate::{
    board_rep::{
        board::{
            idx::{IdxBoard, IdxBoardRank, RunIdxBoard, RunIdxBoardRank, RunIdxRank},
            BoardTy, CellEn, Empty, Filled,
        },
        color::{Black, ColorEn, White},
        piece::{ColoredPiece, Knight, PieceEn},
        square::{file::FileEn, rank::RankEn, Square, SquareTy},
    },
    values,
};
use std::marker::PhantomData;

pub mod knight;
pub mod list;

pub(crate) trait MoveTy {
    fn reify() -> values::Move;
}
pub(crate) struct Move<From: SquareTy, To: SquareTy>(PhantomData<(From, To)>);

pub(crate) trait MaybeMove {
    fn reify() -> Option<values::Move>;
}
pub(crate) struct NoMove;
pub(crate) struct SomeMove<M: MoveTy>(PhantomData<M>);

impl MaybeMove for NoMove {
    fn reify() -> Option<values::Move> {
        None
    }
}
impl<M: MoveTy> MaybeMove for SomeMove<M> {
    fn reify() -> Option<values::Move> {
        Some(M::reify())
    }
}

impl<From: SquareTy, To: SquareTy> MoveTy for Move<From, To> {
    fn reify() -> values::Move {
        values::Move {
            from: From::reify(),
            to: To::reify(),
        }
    }
}

pub(crate) trait RunPMoveLFromSqs<B: BoardTy, Start: SquareTy, MoverC: ColorEn>:
    SquareListTy
{
    type Output: MoveListTy;
}
pub(crate) type PMoveLFromSqs<L, B, Start, MoverC> =
    <L as RunPMoveLFromSqs<B, Start, MoverC>>::Output;

impl<B: BoardTy, Start: SquareTy, MoverC: ColorEn> RunPMoveLFromSqs<B, Start, MoverC> for SLNil {
    type Output = MLNil;
}
impl<Rest: SquareListTy, End: SquareTy, B: BoardTy, Start: SquareTy, MoverC: ColorEn>
    RunPMoveLFromSqs<B, Start, MoverC> for SLCons<End, Rest>
where
    End: RunPMoveFromSq<B, Start, MoverC>,
    Rest: RunPMoveLFromSqs<B, Start, MoverC>,
    PMoveLFromSqs<Rest, B, Start, MoverC>:
        RunAppendMaybeMove<<End as RunPMoveFromSq<B, Start, MoverC>>::Output>,
{
    type Output = AppendMaybeMove<
        PMoveLFromSqs<Rest, B, Start, MoverC>,
        <End as RunPMoveFromSq<B, Start, MoverC>>::Output,
    >;
}

pub(crate) trait RunPMoveFromSq<B: BoardTy, Start: SquareTy, MoverC: ColorEn>:
    SquareTy
{
    type Output: MaybeMove;
}

impl<R: RankEn, F: FileEn, B: BoardTy, Start: SquareTy, MoverC: ColorEn>
    RunPMoveFromSq<B, Start, MoverC> for Square<R, F>
where
    B: RunIdxBoardRank<R>,
    IdxBoardRank<B, R>: RunIdxRank<F>,
    Square<R, F>: RunPMoveFromSqContents<Start, MoverC, IdxBoard<B, Square<R, F>>>,
{
    type Output =
        <Square<R, F> as RunPMoveFromSqContents<Start, MoverC, IdxBoard<B, Square<R, F>>>>::Output;
}

pub(crate) trait RunPMoveFromSqContents<Start: SquareTy, MoverC: ColorEn, Contents: CellEn>:
    SquareTy
{
    type Output: MaybeMove;
}

impl<End: SquareTy, Start: SquareTy, MoverC: ColorEn> RunPMoveFromSqContents<Start, MoverC, Empty>
    for End
{
    type Output = SomeMove<Move<Start, End>>;
}
impl<End: SquareTy, Start: SquareTy, P: PieceEn>
    RunPMoveFromSqContents<Start, White, Filled<ColoredPiece<P, White>>> for End
{
    type Output = NoMove;
}
impl<End: SquareTy, Start: SquareTy, P: PieceEn>
    RunPMoveFromSqContents<Start, Black, Filled<ColoredPiece<P, Black>>> for End
{
    type Output = NoMove;
}
impl<End: SquareTy, Start: SquareTy, P: PieceEn>
    RunPMoveFromSqContents<Start, White, Filled<ColoredPiece<P, Black>>> for End
{
    type Output = SomeMove<Move<Start, End>>;
}
impl<End: SquareTy, Start: SquareTy, P: PieceEn>
    RunPMoveFromSqContents<Start, Black, Filled<ColoredPiece<P, White>>> for End
{
    type Output = SomeMove<Move<Start, End>>;
}

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
impl<S: SquareTy, B: BoardTy> RunPMovesForTypeAtSq<B, White, Filled<ColoredPiece<Knight, White>>>
    for S
where
    S: RunKnightMoveSqs,
    KnightMoveSqs<S>: RunPMoveLFromSqs<B, S, White>,
{
    type Output = PMoveLFromSqs<KnightMoveSqs<S>, B, S, White>;
}
impl<S: SquareTy, B: BoardTy> RunPMovesForTypeAtSq<B, Black, Filled<ColoredPiece<Knight, Black>>>
    for S
where
    S: RunKnightMoveSqs,
    KnightMoveSqs<S>: RunPMoveLFromSqs<B, S, Black>,
{
    type Output = PMoveLFromSqs<KnightMoveSqs<S>, B, S, Black>;
}
