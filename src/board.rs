use core::panic;
use std::{
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
};

use colored::Colorize;

use crate::{
    bitboard::{Bitboard, EMPTY_BB},
    square::{Color, File, Piece, PieceKind, Rank, Square},
};

type BoardPieces = [Piece; 64];

pub struct Board {
    white_pawns: Bitboard,
    white_knights: Bitboard,
    white_bishops: Bitboard,
    white_rooks: Bitboard,
    white_queens: Bitboard,
    white_king: Bitboard,

    black_pawns: Bitboard,
    black_knights: Bitboard,
    black_bishops: Bitboard,
    black_rooks: Bitboard,
    black_queens: Bitboard,
    black_king: Bitboard,

    pieces: BoardPieces,
}

impl Index<Square> for BoardPieces {
    type Output = Piece;

    fn index(&self, square: Square) -> &Self::Output {
        &self[square.index()]
    }
}

impl IndexMut<Square> for BoardPieces {
    fn index_mut(&mut self, square: Square) -> &mut Self::Output {
        &mut self[square.index()]
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            white_pawns: EMPTY_BB,
            white_knights: EMPTY_BB,
            white_bishops: EMPTY_BB,
            white_rooks: EMPTY_BB,
            white_queens: EMPTY_BB,
            white_king: EMPTY_BB,

            black_pawns: EMPTY_BB,
            black_knights: EMPTY_BB,
            black_bishops: EMPTY_BB,
            black_rooks: EMPTY_BB,
            black_queens: EMPTY_BB,
            black_king: EMPTY_BB,

            pieces: [Piece::default(); 64],
        }
    }
}

impl Board {
    pub fn get_piece_bb(&mut self, piece: Piece) -> &mut Bitboard {
        match (piece.color, piece.kind) {
            (Color::White, PieceKind::Pawn) => &mut self.white_pawns,
            (Color::White, PieceKind::Knight) => &mut self.white_knights,
            (Color::White, PieceKind::Bishop) => &mut self.white_bishops,
            (Color::White, PieceKind::Rook) => &mut self.white_rooks,
            (Color::White, PieceKind::Queen) => &mut self.white_queens,
            (Color::White, PieceKind::King) => &mut self.white_king,
            (Color::Black, PieceKind::Pawn) => &mut self.black_pawns,
            (Color::Black, PieceKind::Knight) => &mut self.black_knights,
            (Color::Black, PieceKind::Bishop) => &mut self.black_bishops,
            (Color::Black, PieceKind::Rook) => &mut self.black_rooks,
            (Color::Black, PieceKind::Queen) => &mut self.black_queens,
            (Color::Black, PieceKind::King) => &mut self.black_king,
            _ => panic!("tried to retrieve bitboard for empty piece"),
        }
    }

    pub fn add_piece(&mut self, piece: Piece, square: Square) {
        self.get_piece_bb(piece).set_bit(square);
        self.pieces[square] = piece;
    }

    pub fn get_piece(&self, square: Square) -> Piece {
        self.pieces[square]
    }
}

fn print_board(board: &Board, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for &rank in Rank::EVERY.iter().rev() {
        for file in File::EVERY {
            if file == File::A {
                write!(f, "  {}", (rank + 1).to_string().cyan())?;
            }

            let square = Square::new(rank, file);

            let piece = board.get_piece(square);

            write!(f, "  {}", piece)?;
        }

        writeln!(f)?;
    }

    write!(f, "   ")?;

    for file in File::EVERY {
        write!(f, "  {}", file.to_string().cyan())?;
    }

    Ok(())
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        print_board(self, f)
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        print_board(self, f)
    }
}
