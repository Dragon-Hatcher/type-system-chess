use crate::{
    board_rep::{
        board::{Board, BoardRank},
        color::{Black, White},
        piece::{ColoredPiece, King, Pawn, Rook},
        square::{
            file,
            offset::{NoSquare, SomeSquare},
            rank, Square,
        },
    },
    move_gen::Move,
    state::{CastleState, FullCa, MakeMove, State, StateTy},
    util::{board_creator::*, False, True},
    values,
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
    type S2 = State<Black, B2, EP, FullCa>;

    type Applied = MakeMove<M, S1>;

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

    type M = Move<
        Square<rank::R1, file::FE>,
        Square<rank::R1, file::FG>,
        ColoredPiece<King, White>,
        NoSquare,
        SomeSquare<Square<rank::R1, file::FH>>,
        SomeSquare<Square<rank::R1, file::FF>>,
    >;

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
    type S1 = State<White, B1, EP, FullCa>;

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
    type S2 = State<Black, B2, NoSquare, FullCa>;

    type Applied = MakeMove<M, S1>;

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
        type M =
            Move<Square<rank::R1, file::FE>, Square<rank::R2, file::FE>, ColoredPiece<King, White>>;
        type Applied = MakeMove<M, S1>;

        assert_eq!(
            Applied::reify().castle_state,
            values::CastleState {
                wk: false,
                wq: false,
                bk: true,
                bq: true
            }
        );
    }

    {
        type M =
            Move<Square<rank::R1, file::FA>, Square<rank::R8, file::FA>, ColoredPiece<Rook, White>>;
        type Applied = MakeMove<M, S1>;

        assert_eq!(
            Applied::reify().castle_state,
            values::CastleState {
                wk: true,
                wq: false,
                bk: true,
                bq: false
            }
        );
    }

    {
        type M =
            Move<Square<rank::R1, file::FH>, Square<rank::R7, file::FH>, ColoredPiece<Rook, White>>;
        type Applied = MakeMove<M, S1>;

        assert_eq!(
            Applied::reify().castle_state,
            values::CastleState {
                wk: false,
                wq: true,
                bk: true,
                bq: true
            }
        );
    }
}
