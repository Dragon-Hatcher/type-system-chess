use self::{
    bishop::{BishopMoveSqs, RunBishopMoveSqs},
    king::{KingMoveSqs, RunKingMoveSqs},
    knight::{KnightMoveSqs, RunKnightMoveSqs},
    list::{MLNil, MoveListTy},
    ml_from_sl::{PMoveLFromSqs, RunPMoveLFromSqs},
    pawn::{PawnMoves, RunPawnMoves},
    queen::{QueenMoveSqs, RunQueenMoveSqs},
    rook::{RookMoveSqs, RunRookMoveSqs},
};
use crate::{
    board_rep::{
        board::{
            idx::{IdxBoard, RunIdxBoard},
            BoardTy, CellEn, Empty, Filled,
        },
        color::{Black, ColorEn, White},
        piece::{Bishop, ColoredPiece, ColoredPieceTy, King, Knight, Pawn, PieceEn, Queen, Rook},
        square::SquareTy,
    },
    values,
};
use std::marker::PhantomData;

pub mod attacked;
pub mod bishop;
pub mod cast;
pub mod king;
pub mod knight;
pub mod list;
pub mod ml_from_sl;
pub mod pawn;
pub mod queen;
pub mod rook;

pub(crate) trait MoveTy {
    fn reify() -> values::Move;
}
pub(crate) struct Move<From: SquareTy, To: SquareTy, P: ColoredPieceTy>(PhantomData<(From, To, P)>);

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

impl<From: SquareTy, To: SquareTy, P: ColoredPieceTy> MoveTy for Move<From, To, P> {
    fn reify() -> values::Move {
        values::Move {
            from: From::reify(),
            to: To::reify(),
            piece: P::reify(),
        }
    }
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
