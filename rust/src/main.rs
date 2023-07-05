#![allow(clippy::type_complexity)]

use crate::{
    board_rep::{
        board::{Board, BoardRank, BoardTy, Empty, Filled},
        color::{self, White},
        piece::{self, ColoredPiece},
    },
    move_gen::{list::MoveListTy, moves::Moves},
    state::State,
};

mod board_rep;
mod move_gen;
mod state;
mod util;
mod values;

type WP = Filled<ColoredPiece<piece::Pawn, color::White>>;
type WB = Filled<ColoredPiece<piece::Bishop, color::White>>;
type WN = Filled<ColoredPiece<piece::Knight, color::White>>;
type WR = Filled<ColoredPiece<piece::Rook, color::White>>;
type WQ = Filled<ColoredPiece<piece::Queen, color::White>>;
type WK = Filled<ColoredPiece<piece::King, color::White>>;
type BP = Filled<ColoredPiece<piece::Pawn, color::Black>>;
type BB = Filled<ColoredPiece<piece::Bishop, color::Black>>;
type BN = Filled<ColoredPiece<piece::Knight, color::Black>>;
type BR = Filled<ColoredPiece<piece::Rook, color::Black>>;
type BQ = Filled<ColoredPiece<piece::Queen, color::Black>>;
type BK = Filled<ColoredPiece<piece::King, color::Black>>;
type __ = Empty;

type B = Board<
    //        AA  BB  CC  DD  EE  FF  GG  HH
    BoardRank<__, __, __, __, __, __, __, __>, // 1
    BoardRank<__, __, __, __, __, __, __, __>, // 2
    BoardRank<BR, __, WR, __, __, WK, __, __>, // 3
    BoardRank<__, __, __, __, __, __, __, __>, // 4
    BoardRank<__, __, __, __, __, __, __, __>, // 5
    BoardRank<__, __, __, __, __, __, __, __>, // 6
    BoardRank<__, __, __, __, __, __, __, __>, // 7
    BoardRank<__, __, __, __, __, __, __, __>, // 8
>;

fn main() {
    type M = Moves<State<White, B>>;

    println!("Board:\n{}", B::reify());
    println!("Moves:\n{}", M::reify().destinations());
}
