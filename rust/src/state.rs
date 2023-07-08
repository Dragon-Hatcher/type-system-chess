use crate::{
    board_rep::{
        board::{
            write::{RunWriteToBoard, WriteToBoard},
            BoardTy, Empty, Filled,
        },
        color::{Black, ColorEn, White},
        piece::{Bishop, ColoredPiece, ColoredPieceTy, King, Knight, Pawn, Queen, Rook},
        square::{
            offset::{
                MaybeSquare, Neg1, Neg2, NoSquare, Offset, OffsetSquare, OffsetTy, Pos1, Pos2,
                RunOffsetSquare, SomeSquare, Zero,
            },
            RunSquareEq, SquareEq, SquareTy,
        },
    },
    move_gen::{Move, MoveTy},
    util::{Bool, False, True},
    values,
};
use std::marker::PhantomData;

pub mod outcome;

pub(crate) trait StateTy {
    fn reify() -> values::State;
}
pub(crate) struct State<ToMove: ColorEn, Pieces: BoardTy, EPSquare: MaybeSquare>(
    PhantomData<(ToMove, Pieces, EPSquare)>,
);

impl<ToMove: ColorEn, Pieces: BoardTy, EPSquare: MaybeSquare> StateTy
    for State<ToMove, Pieces, EPSquare>
{
    fn reify() -> values::State {
        values::State {
            to_move: ToMove::reify(),
            pieces: Pieces::reify(),
            ep_square: EPSquare::reify(),
        }
    }
}

pub(crate) trait RunMakeMove<S: StateTy>: MoveTy {
    type Output: StateTy;
}
pub(crate) type MakeMove<M, S> = <M as RunMakeMove<S>>::Output;

impl<M: MoveTy, B: BoardTy, EPSquare: MaybeSquare> RunMakeMove<State<White, B, EPSquare>> for M
where
    M: RunMakeMoveForBoard<B>,
    M: RunMakeMoveForEp,
{
    type Output = State<Black, MakeMoveForBoard<M, B>, MakeMoveForEP<M>>;
}
impl<M: MoveTy, B: BoardTy, EPSquare: MaybeSquare> RunMakeMove<State<Black, B, EPSquare>> for M
where
    M: RunMakeMoveForBoard<B>,
    M: RunMakeMoveForEp,
{
    type Output = State<White, MakeMoveForBoard<M, B>, MakeMoveForEP<M>>;
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
impl<B: BoardTy, F: SquareTy, T: SquareTy, P: ColoredPieceTy, EP: SquareTy> RunMakeMoveForBoard<B>
    for Move<F, T, P, SomeSquare<EP>>
where
    B: RunWriteToBoard<T, Filled<P>>,
    WriteToBoard<B, T, Filled<P>>: RunWriteToBoard<F, Empty>,
    WriteToBoard<WriteToBoard<B, T, Filled<P>>, F, Empty>: RunWriteToBoard<EP, Empty>,
{
    type Output = WriteToBoard<WriteToBoard<WriteToBoard<B, T, Filled<P>>, F, Empty>, EP, Empty>;
}

pub(crate) trait RunMakeMoveForEp: MoveTy {
    type Output: MaybeSquare;
}
pub(crate) type MakeMoveForEP<M> = <M as RunMakeMoveForEp>::Output;

impl<F: SquareTy, T: SquareTy, MoverC: ColorEn> RunMakeMoveForEp
    for Move<F, T, ColoredPiece<Bishop, MoverC>>
{
    type Output = NoSquare;
}
impl<F: SquareTy, T: SquareTy, MoverC: ColorEn> RunMakeMoveForEp
    for Move<F, T, ColoredPiece<Knight, MoverC>>
{
    type Output = NoSquare;
}
impl<F: SquareTy, T: SquareTy, MoverC: ColorEn> RunMakeMoveForEp
    for Move<F, T, ColoredPiece<Rook, MoverC>>
{
    type Output = NoSquare;
}
impl<F: SquareTy, T: SquareTy, MoverC: ColorEn> RunMakeMoveForEp
    for Move<F, T, ColoredPiece<Queen, MoverC>>
{
    type Output = NoSquare;
}
impl<F: SquareTy, T: SquareTy, MoverC: ColorEn> RunMakeMoveForEp
    for Move<F, T, ColoredPiece<King, MoverC>>
{
    type Output = NoSquare;
}
impl<F: SquareTy, T: SquareTy, MoverC: ColorEn, EP: MaybeSquare> RunMakeMoveForEp
    for Move<F, T, ColoredPiece<Pawn, MoverC>, EP>
where
    MoverC: RunForward1O + RunForward2O,
    F: RunOffsetSquare<Forward2O<MoverC>> + RunOffsetSquare<Forward1O<MoverC>>,
    T: RunMaybeSqEq<OffsetSquare<F, Forward2O<MoverC>>>,
    OffsetSquare<F, Forward1O<MoverC>>:
        RunSelectIf<MaybeSqEq<T, OffsetSquare<F, Forward2O<MoverC>>>>,
{
    type Output = <OffsetSquare<F, Forward1O<MoverC>> as RunSelectIf<
        MaybeSqEq<T, OffsetSquare<F, Forward2O<MoverC>>>,
    >>::Output;
}

pub(crate) trait RunMaybeSqEq<MS: MaybeSquare>: SquareTy {
    type Output: Bool;
}
pub(crate) type MaybeSqEq<S, MS> = <S as RunMaybeSqEq<MS>>::Output;

impl<S: SquareTy> RunMaybeSqEq<NoSquare> for S {
    type Output = False;
}
impl<S1: SquareTy, S2: SquareTy> RunMaybeSqEq<SomeSquare<S1>> for S2
where
    S1: RunSquareEq<S2>,
{
    type Output = SquareEq<S1, S2>;
}

pub(crate) trait RunForward2O: ColorEn {
    type Output: OffsetTy;
}
pub(crate) type Forward2O<C> = <C as RunForward2O>::Output;
impl RunForward2O for White {
    type Output = Offset<Pos2, Zero>;
}
impl RunForward2O for Black {
    type Output = Offset<Neg2, Zero>;
}

pub(crate) trait RunForward1O: ColorEn {
    type Output: OffsetTy;
}
pub(crate) type Forward1O<C> = <C as RunForward1O>::Output;
impl RunForward1O for White {
    type Output = Offset<Pos1, Zero>;
}
impl RunForward1O for Black {
    type Output = Offset<Neg1, Zero>;
}

pub(crate) trait RunSelectIf<B: Bool>: MaybeSquare {
    type Output: MaybeSquare;
}

impl<B: Bool> RunSelectIf<B> for NoSquare {
    type Output = NoSquare;
}
impl<S: SquareTy> RunSelectIf<False> for SomeSquare<S> {
    type Output = NoSquare;
}
impl<S: SquareTy> RunSelectIf<True> for SomeSquare<S> {
    type Output = Self;
}
