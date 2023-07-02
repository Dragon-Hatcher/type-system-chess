use crate::values;

pub(crate) trait RankEn {
    fn reify() -> values::Rank;
}
pub(crate) struct R1;
pub(crate) struct R2;
pub(crate) struct R3;
pub(crate) struct R4;
pub(crate) struct R5;
pub(crate) struct R6;
pub(crate) struct R7;
pub(crate) struct R8;

impl RankEn for R1 {
    fn reify() -> values::Rank {
        values::Rank::R1
    }
}
impl RankEn for R2 {
    fn reify() -> values::Rank {
        values::Rank::R2
    }
}
impl RankEn for R3 {
    fn reify() -> values::Rank {
        values::Rank::R3
    }
}
impl RankEn for R4 {
    fn reify() -> values::Rank {
        values::Rank::R4
    }
}
impl RankEn for R5 {
    fn reify() -> values::Rank {
        values::Rank::R5
    }
}
impl RankEn for R6 {
    fn reify() -> values::Rank {
        values::Rank::R6
    }
}
impl RankEn for R7 {
    fn reify() -> values::Rank {
        values::Rank::R7
    }
}
impl RankEn for R8 {
    fn reify() -> values::Rank {
        values::Rank::R8
    }
}
