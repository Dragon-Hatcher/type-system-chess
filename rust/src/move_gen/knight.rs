use super::list::{AppendMaybeSquare, RunAppendMaybeSquare, SLNil, SquareListTy};
use crate::board_rep::square::{
    offset::{self, Offset, OffsetSquare, RunOffsetSquare},
    SquareTy,
};

pub(crate) trait RunKnightMoveSqs: SquareTy {
    type Output: SquareListTy;
}
pub(crate) type KnightMoveSqs<S> = <S as RunKnightMoveSqs>::Output;

type O1 = Offset<offset::Neg2, offset::Neg1>;
type O2 = Offset<offset::Neg2, offset::Pos1>;
type O3 = Offset<offset::Pos2, offset::Neg1>;
type O4 = Offset<offset::Pos2, offset::Pos1>;
type O5 = Offset<offset::Neg1, offset::Neg2>;
type O6 = Offset<offset::Pos1, offset::Neg2>;
type O7 = Offset<offset::Neg1, offset::Pos2>;
type O8 = Offset<offset::Pos1, offset::Pos2>;

type L1<S> = AppendMaybeSquare<SLNil, OffsetSquare<S, O1>>;
type L2<S> = AppendMaybeSquare<L1<S>, OffsetSquare<S, O2>>;
type L3<S> = AppendMaybeSquare<L2<S>, OffsetSquare<S, O3>>;
type L4<S> = AppendMaybeSquare<L3<S>, OffsetSquare<S, O4>>;
type L5<S> = AppendMaybeSquare<L4<S>, OffsetSquare<S, O5>>;
type L6<S> = AppendMaybeSquare<L5<S>, OffsetSquare<S, O6>>;
type L7<S> = AppendMaybeSquare<L6<S>, OffsetSquare<S, O7>>;
type L8<S> = AppendMaybeSquare<L7<S>, OffsetSquare<S, O8>>;

impl<S> RunKnightMoveSqs for S
where
    S: SquareTy
        + RunOffsetSquare<O1>
        + RunOffsetSquare<O2>
        + RunOffsetSquare<O3>
        + RunOffsetSquare<O4>
        + RunOffsetSquare<O5>
        + RunOffsetSquare<O6>
        + RunOffsetSquare<O7>
        + RunOffsetSquare<O8>,
    SLNil: RunAppendMaybeSquare<OffsetSquare<S, O1>>,
    L1<S>: RunAppendMaybeSquare<OffsetSquare<S, O2>>,
    L2<S>: RunAppendMaybeSquare<OffsetSquare<S, O3>>,
    L3<S>: RunAppendMaybeSquare<OffsetSquare<S, O4>>,
    L4<S>: RunAppendMaybeSquare<OffsetSquare<S, O5>>,
    L5<S>: RunAppendMaybeSquare<OffsetSquare<S, O6>>,
    L6<S>: RunAppendMaybeSquare<OffsetSquare<S, O7>>,
    L7<S>: RunAppendMaybeSquare<OffsetSquare<S, O8>>,
{
    type Output = L8<S>;
}
