#![allow(clippy::type_complexity)]

use crate::{
    board_rep::{
        board::{Board, BoardRank, BoardTy},
        color::White,
    },
    move_gen::{list::MoveListTy, moves::Moves},
    state::State,
};
use util::board_creator::*;

mod board_rep;
mod move_gen;
mod state;
#[cfg(test)]
mod test;
mod util;
mod values;

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
