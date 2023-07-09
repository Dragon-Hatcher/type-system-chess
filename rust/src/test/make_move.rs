use crate::{
    state::{CastleState, FullCa},
    ui::prelude::*,
    util::{False, True}, values,
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
    type S1 = State<White, B1, NoSquare, FullCa>;

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
    type EP = SomeSquare<E3>;
    type S2 = State<Black, B2, EP, FullCa>;

    type Applied = MakeEM<S1, E2, E4>;

    assert_eq!(Applied::reify(), S2::reify());
}

#[test]
fn test_castle() {
    type B1 = Board<
        //        AA  BB  CC  DD  EE  FF  GG  HH
        BoardRank<WR, WN, WB, WQ, WK, __, __, WR>, // 1
        BoardRank<WP, WP, WP, WP, __, WP, WP, WP>, // 2
        BoardRank<__, __, __, WB, __, WN, __, __>, // 3
        BoardRank<__, __, __, __, WP, __, __, __>, // 4
        BoardRank<__, __, __, __, BP, __, __, __>, // 5
        BoardRank<__, __, __, BP, __, BN, __, __>, // 6
        BoardRank<BP, BP, BP, __, __, BP, BP, BP>, // 7
        BoardRank<BR, BN, BB, BQ, BK, BB, __, BR>, // 8
    >;
    type S1 = State<White, B1, NoSquare, FullCa>;

    type B2 = Board<
        //        AA  BB  CC  DD  EE  FF  GG  HH
        BoardRank<WR, WN, WB, WQ, __, WR, WK, __>, // 1
        BoardRank<WP, WP, WP, WP, __, WP, WP, WP>, // 2
        BoardRank<__, __, __, WB, __, WN, __, __>, // 3
        BoardRank<__, __, __, __, WP, __, __, __>, // 4
        BoardRank<__, __, __, __, BP, __, __, __>, // 5
        BoardRank<__, __, __, BP, __, BN, __, __>, // 6
        BoardRank<BP, BP, BP, __, __, BP, BP, BP>, // 7
        BoardRank<BR, BN, BB, BQ, BK, BB, __, BR>, // 8
    >;
    type S2 = State<Black, B2, NoSquare, CastleState<False, False, True, True>>;

    type Applied = MakeEM<S1, E1, G1>;

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
    type S1 = State<White, B1, SomeSquare<E6>, FullCa>;

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
    type S2 = State<Black, B2, NoSquare, FullCa>;

    type Applied = MakeEM<S1, D5, E6>;

    assert_eq!(Applied::reify(), S2::reify());
}

#[test]
fn test_lose_castling() {
    type B1 = Board<
        //        AA  BB  CC  DD  EE  FF  GG  HH
        BoardRank<WR, __, __, __, WK, __, __, WR>, // 1
        BoardRank<__, __, __, __, __, __, __, __>, // 2
        BoardRank<__, __, __, __, __, __, __, __>, // 3
        BoardRank<__, __, __, __, __, __, __, __>, // 4
        BoardRank<__, __, __, __, __, __, __, __>, // 5
        BoardRank<__, __, __, __, __, __, __, __>, // 6
        BoardRank<__, __, __, __, __, __, __, __>, // 7
        BoardRank<BR, __, __, __, BK, __, __, BR>, // 8
    >;
    type S1 = State<White, B1, NoSquare, FullCa>;

    {
        type Applied = MakeEM<S1, E1, E2>;

        assert_eq!(
            Applied::reify().unwrap_state().castle_state,
            values::CastleState {
                wk: false,
                wq: false,
                bk: true,
                bq: true
            }
        );
    }

    {
        type Applied = MakeEM<S1, A1, A8>;

        assert_eq!(
            Applied::reify().unwrap_state().castle_state,
            values::CastleState {
                wk: true,
                wq: false,
                bk: true,
                bq: false
            }
        );
    }

    {
        type Applied = MakeEM<S1, H1, H7>;

        assert_eq!(
            Applied::reify().unwrap_state().castle_state,
            values::CastleState {
                wk: false,
                wq: true,
                bk: true,
                bq: true
            }
        );
    }
}
