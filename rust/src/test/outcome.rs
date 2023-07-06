use crate::{
    board_rep::{
        board::{Board, BoardRank},
        color::{Black, White},
    },
    state::{
        outcome::{Outcome, OutcomeEn},
        State,
    },
    util::board_creator::*,
    values,
};

#[test]
fn test_ongoing() {
    type B = Board<
        //        AA  BB  CC  DD  EE  FF  GG  HH
        BoardRank<WR, WN, WB, WQ, WK, WB, WN, WR>, // 1
        BoardRank<WP, WP, WP, WP, __, WP, WP, WP>, // 2
        BoardRank<__, __, __, __, WP, __, __, __>, // 3
        BoardRank<__, __, __, __, __, __, __, __>, // 4
        BoardRank<__, __, __, __, __, __, BP, __>, // 5
        BoardRank<__, __, __, __, __, BP, __, __>, // 6
        BoardRank<BP, BP, BP, BP, BP, __, __, BP>, // 7
        BoardRank<BR, BN, BB, BQ, BK, BB, BN, BR>, // 8
    >;
    type S = State<White, B>;
    type O = Outcome<S>;

    assert_eq!(O::reify(), values::Outcome::Ongoing);
}

#[test]
fn test_stalemate() {
    type B = Board<
        //        AA  BB  CC  DD  EE  FF  GG  HH
        BoardRank<__, __, __, __, __, __, __, __>, // 1
        BoardRank<__, __, __, __, __, __, __, __>, // 2
        BoardRank<__, __, __, __, __, __, __, __>, // 3
        BoardRank<__, __, __, __, __, __, __, __>, // 4
        BoardRank<__, __, __, __, __, __, __, __>, // 5
        BoardRank<__, WQ, __, __, __, __, __, __>, // 6
        BoardRank<__, __, __, __, __, __, __, __>, // 7
        BoardRank<BK, __, __, __, __, __, __, __>, // 8
    >;
    type S = State<Black, B>;
    type O = Outcome<S>;

    assert_eq!(O::reify(), values::Outcome::Draw);
}

#[test]
fn test_checkmate() {
    type B = Board<
        //        AA  BB  CC  DD  EE  FF  GG  HH
        BoardRank<WR, WN, WB, __, WK, WB, WN, WR>, // 1
        BoardRank<WP, WP, WP, WP, __, WP, WP, WP>, // 2
        BoardRank<__, __, __, __, WP, __, __, __>, // 3
        BoardRank<__, __, __, __, __, __, __, __>, // 4
        BoardRank<__, __, __, __, __, __, BP, WQ>, // 5
        BoardRank<__, __, __, __, __, BP, __, __>, // 6
        BoardRank<BP, BP, BP, BP, BP, __, __, BP>, // 7
        BoardRank<BR, BN, BB, BQ, BK, BB, BN, BR>, // 8
    >;
    type S = State<Black, B>;
    type O = Outcome<S>;

    assert_eq!(O::reify(), values::Outcome::Checkmate(values::Color::White));
}
