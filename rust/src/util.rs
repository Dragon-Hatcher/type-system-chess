pub(crate) trait Bool {
    fn reify() -> bool;
}
pub(crate) struct True;
pub(crate) struct False;

impl Bool for True {
    fn reify() -> bool {
        true
    }
}
impl Bool for False {
    fn reify() -> bool {
        false
    }
}

pub(crate) trait RunOr<A: Bool>: Bool {
    type Output: Bool;
}
pub(crate) type Or<A, B> = <A as RunOr<B>>::Output;

impl RunOr<True> for True {
    type Output = True;
}
impl RunOr<True> for False {
    type Output = True;
}
impl RunOr<False> for True {
    type Output = True;
}
impl RunOr<False> for False {
    type Output = False;
}

pub mod board_creator {
    #![allow(unused)]

    use crate::board_rep::{board::{Filled, Empty}, piece::{ColoredPiece, self}, color};

    pub(crate) type WP = Filled<ColoredPiece<piece::Pawn, color::White>>;
    pub(crate) type WB = Filled<ColoredPiece<piece::Bishop, color::White>>;
    pub(crate) type WN = Filled<ColoredPiece<piece::Knight, color::White>>;
    pub(crate) type WR = Filled<ColoredPiece<piece::Rook, color::White>>;
    pub(crate) type WQ = Filled<ColoredPiece<piece::Queen, color::White>>;
    pub(crate) type WK = Filled<ColoredPiece<piece::King, color::White>>;
    pub(crate) type BP = Filled<ColoredPiece<piece::Pawn, color::Black>>;
    pub(crate) type BB = Filled<ColoredPiece<piece::Bishop, color::Black>>;
    pub(crate) type BN = Filled<ColoredPiece<piece::Knight, color::Black>>;
    pub(crate) type BR = Filled<ColoredPiece<piece::Rook, color::Black>>;
    pub(crate) type BQ = Filled<ColoredPiece<piece::Queen, color::Black>>;
    pub(crate) type BK = Filled<ColoredPiece<piece::King, color::Black>>;
    pub(crate) type __ = Empty;
}
