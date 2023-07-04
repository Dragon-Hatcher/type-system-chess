pub mod file;
pub mod offset;
pub mod rank;
pub mod set;

use self::{file::FileEn, rank::RankEn};
use crate::{
    move_gen::list::{SLCons, SLNil},
    values,
};
use std::marker::PhantomData;

pub(crate) trait SquareTy {
    fn reify() -> values::Square;
}
pub(crate) struct Square<R: RankEn, F: FileEn>(PhantomData<(R, F)>);

impl<R: RankEn, F: FileEn> SquareTy for Square<R, F> {
    fn reify() -> values::Square {
        values::Square {
            rank: R::reify(),
            file: F::reify(),
        }
    }
}

// I can't think of a more elegant way to do this.
pub(crate) type AllSqs =
SLCons<Square<rank::R1, file::FA>,
SLCons<Square<rank::R1, file::FB>,
SLCons<Square<rank::R1, file::FC>,
SLCons<Square<rank::R1, file::FD>,
SLCons<Square<rank::R1, file::FE>,
SLCons<Square<rank::R1, file::FF>,
SLCons<Square<rank::R1, file::FG>,
SLCons<Square<rank::R1, file::FH>,
SLCons<Square<rank::R2, file::FA>,
SLCons<Square<rank::R2, file::FB>,
SLCons<Square<rank::R2, file::FC>,
SLCons<Square<rank::R2, file::FD>,
SLCons<Square<rank::R2, file::FE>,
SLCons<Square<rank::R2, file::FF>,
SLCons<Square<rank::R2, file::FG>,
SLCons<Square<rank::R2, file::FH>,
SLCons<Square<rank::R3, file::FA>,
SLCons<Square<rank::R3, file::FB>,
SLCons<Square<rank::R3, file::FC>,
SLCons<Square<rank::R3, file::FD>,
SLCons<Square<rank::R3, file::FE>,
SLCons<Square<rank::R3, file::FF>,
SLCons<Square<rank::R3, file::FG>,
SLCons<Square<rank::R3, file::FH>,
SLCons<Square<rank::R4, file::FA>,
SLCons<Square<rank::R4, file::FB>,
SLCons<Square<rank::R4, file::FC>,
SLCons<Square<rank::R4, file::FD>,
SLCons<Square<rank::R4, file::FE>,
SLCons<Square<rank::R4, file::FF>,
SLCons<Square<rank::R4, file::FG>,
SLCons<Square<rank::R4, file::FH>,
SLCons<Square<rank::R5, file::FA>,
SLCons<Square<rank::R5, file::FB>,
SLCons<Square<rank::R5, file::FC>,
SLCons<Square<rank::R5, file::FD>,
SLCons<Square<rank::R5, file::FE>,
SLCons<Square<rank::R5, file::FF>,
SLCons<Square<rank::R5, file::FG>,
SLCons<Square<rank::R5, file::FH>,
SLCons<Square<rank::R6, file::FA>,
SLCons<Square<rank::R6, file::FB>,
SLCons<Square<rank::R6, file::FC>,
SLCons<Square<rank::R6, file::FD>,
SLCons<Square<rank::R6, file::FE>,
SLCons<Square<rank::R6, file::FF>,
SLCons<Square<rank::R6, file::FG>,
SLCons<Square<rank::R6, file::FH>,
SLCons<Square<rank::R7, file::FA>,
SLCons<Square<rank::R7, file::FB>,
SLCons<Square<rank::R7, file::FC>,
SLCons<Square<rank::R7, file::FD>,
SLCons<Square<rank::R7, file::FE>,
SLCons<Square<rank::R7, file::FF>,
SLCons<Square<rank::R7, file::FG>,
SLCons<Square<rank::R7, file::FH>,
SLCons<Square<rank::R8, file::FA>,
SLCons<Square<rank::R8, file::FB>,
SLCons<Square<rank::R8, file::FC>,
SLCons<Square<rank::R8, file::FD>,
SLCons<Square<rank::R8, file::FE>,
SLCons<Square<rank::R8, file::FF>,
SLCons<Square<rank::R8, file::FG>,
SLCons<Square<rank::R8, file::FH>,
SLNil,
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>;
