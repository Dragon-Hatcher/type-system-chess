use std::marker::PhantomData;

use crate::{
    board_rep::{
        board::BoardTy,
        color::{Black, ColorEn, White},
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

impl<B: BoardTy> RunOutcome for State<White, B>
where
    State<White, B>: RunMoves,
    State<White, B>: RunOutcomeWML<Moves<State<White, B>>, Black>,
{
    type Output = <State<White, B> as RunOutcomeWML<Moves<State<White, B>>, Black>>::Output;
}
impl<B: BoardTy> RunOutcome for State<Black, B>
where
    State<Black, B>: RunMoves,
    State<Black, B>: RunOutcomeWML<Moves<State<Black, B>>, White>,
{
    type Output = <State<Black, B> as RunOutcomeWML<Moves<State<Black, B>>, White>>::Output;
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
impl<B: BoardTy> RunOutcomeWIsCheck<True> for State<White, B> {
    type Output = Checkmate<Black>;
}
impl<B: BoardTy> RunOutcomeWIsCheck<True> for State<Black, B> {
    type Output = Checkmate<White>;
}
