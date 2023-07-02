use std::marker::PhantomData;

use crate::{
    board_rep::{board::BoardTy, color::ColorEn},
    values,
};

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
