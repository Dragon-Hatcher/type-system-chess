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
    list::{MLNil, MoveListTy, SLCons, SLNil, SquareListTy},
    ml_from_sl::{PMoveLFromSqs, RunPMoveLFromSqs},
    pawn::{PawnMoves, RunPawnMoves},
    queen::{QueenMoveSqs, RunQueenMoveSqs},
    rook::{RookMoveSqs, RunRookMoveSqs},
};

pub(crate) trait RunPMovesForSq<B: BoardTy, MoverC: ColorEn, ML: MoveListTy>:
    SquareTy
{
    type Output: MoveListTy;
}
pub(crate) type PMovesForSq<S, B, MoverC, ML> = <S as RunPMovesForSq<B, MoverC, ML>>::Output;

impl<S: SquareTy, B: BoardTy, MoverC: ColorEn, ML: MoveListTy> RunPMovesForSq<B, MoverC, ML> for S
where
    B: RunIdxBoard<S>,
    S: RunPMovesForTypeAtSq<B, MoverC, IdxBoard<B, S>, ML>,
{
    type Output = <S as RunPMovesForTypeAtSq<B, MoverC, IdxBoard<B, S>, ML>>::Output;
}

pub(crate) trait RunPMovesForTypeAtSq<B: BoardTy, MoverC: ColorEn, Type: CellEn, ML: MoveListTy>:
    SquareTy
{
    type Output: MoveListTy;
}

impl<S: SquareTy, B: BoardTy, MoverC: ColorEn, ML: MoveListTy> RunPMovesForTypeAtSq<B, MoverC, Empty, ML> for S {
    type Output = ML;
}
impl<S: SquareTy, B: BoardTy, P: PieceEn, ML: MoveListTy>
    RunPMovesForTypeAtSq<B, White, Filled<ColoredPiece<P, Black>>, ML> for S
{
    type Output = ML;
}
impl<S: SquareTy, B: BoardTy, P: PieceEn, ML: MoveListTy>
    RunPMovesForTypeAtSq<B, Black, Filled<ColoredPiece<P, White>>, ML> for S
{
    type Output = ML;
}
impl<S: SquareTy, B: BoardTy, C: ColorEn, ML: MoveListTy>
    RunPMovesForTypeAtSq<B, C, Filled<ColoredPiece<Knight, C>>, ML> for S
where
    S: RunKnightMoveSqs,
    KnightMoveSqs<S>: RunPMoveLFromSqs<B, S, C, Knight, ML>,
{
    type Output = PMoveLFromSqs<KnightMoveSqs<S>, B, S, C, Knight, ML>;
}
impl<S: SquareTy, B: BoardTy, C: ColorEn, ML: MoveListTy> RunPMovesForTypeAtSq<B, C, Filled<ColoredPiece<King, C>>, ML>
    for S
where
    S: RunKingMoveSqs,
    KingMoveSqs<S>: RunPMoveLFromSqs<B, S, C, King, ML>,
{
    type Output = PMoveLFromSqs<KingMoveSqs<S>, B, S, C, King, ML>;
}
impl<S: SquareTy, B: BoardTy, C: ColorEn, ML: MoveListTy> RunPMovesForTypeAtSq<B, C, Filled<ColoredPiece<Rook, C>>, ML>
    for S
where
    S: RunRookMoveSqs<B>,
    RookMoveSqs<S, B>: RunPMoveLFromSqs<B, S, C, Rook, ML>,
{
    type Output = PMoveLFromSqs<RookMoveSqs<S, B>, B, S, C, Rook, ML>;
}
impl<S: SquareTy, B: BoardTy, C: ColorEn, ML: MoveListTy>
    RunPMovesForTypeAtSq<B, C, Filled<ColoredPiece<Bishop, C>>, ML> for S
where
    S: RunBishopMoveSqs<B>,
    BishopMoveSqs<S, B>: RunPMoveLFromSqs<B, S, C, Bishop, ML>,
{
    type Output = PMoveLFromSqs<BishopMoveSqs<S, B>, B, S, C, Bishop, ML>;
}
impl<S: SquareTy, B: BoardTy, C: ColorEn, ML: MoveListTy> RunPMovesForTypeAtSq<B, C, Filled<ColoredPiece<Queen, C>>, ML>
    for S
where
    S: RunQueenMoveSqs<B>,
    QueenMoveSqs<S, B>: RunPMoveLFromSqs<B, S, C, Queen, ML>,
{
    type Output = PMoveLFromSqs<QueenMoveSqs<S, B>, B, S, C, Queen, ML>;
}
impl<S: SquareTy, B: BoardTy, ML: MoveListTy> RunPMovesForTypeAtSq<B, White, Filled<ColoredPiece<Pawn, White>>, ML>
    for S
where
    S: RunPawnMoves<B, White, ML>,
{
    type Output = PawnMoves<S, B, White, ML>;
}
impl<S: SquareTy, B: BoardTy, ML: MoveListTy> RunPMovesForTypeAtSq<B, Black, Filled<ColoredPiece<Pawn, Black>>, ML>
    for S
where
    S: RunPawnMoves<B, Black, ML>,
{
    type Output = PawnMoves<S, B, Black, ML>;
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
    S: RunPMovesForSq<B, MoverC, PMovesForSqL<B, Next, MoverC>>,
    B: RunPMovesForSqL<Next, MoverC>,
{
    type Output = PMovesForSq<S, B, MoverC, PMovesForSqL<B, Next, MoverC>>;
}
