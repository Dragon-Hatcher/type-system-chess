use crate::{
    board_rep::{
        board::{Board, BoardRank},
        color::White,
        square::{
            file,
            offset::{NoSquare, SomeSquare},
            rank, Square,
        },
    },
    move_gen::{list::MoveListTy, moves::Moves},
    state::{EmptyCa, FullCa, State},
    util::board_creator::*,
    values,
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
    type S = State<White, B, NoSquare, FullCa>;
    type Ms = Moves<S>;

    assert_eq!(Ms::reify().0.len(), 20);
}

#[test]
fn test_pos_2_mc() {
    type B = Board<
        //        AA  BB  CC  DD  EE  FF  GG  HH
        BoardRank<WR, __, __, __, WK, __, __, WR>, // 1
        BoardRank<WP, WP, WP, WB, WB, WP, WP, WP>, // 2
        BoardRank<__, __, WN, __, __, WQ, __, BP>, // 3
        BoardRank<__, BP, __, __, WP, __, __, __>, // 4
        BoardRank<__, __, __, WP, WN, __, __, __>, // 5
        BoardRank<BB, BN, __, __, BP, BN, BP, __>, // 6
        BoardRank<BP, __, BP, BP, BQ, BP, BB, __>, // 7
        BoardRank<BR, __, __, __, BK, __, __, BR>, // 8
    >;
    type S = State<White, B, NoSquare, FullCa>;
    type Ms = Moves<S>;

    assert_eq!(Ms::reify().0.len(), 48);
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
    type S = State<White, B, NoSquare, EmptyCa>;
    type Ms = Moves<S>;

    assert_eq!(Ms::reify().0.len(), 14);
}

#[test]
fn test_ep_mc() {
    type B = Board<
        //        AA  BB  CC  DD  EE  FF  GG  HH
        BoardRank<WR, WN, WB, WQ, WK, WB, WN, WR>, // 1
        BoardRank<WP, WP, __, WP, WP, WP, WP, WP>, // 2
        BoardRank<WP, WP, __, WP, WP, WP, WP, WP>, // 3
        BoardRank<__, __, __, __, __, __, __, __>, // 4
        BoardRank<__, BP, WP, BP, __, __, __, __>, // 5
        BoardRank<__, __, __, __, __, __, __, __>, // 6
        BoardRank<BP, __, BP, __, BP, BP, BP, BP>, // 7
        BoardRank<BR, BN, BB, BQ, BK, BB, BN, BR>, // 8
    >;
    type EP = Square<rank::R6, file::FD>;
    type S = State<White, B, SomeSquare<EP>, FullCa>;
    type Ms = Moves<S>;

    let ms = Ms::reify().0;
    assert_eq!(ms.len(), 11);

    let f = values::Square {
        rank: values::Rank::R5,
        file: values::File::C,
    };
    let t = values::Square {
        rank: values::Rank::R6,
        file: values::File::D,
    };
    let ep = values::Square {
        rank: values::Rank::R5,
        file: values::File::D,
    };
    assert!(ms.iter().find(|m| m.from == f && m.to == t).unwrap().ep == Some(ep));
}
