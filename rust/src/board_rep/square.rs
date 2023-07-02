pub mod file;
pub mod rank;
pub mod offset;

use crate::values;
use self::{file::FileEn, rank::RankEn};
use std::marker::PhantomData;

pub(crate) trait SquareTy {
    fn reify() -> values::Square;
}
pub(crate) struct Square<R: RankEn, F: FileEn>(PhantomData<(R, F)>);

impl<R: RankEn, F: FileEn> SquareTy for Square<R, F> {
    fn reify() -> values::Square {
        values::Square { rank: R::reify(), file: F::reify() }
    }
}
