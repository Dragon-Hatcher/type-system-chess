use crate::{
    board_rep::{
        board::{Board, BoardRank},
        color::{Black, White},
        piece::{ColoredPiece, Pawn},
        square::{
            file,
            offset::{NoSquare, SomeSquare},
            rank, Square,
        },
    },
    move_gen::Move,
    state::{MakeMove, State, StateTy},
    util::board_creator::*,
};

#[test]
fn test_double_forward() {
    type B1 = Board<
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
    type S1 = State<White, B1, NoSquare>;

    type M =
        Move<Square<rank::R2, file::FE>, Square<rank::R4, file::FE>, ColoredPiece<Pawn, White>>;

    type B2 = Board<
        //        AA  BB  CC  DD  EE  FF  GG  HH
        BoardRank<WR, WN, WB, WQ, WK, WB, WN, WR>, // 1
        BoardRank<WP, WP, WP, WP, __, WP, WP, WP>, // 2
        BoardRank<__, __, __, __, __, __, __, __>, // 3
        BoardRank<__, __, __, __, WP, __, __, __>, // 4
        BoardRank<__, __, __, __, __, __, __, __>, // 5
        BoardRank<__, __, __, __, __, __, __, __>, // 6
        BoardRank<BP, BP, BP, BP, BP, BP, BP, BP>, // 7
        BoardRank<BR, BN, BB, BQ, BK, BB, BN, BR>, // 8
    >;
    type EP = SomeSquare<Square<rank::R3, file::FE>>;
    type S2 = State<Black, B2, EP>;

    type Applied = MakeMove<M, S1>;

    assert_eq!(Applied::reify(), S2::reify());
}

#[test]
fn test_take_ep() {
    type B1 = Board<
        //        AA  BB  CC  DD  EE  FF  GG  HH
        BoardRank<WR, WN, WB, WQ, WK, WB, WN, WR>, // 1
        BoardRank<WP, WP, WP, __, WP, WP, WP, WP>, // 2
        BoardRank<__, __, __, __, __, __, __, __>, // 3
        BoardRank<__, __, __, __, __, __, __, __>, // 4
        BoardRank<__, __, __, WP, BP, __, __, __>, // 5
        BoardRank<__, __, __, __, __, __, __, __>, // 6
        BoardRank<BP, BP, BP, BP, __, BP, BP, BP>, // 7
        BoardRank<BR, BN, BB, BQ, BK, BB, BN, BR>, // 8
    >;
    type EP = SomeSquare<Square<rank::R6, file::FE>>;
    type S1 = State<White, B1, EP>;

    type EPDel = SomeSquare<Square<rank::R5, file::FE>>;
    type M = Move<
        Square<rank::R5, file::FD>,
        Square<rank::R6, file::FE>,
        ColoredPiece<Pawn, White>,
        EPDel,
    >;

    type B2 = Board<
        //        AA  BB  CC  DD  EE  FF  GG  HH
        BoardRank<WR, WN, WB, WQ, WK, WB, WN, WR>, // 1
        BoardRank<WP, WP, WP, __, WP, WP, WP, WP>, // 2
        BoardRank<__, __, __, __, __, __, __, __>, // 3
        BoardRank<__, __, __, __, __, __, __, __>, // 4
        BoardRank<__, __, __, __, __, __, __, __>, // 5
        BoardRank<__, __, __, __, WP, __, __, __>, // 6
        BoardRank<BP, BP, BP, BP, __, BP, BP, BP>, // 7
        BoardRank<BR, BN, BB, BQ, BK, BB, BN, BR>, // 8
    >;
    type S2 = State<Black, B2, NoSquare>;

    type Applied = MakeMove<M, S1>;

    assert_eq!(Applied::reify(), S2::reify());
}
