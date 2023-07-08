use std::marker::PhantomData;

use crate::{
    board_rep::{
        board::BoardTy,
        color::{Black, ColorEn, White},
        square::offset::MaybeSquare,
    },
    move_gen::{
        check::{IsCheck, RunIsCheck},
        list::{MLCons, MLNil, MoveListTy},
        moves::{Moves, RunMoves},
        MoveTy,
    },
    util::{Bool, False, True},
    values,
};

use super::{State, StateTy};

pub(crate) trait OutcomeEn {
    fn reify() -> values::Outcome;
}
pub(crate) struct Ongoing;
pub(crate) struct Draw;
pub(crate) struct Checkmate<Winner: ColorEn>(PhantomData<Winner>);

impl OutcomeEn for Ongoing {
    fn reify() -> values::Outcome {
        values::Outcome::Ongoing
    }
}
impl OutcomeEn for Draw {
    fn reify() -> values::Outcome {
        values::Outcome::Draw
    }
}
impl<W: ColorEn> OutcomeEn for Checkmate<W> {
    fn reify() -> values::Outcome {
        values::Outcome::Checkmate(W::reify())
    }
}

pub(crate) trait RunOutcome: StateTy {
    type Output: OutcomeEn;
}
pub(crate) type Outcome<S> = <S as RunOutcome>::Output;

impl<B: BoardTy, EP: MaybeSquare> RunOutcome for State<White, B, EP>
where
    State<White, B, EP>: RunMoves,
    State<White, B, EP>: RunOutcomeWML<Moves<State<White, B, EP>>, Black>,
{
    type Output = <State<White, B, EP> as RunOutcomeWML<Moves<State<White, B, EP>>, Black>>::Output;
}
impl<B: BoardTy, EP: MaybeSquare> RunOutcome for State<Black, B, EP>
where
    State<Black, B, EP>: RunMoves,
    State<Black, B, EP>: RunOutcomeWML<Moves<State<Black, B, EP>>, White>,
{
    type Output = <State<Black, B, EP> as RunOutcomeWML<Moves<State<Black, B, EP>>, White>>::Output;
}

pub(crate) trait RunOutcomeWML<ML: MoveListTy, MoverC: ColorEn>: StateTy {
    type Output: OutcomeEn;
}

impl<M: MoveTy, L: MoveListTy, S: StateTy, MoverC: ColorEn> RunOutcomeWML<MLCons<M, L>, MoverC>
    for S
{
    type Output = Ongoing;
}
impl<S: StateTy, MoverC: ColorEn> RunOutcomeWML<MLNil, MoverC> for S
where
    S: RunIsCheck<MoverC>,
    S: RunOutcomeWIsCheck<IsCheck<S, MoverC>>,
{
    type Output = <S as RunOutcomeWIsCheck<IsCheck<S, MoverC>>>::Output;
}

pub(crate) trait RunOutcomeWIsCheck<C: Bool>: StateTy {
    type Output: OutcomeEn;
}

impl<S: StateTy> RunOutcomeWIsCheck<False> for S {
    type Output = Draw;
}
impl<B: BoardTy, EP: MaybeSquare> RunOutcomeWIsCheck<True> for State<White, B, EP> {
    type Output = Checkmate<Black>;
}
impl<B: BoardTy, EP: MaybeSquare> RunOutcomeWIsCheck<True> for State<Black, B, EP> {
    type Output = Checkmate<White>;
}
