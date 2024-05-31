use core::panic;
use std::{
    fmt::{Debug, Display},
    ops::{Add, Mul},
};

use anyhow::{bail, Context};

use crate::{
    bitboard::{Bitboard, EMPTY_BB},
    board::Side,
};

macro_rules! define_squares {
    ($($square_name:ident),*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Square {
            $($square_name),*, None
        }

        impl Square {
            pub fn new(rank: Rank, file: File) -> Self {
                let index = (rank * 8) + file;

                match index {
                    $(x if x == (Square::$square_name as u8) => Square::$square_name,)*
                    _ => panic!("out of range!"),
                }
            }

        }

        impl From<u32> for Square {
            fn from(value: u32) -> Self {
                match value {
                    $(x if x == (Square::$square_name as u32) => Square::$square_name,)*
                    _ => panic!("out of range!"),
                }
            }
        }

        impl From<usize> for Square {
            fn from(value: usize) -> Self {
                match value {
                    $(x if x == (Square::$square_name as usize) => Square::$square_name,)*
                    _ => panic!("out of range!"),
                }
            }
        }
    };
}

#[rustfmt::skip]
define_squares! (
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8 
);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

#[derive(PartialEq, Eq)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    pub color: PieceColor,
    pub kind: PieceKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    NoPiece,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceColor {
    White,
    Black,
    None,
}

impl From<Side> for PieceColor {
    fn from(value: Side) -> Self {
        match value {
            Side::White => PieceColor::White,
            Side::Black => PieceColor::Black,
        }
    }
}

const fn init_square_bitboards() -> [Bitboard; 64] {
    let mut bitboards: [Bitboard; 64] = [EMPTY_BB; 64];

    let mut square = 0;

    while square < 64 {
        bitboards[square] = Bitboard(1u64 << square);
        square += 1;
    }

    bitboards
}

impl Square {
    const SQUARE_BB: [Bitboard; 64] = init_square_bitboards();

    pub fn index(&self) -> usize {
        match self {
            Square::None => panic!("invalid square"),
            _ => *self as usize,
        }
    }

    pub fn bitboard(self) -> Bitboard {
        let square = self as usize;

        if square > 63 {
            panic!("attempted to get bitboard for non-existent square!");
        }

        Self::SQUARE_BB[square]
    }

    pub fn rank(&self) -> anyhow::Result<Rank> {
        self.try_into()
    }

    pub fn north(&self) -> Self {
        (self.index() + 8).into()
    }

    pub fn south(&self) -> Self {
        (self.index() - 8).into()
    }

    pub fn distance_between(&self, other_square: Square) -> u32 {
        let index1 = self.index() as i32;
        let index2 = other_square.index() as i32;

        (index1).abs_diff(index2)
    }
}

impl Rank {
    pub const EVERY: [Self; 8] = [
        Self::First,
        Self::Second,
        Self::Third,
        Self::Fourth,
        Self::Fifth,
        Self::Sixth,
        Self::Seventh,
        Self::Eighth,
    ];
}

impl TryFrom<usize> for Rank {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::First),
            1 => Ok(Self::Second),
            2 => Ok(Self::Third),
            3 => Ok(Self::Fourth),
            4 => Ok(Self::Fifth),
            5 => Ok(Self::Sixth),
            6 => Ok(Self::Seventh),
            7 => Ok(Self::Eighth),

            _ => bail!("Invalid rank. Should be between 0 and 7 but got {}", value),
        }
    }
}

impl TryFrom<&Square> for Rank {
    type Error = anyhow::Error;
    fn try_from(square: &Square) -> Result<Self, Self::Error> {
        match square {
            Square::A1
            | Square::B1
            | Square::C1
            | Square::D1
            | Square::E1
            | Square::F1
            | Square::G1
            | Square::H1 => Ok(Self::First),

            Square::A2
            | Square::B2
            | Square::C2
            | Square::D2
            | Square::E2
            | Square::F2
            | Square::G2
            | Square::H2 => Ok(Self::Second),

            Square::A3
            | Square::B3
            | Square::C3
            | Square::D3
            | Square::E3
            | Square::F3
            | Square::G3
            | Square::H3 => Ok(Self::Third),

            Square::A4
            | Square::B4
            | Square::C4
            | Square::D4
            | Square::E4
            | Square::F4
            | Square::G4
            | Square::H4 => Ok(Self::Fourth),

            Square::A5
            | Square::B5
            | Square::C5
            | Square::D5
            | Square::E5
            | Square::F5
            | Square::G5
            | Square::H5 => Ok(Self::Fifth),

            Square::A6
            | Square::B6
            | Square::C6
            | Square::D6
            | Square::E6
            | Square::F6
            | Square::G6
            | Square::H6 => Ok(Self::Sixth),

            Square::A7
            | Square::B7
            | Square::C7
            | Square::D7
            | Square::E7
            | Square::F7
            | Square::G7
            | Square::H7 => Ok(Self::Seventh),

            Square::A8
            | Square::B8
            | Square::C8
            | Square::D8
            | Square::E8
            | Square::F8
            | Square::G8
            | Square::H8 => Ok(Self::Eighth),

            Square::None => bail!("cannot get rank of non-existent square"),
        }
    }
}

impl Add<u8> for Rank {
    type Output = u8;

    fn add(self, rhs: u8) -> Self::Output {
        (self as u8) + rhs
    }
}

impl Mul<u8> for Rank {
    type Output = u8;

    fn mul(self, rhs: u8) -> Self::Output {
        (self as u8) * rhs
    }
}

impl TryFrom<char> for Rank {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let digit: usize = value
            .to_digit(10)
            .context("Rank should be a digit")?
            .try_into()?;

        (digit - 1).try_into()
    }
}

impl File {
    pub const EVERY: [Self; 8] = [
        Self::A,
        Self::B,
        Self::C,
        Self::D,
        Self::E,
        Self::F,
        Self::G,
        Self::H,
    ];
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            File::A => write!(f, "a"),
            File::B => write!(f, "b"),
            File::C => write!(f, "c"),
            File::D => write!(f, "d"),
            File::E => write!(f, "e"),
            File::F => write!(f, "f"),
            File::G => write!(f, "g"),
            File::H => write!(f, "h"),
        }
    }
}

impl Add<u8> for File {
    type Output = u8;

    fn add(self, rhs: u8) -> Self::Output {
        (self as u8) + rhs
    }
}

impl Add<File> for u8 {
    type Output = u8;

    fn add(self, rhs: File) -> Self::Output {
        self + (rhs as u8)
    }
}

impl TryFrom<usize> for File {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::A),
            1 => Ok(Self::B),
            2 => Ok(Self::C),
            3 => Ok(Self::D),
            4 => Ok(Self::E),
            5 => Ok(Self::F),
            6 => Ok(Self::G),
            7 => Ok(Self::H),

            _ => bail!("Invalid file. Should be between 0 and 7 but got {}", value),
        }
    }
}

impl TryFrom<char> for File {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a' => Ok(Self::A),
            'b' => Ok(Self::B),
            'c' => Ok(Self::C),
            'd' => Ok(Self::D),
            'e' => Ok(Self::E),
            'f' => Ok(Self::F),
            'g' => Ok(Self::G),
            'h' => Ok(Self::H),
            _ => bail!("Invalid file. Should be a lowercase letter between a and h"),
        }
    }
}

impl Piece {
    pub fn new(color: PieceColor, kind: PieceKind) -> Self {
        Self { color, kind }
    }
}

impl Default for Piece {
    fn default() -> Self {
        Self {
            color: PieceColor::None,
            kind: PieceKind::NoPiece,
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let unicode = match (self.color, self.kind) {
            (PieceColor::White, PieceKind::Pawn) => 0x2659,
            (PieceColor::White, PieceKind::Knight) => 0x2658,
            (PieceColor::White, PieceKind::Bishop) => 0x2657,
            (PieceColor::White, PieceKind::Rook) => 0x2656,
            (PieceColor::White, PieceKind::Queen) => 0x2655,
            (PieceColor::White, PieceKind::King) => 0x2654,
            (PieceColor::Black, PieceKind::Pawn) => 0x265F,
            (PieceColor::Black, PieceKind::Knight) => 0x265E,
            (PieceColor::Black, PieceKind::Bishop) => 0x265D,
            (PieceColor::Black, PieceKind::Rook) => 0x265C,
            (PieceColor::Black, PieceKind::Queen) => 0x265B,
            (PieceColor::Black, PieceKind::King) => 0x265A,
            _ => 0x0030,
        };

        write!(f, "{}", char::from_u32(unicode).unwrap_or('?'))
    }
}

impl TryFrom<char> for Piece {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            // black pieces
            'p' => Ok(Self::new(PieceColor::Black, PieceKind::Pawn)),
            'n' => Ok(Self::new(PieceColor::Black, PieceKind::Knight)),
            'b' => Ok(Self::new(PieceColor::Black, PieceKind::Bishop)),
            'r' => Ok(Self::new(PieceColor::Black, PieceKind::Rook)),
            'q' => Ok(Self::new(PieceColor::Black, PieceKind::Queen)),
            'k' => Ok(Self::new(PieceColor::Black, PieceKind::King)),

            // white pieces
            'P' => Ok(Self::new(PieceColor::White, PieceKind::Pawn)),
            'N' => Ok(Self::new(PieceColor::White, PieceKind::Knight)),
            'B' => Ok(Self::new(PieceColor::White, PieceKind::Bishop)),
            'R' => Ok(Self::new(PieceColor::White, PieceKind::Rook)),
            'Q' => Ok(Self::new(PieceColor::White, PieceKind::Queen)),
            'K' => Ok(Self::new(PieceColor::White, PieceKind::King)),

            _ => bail!("Character {} could not be converted to a piece", value),
        }
    }
}
