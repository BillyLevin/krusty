use core::panic;
use std::{
    fmt::{Debug, Display},
    ops::{Add, Mul},
};

use anyhow::bail;

use crate::bitboard::Bitboard;

#[rustfmt::skip]
#[derive(Clone, Copy)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8, None
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceKind,
}

#[derive(Debug, Clone, Copy)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    NoPiece,
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    White,
    Black,
    None,
}

const fn init_square_bitboards() -> [Bitboard; 64] {
    let mut bitboards: [Bitboard; 64] = [Bitboard(0u64); 64];

    let mut square = 0;

    while square < 64 {
        bitboards[square] = Bitboard(1u64 << square);
        square += 1;
    }

    bitboards
}

impl Square {
    const SQUARE_BB: [Bitboard; 64] = init_square_bitboards();

    pub fn new(rank: Rank, file: File) -> Self {
        let index = (rank * 8) + file;

        match index {
            x if x == (Square::A1 as u8) => Square::A1,
            x if x == (Square::B1 as u8) => Square::B1,
            x if x == (Square::C1 as u8) => Square::C1,
            x if x == (Square::D1 as u8) => Square::D1,
            x if x == (Square::E1 as u8) => Square::E1,
            x if x == (Square::F1 as u8) => Square::F1,
            x if x == (Square::G1 as u8) => Square::G1,
            x if x == (Square::H1 as u8) => Square::H1,
            x if x == (Square::A2 as u8) => Square::A2,
            x if x == (Square::B2 as u8) => Square::B2,
            x if x == (Square::C2 as u8) => Square::C2,
            x if x == (Square::D2 as u8) => Square::D2,
            x if x == (Square::E2 as u8) => Square::E2,
            x if x == (Square::F2 as u8) => Square::F2,
            x if x == (Square::G2 as u8) => Square::G2,
            x if x == (Square::H2 as u8) => Square::H2,
            x if x == (Square::A3 as u8) => Square::A3,
            x if x == (Square::B3 as u8) => Square::B3,
            x if x == (Square::C3 as u8) => Square::C3,
            x if x == (Square::D3 as u8) => Square::D3,
            x if x == (Square::E3 as u8) => Square::E3,
            x if x == (Square::F3 as u8) => Square::F3,
            x if x == (Square::G3 as u8) => Square::G3,
            x if x == (Square::H3 as u8) => Square::H3,
            x if x == (Square::A4 as u8) => Square::A4,
            x if x == (Square::B4 as u8) => Square::B4,
            x if x == (Square::C4 as u8) => Square::C4,
            x if x == (Square::D4 as u8) => Square::D4,
            x if x == (Square::E4 as u8) => Square::E4,
            x if x == (Square::F4 as u8) => Square::F4,
            x if x == (Square::G4 as u8) => Square::G4,
            x if x == (Square::H4 as u8) => Square::H4,
            x if x == (Square::A5 as u8) => Square::A5,
            x if x == (Square::B5 as u8) => Square::B5,
            x if x == (Square::C5 as u8) => Square::C5,
            x if x == (Square::D5 as u8) => Square::D5,
            x if x == (Square::E5 as u8) => Square::E5,
            x if x == (Square::F5 as u8) => Square::F5,
            x if x == (Square::G5 as u8) => Square::G5,
            x if x == (Square::H5 as u8) => Square::H5,
            x if x == (Square::A6 as u8) => Square::A6,
            x if x == (Square::B6 as u8) => Square::B6,
            x if x == (Square::C6 as u8) => Square::C6,
            x if x == (Square::D6 as u8) => Square::D6,
            x if x == (Square::E6 as u8) => Square::E6,
            x if x == (Square::F6 as u8) => Square::F6,
            x if x == (Square::G6 as u8) => Square::G6,
            x if x == (Square::H6 as u8) => Square::H6,
            x if x == (Square::A7 as u8) => Square::A7,
            x if x == (Square::B7 as u8) => Square::B7,
            x if x == (Square::C7 as u8) => Square::C7,
            x if x == (Square::D7 as u8) => Square::D7,
            x if x == (Square::E7 as u8) => Square::E7,
            x if x == (Square::F7 as u8) => Square::F7,
            x if x == (Square::G7 as u8) => Square::G7,
            x if x == (Square::H7 as u8) => Square::H7,
            x if x == (Square::A8 as u8) => Square::A8,
            x if x == (Square::B8 as u8) => Square::B8,
            x if x == (Square::C8 as u8) => Square::C8,
            x if x == (Square::D8 as u8) => Square::D8,
            x if x == (Square::E8 as u8) => Square::E8,
            x if x == (Square::F8 as u8) => Square::F8,
            x if x == (Square::G8 as u8) => Square::G8,
            x if x == (Square::H8 as u8) => Square::H8,
            _ => panic!("out of range!"),
        }
    }

    pub fn index(&self) -> usize {
        match self {
            Square::A1 => 0,
            Square::B1 => 1,
            Square::C1 => 2,
            Square::D1 => 3,
            Square::E1 => 4,
            Square::F1 => 5,
            Square::G1 => 6,
            Square::H1 => 7,
            Square::A2 => 8,
            Square::B2 => 9,
            Square::C2 => 10,
            Square::D2 => 11,
            Square::E2 => 12,
            Square::F2 => 13,
            Square::G2 => 14,
            Square::H2 => 15,
            Square::A3 => 16,
            Square::B3 => 17,
            Square::C3 => 18,
            Square::D3 => 19,
            Square::E3 => 20,
            Square::F3 => 21,
            Square::G3 => 22,
            Square::H3 => 23,
            Square::A4 => 24,
            Square::B4 => 25,
            Square::C4 => 26,
            Square::D4 => 27,
            Square::E4 => 28,
            Square::F4 => 29,
            Square::G4 => 30,
            Square::H4 => 31,
            Square::A5 => 32,
            Square::B5 => 33,
            Square::C5 => 34,
            Square::D5 => 35,
            Square::E5 => 36,
            Square::F5 => 37,
            Square::G5 => 38,
            Square::H5 => 39,
            Square::A6 => 40,
            Square::B6 => 41,
            Square::C6 => 42,
            Square::D6 => 43,
            Square::E6 => 44,
            Square::F6 => 45,
            Square::G6 => 46,
            Square::H6 => 47,
            Square::A7 => 48,
            Square::B7 => 49,
            Square::C7 => 50,
            Square::D7 => 51,
            Square::E7 => 52,
            Square::F7 => 53,
            Square::G7 => 54,
            Square::H7 => 55,
            Square::A8 => 56,
            Square::B8 => 57,
            Square::C8 => 58,
            Square::D8 => 59,
            Square::E8 => 60,
            Square::F8 => 61,
            Square::G8 => 62,
            Square::H8 => 63,
            Square::None => panic!("invalid square"),
        }
    }

    pub fn bitboard(self) -> Bitboard {
        let square = self as usize;

        if square > 63 {
            panic!("attempted to get bitboard for non-existent square!");
        }

        Self::SQUARE_BB[square]
    }
}

impl std::ops::Shl<Square> for u64 {
    type Output = u64;

    fn shl(self, rhs: Square) -> Self::Output {
        self << (rhs as u8)
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

impl Piece {
    pub fn new(color: Color, kind: PieceKind) -> Self {
        Self { color, kind }
    }
}

impl Default for Piece {
    fn default() -> Self {
        Self {
            color: Color::None,
            kind: PieceKind::NoPiece,
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let unicode = match (self.color, self.kind) {
            (Color::White, PieceKind::Pawn) => 0x2659,
            (Color::White, PieceKind::Knight) => 0x2658,
            (Color::White, PieceKind::Bishop) => 0x2657,
            (Color::White, PieceKind::Rook) => 0x2656,
            (Color::White, PieceKind::Queen) => 0x2655,
            (Color::White, PieceKind::King) => 0x2654,
            (Color::Black, PieceKind::Pawn) => 0x265F,
            (Color::Black, PieceKind::Knight) => 0x265E,
            (Color::Black, PieceKind::Bishop) => 0x265D,
            (Color::Black, PieceKind::Rook) => 0x265C,
            (Color::Black, PieceKind::Queen) => 0x265B,
            (Color::Black, PieceKind::King) => 0x265A,
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
            'p' => Ok(Self::new(Color::Black, PieceKind::Pawn)),
            'n' => Ok(Self::new(Color::Black, PieceKind::Knight)),
            'b' => Ok(Self::new(Color::Black, PieceKind::Bishop)),
            'r' => Ok(Self::new(Color::Black, PieceKind::Rook)),
            'q' => Ok(Self::new(Color::Black, PieceKind::Queen)),
            'k' => Ok(Self::new(Color::Black, PieceKind::King)),

            // white pieces
            'P' => Ok(Self::new(Color::White, PieceKind::Pawn)),
            'N' => Ok(Self::new(Color::White, PieceKind::Knight)),
            'B' => Ok(Self::new(Color::White, PieceKind::Bishop)),
            'R' => Ok(Self::new(Color::White, PieceKind::Rook)),
            'Q' => Ok(Self::new(Color::White, PieceKind::Queen)),
            'K' => Ok(Self::new(Color::White, PieceKind::King)),

            _ => bail!("Character {} could not be converted to a piece", value),
        }
    }
}
