use crate::values;

use super::color::ColorEn;
use std::marker::PhantomData;

pub(crate) trait PieceEn {
    fn reify() -> values::Piece;
}
pub(crate) struct Pawn;
pub(crate) struct Bishop;
pub(crate) struct Knight;
pub(crate) struct Rook;
pub(crate) struct Queen;
pub(crate) struct King;

impl PieceEn for Pawn {
    fn reify() -> values::Piece {
        values::Piece::Pawn
    }
}
impl PieceEn for Bishop {
    fn reify() -> values::Piece {
        values::Piece::Bishop
    }
}
impl PieceEn for Knight {
    fn reify() -> values::Piece {
        values::Piece::Knight
    }
}
impl PieceEn for Rook {
    fn reify() -> values::Piece {
        values::Piece::Rook
    }
}
impl PieceEn for Queen {
    fn reify() -> values::Piece {
        values::Piece::Queen
    }
}
impl PieceEn for King {
    fn reify() -> values::Piece {
        values::Piece::King
    }
}

pub(crate) trait ColoredPieceTy {
    fn reify() -> values::ColoredPiece;
}
pub(crate) struct ColoredPiece<P: PieceEn, C: ColorEn>(PhantomData<(P, C)>);

impl<P: PieceEn, C: ColorEn> ColoredPieceTy for ColoredPiece<P, C> {
    fn reify() -> values::ColoredPiece {
        values::ColoredPiece {
            piece: P::reify(),
            color: C::reify(),
        }
    }
}
