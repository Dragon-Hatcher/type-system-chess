use super::list::{
    AppendMaybeSquare, MLNil, MoveListTy, RunAppendMaybeSquare, SLNil, SquareListTy,
};
use crate::board_rep::{
    board::BoardTy,
    color::{Black, ColorEn, White},
    square::{
        offset::{Neg1, Offset, Offset1DEn, OffsetSquare, OffsetTy, Pos1, RunOffsetSquare},
        SquareTy,
    },
};

pub(crate) trait RunForwardO<F: Offset1DEn>: ColorEn {
    type Output: OffsetTy;
}
pub(crate) type ForwardO<C, F> = <C as RunForwardO<F>>::Output;
impl<F: Offset1DEn> RunForwardO<F> for White {
    type Output = Offset<Pos1, F>;
}
impl<F: Offset1DEn> RunForwardO<F> for Black {
    type Output = Offset<Neg1, F>;
}

pub(crate) trait RunPawnMoves<B: BoardTy>: SquareTy {
    type Output: MoveListTy;
}
pub(crate) type PawnMoves<S, B> = <S as RunPawnMoves<B>>::Output;

impl<S: SquareTy, B: BoardTy> RunPawnMoves<B> for S {
    type Output = MLNil;
}

pub(crate) trait RunPawnAttackSqs<MoverC: ColorEn>: SquareTy {
    type Output: SquareListTy;
}
pub(crate) type PawnAttackSqs<S, C> = <S as RunPawnAttackSqs<C>>::Output;

type L1<S, C> = AppendMaybeSquare<SLNil, OffsetSquare<S, ForwardO<C, Neg1>>>;
type L2<S, C> = AppendMaybeSquare<L1<S, C>, OffsetSquare<S, ForwardO<C, Pos1>>>;

impl<S: SquareTy, MoverC: ColorEn> RunPawnAttackSqs<MoverC> for S
where
    MoverC: RunForwardO<Neg1> + RunForwardO<Pos1>,
    S: RunOffsetSquare<ForwardO<MoverC, Neg1>> + RunOffsetSquare<ForwardO<MoverC, Pos1>>,
    SLNil: RunAppendMaybeSquare<OffsetSquare<S, ForwardO<MoverC, Neg1>>>,
    L1<S, MoverC>: RunAppendMaybeSquare<OffsetSquare<S, ForwardO<MoverC, Pos1>>>,
{
    type Output = L2<S, MoverC>;
}
