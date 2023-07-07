use crate::{
    board_rep::{
        board::{Board, BoardRank},
        color::White,
    },
    move_gen::{list::MoveListTy, moves::Moves},
    state::State,
    util::board_creator::*,
};

#[test]
fn test_start_pos_mc() {
    type B = Board<
        //        AA  BB  CC  DD  EE  FF  GG  HH
        BoardRank<WR, WN, WB, WQ, WK, WB, WN, WR>, // 1
        BoardRank<WP, WP, WP, WP, WP, WP, WP, WP>, // 2
        BoardRank<__, __, __, __, __, __, __, __>, // 3
        BoardRank<__, __, __, __, __, __, __, __>, // 4
        BoardRank<__, __, __, __, __, __, __, __>, // 5
        BoardRank<__, __, __, __, __, __, __, __>, // 6
        BoardRank<BP, BP, BP, BP, BP, BP, BP, BP>, // 7
        BoardRank<BR, BN, BB, BQ, BK, BB, BN, BR>, // 8
    >;
    type S = State<White, B>;
    type Ms = Moves<S>;

    assert_eq!(Ms::reify().0.len(), 20);
}

#[test]
fn test_pos_3_mc() {
    type B = Board<
        //        AA  BB  CC  DD  EE  FF  GG  HH
        BoardRank<__, __, __, __, __, __, __, __>, // 1
        BoardRank<__, __, __, __, WP, __, WP, __>, // 2
        BoardRank<__, __, __, __, __, __, __, __>, // 3
        BoardRank<__, WR, __, __, __, BP, __, BK>, // 4
        BoardRank<WK, WP, __, __, __, __, __, BR>, // 5
        BoardRank<__, __, __, BP, __, __, __, __>, // 6
        BoardRank<__, __, BP, __, __, __, __, __>, // 7
        BoardRank<__, __, __, __, __, __, __, __>, // 8
    >;
    type S = State<White, B>;
    type Ms = Moves<S>;

    assert_eq!(Ms::reify().0.len(), 14);
}
