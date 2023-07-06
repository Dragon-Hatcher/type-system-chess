use crate::{
    board_rep::{
        board::{
            write::{RunWriteToBoard, WriteToBoard},
            BoardTy, Empty, Filled,
        },
        color::{Black, ColorEn, White},
        piece::ColoredPieceTy,
        square::SquareTy,
    },
    move_gen::{Move, MoveTy},
    values,
};
use std::marker::PhantomData;

pub mod outcome;

pub(crate) trait StateTy {
    fn reify() -> values::State;
}
pub(crate) struct State<ToMove: ColorEn, Pieces: BoardTy>(PhantomData<(ToMove, Pieces)>);

impl<ToMove: ColorEn, Pieces: BoardTy> StateTy for State<ToMove, Pieces> {
    fn reify() -> values::State {
        values::State {
            to_move: ToMove::reify(),
            pieces: Pieces::reify(),
        }
    }
}

pub(crate) trait RunMakeMove<S: StateTy>: MoveTy {
    type Output: StateTy;
}
pub(crate) type MakeMove<M, S> = <M as RunMakeMove<S>>::Output;

impl<M: MoveTy, B: BoardTy> RunMakeMove<State<White, B>> for M
where
    M: RunMakeMoveForBoard<B>,
{
    type Output = State<Black, MakeMoveForBoard<M, B>>;
}
impl<M: MoveTy, B: BoardTy> RunMakeMove<State<Black, B>> for M
where
    M: RunMakeMoveForBoard<B>,
{
    type Output = State<White, MakeMoveForBoard<M, B>>;
}

pub(crate) trait RunMakeMoveForBoard<B: BoardTy>: MoveTy {
    type Output: BoardTy;
}
pub(crate) type MakeMoveForBoard<M, B> = <M as RunMakeMoveForBoard<B>>::Output;

impl<B: BoardTy, F: SquareTy, T: SquareTy, P: ColoredPieceTy> RunMakeMoveForBoard<B>
    for Move<F, T, P>
where
    B: RunWriteToBoard<T, Filled<P>>,
    WriteToBoard<B, T, Filled<P>>: RunWriteToBoard<F, Empty>,
{
    type Output = WriteToBoard<WriteToBoard<B, T, Filled<P>>, F, Empty>;
}
