#![allow(clippy::type_complexity)]

use crate::{
    board_rep::{
        board::{Board, BoardRank, BoardTy, Empty, Filled},
        color::{self, White},
        piece::{self, ColoredPiece},
        square::{file, rank, set::SquareSetTy, Square},
    },
    move_gen::{attacked::Attacked, list::MoveListTy, PMovesForSq},
};

mod board_rep;
mod move_gen;
mod state;
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
type EE = Empty;

type B = Board<
    //        AA  BB  CC  DD  EE  FF  GG  HH
    BoardRank<WN, EE, EE, EE, EE, EE, EE, EE>, // 1
    BoardRank<EE, EE, BP, EE, EE, EE, EE, EE>, // 2
    BoardRank<EE, WP, EE, EE, EE, EE, EE, EE>, // 3
    BoardRank<EE, EE, EE, EE, EE, EE, EE, EE>, // 4
    BoardRank<WP, EE, WR, EE, EE, EE, EE, EE>, // 5
    BoardRank<EE, EE, EE, EE, EE, EE, EE, EE>, // 6
    BoardRank<EE, EE, EE, EE, EE, EE, EE, EE>, // 7
    BoardRank<EE, EE, EE, EE, EE, EE, EE, EE>, // 8
>;

fn main() {
    type S = Square<rank::R5, file::FC>;
    type Y = PMovesForSq<S, B, White>;
    type Z = Attacked<B, White>;

    println!("{}", B::reify());
    println!("{}", Y::reify().destinations());
    println!("{}", Z::reify());
}
