#![allow(unused)]

use std::fmt::Debug;

#[derive(Debug)]
pub(crate) enum Color {
    White,
    Black,
}

#[derive(Debug)]
pub(crate) enum Piece {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

#[derive(Debug)]
pub(crate) struct ColoredPiece {
    pub(crate) piece: Piece,
    pub(crate) color: Color
}

pub(crate) enum Rank {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}
impl Debug for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::R1 => write!(f, "1"),
            Self::R2 => write!(f, "2"),
            Self::R3 => write!(f, "3"),
            Self::R4 => write!(f, "4"),
            Self::R5 => write!(f, "5"),
            Self::R6 => write!(f, "6"),
            Self::R7 => write!(f, "7"),
            Self::R8 => write!(f, "8"),
        }
    }
}

#[derive(Debug)]
pub(crate) enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

pub(crate) struct Square {
    pub(crate) rank: Rank,
    pub(crate) file: File,
}
impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}", self.file, self.rank)
    }
}

#[derive(Debug)]
pub(crate) enum Cell {
    Empty,
    Filled(ColoredPiece)
}

#[derive(Debug)]
pub(crate) struct BoardRank {
    pub(crate) a: Cell,
    pub(crate) b: Cell,
    pub(crate) c: Cell,
    pub(crate) d: Cell,
    pub(crate) e: Cell,
    pub(crate) f: Cell,
    pub(crate) g: Cell,
    pub(crate) h: Cell,
}

#[derive(Debug)]
pub(crate) struct Board {
    pub(crate) r1: BoardRank,
    pub(crate) r2: BoardRank,
    pub(crate) r3: BoardRank,
    pub(crate) r4: BoardRank,
    pub(crate) r5: BoardRank,
    pub(crate) r6: BoardRank,
    pub(crate) r7: BoardRank,
    pub(crate) r8: BoardRank,
}

#[derive(Debug)]
pub(crate) struct State {
    pub(crate) to_move: Color,
    pub(crate) pieces: Board,
}

#[derive(Debug)]
pub(crate) struct Move {
    pub(crate) from: Square,
    pub(crate) to: Square,
}

#[derive(Debug)]
pub(crate) struct Offset {
    pub(crate) rank: i8,
    pub(crate) file: i8,
}
