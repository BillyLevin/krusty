use std::{
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
};

use anyhow::{bail, Context};
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
    pub fn get_piece_bb(&mut self, piece: Piece) -> Option<&mut Bitboard> {
        match (piece.color, piece.kind) {
            (Color::White, PieceKind::Pawn) => Some(&mut self.white_pawns),
            (Color::White, PieceKind::Knight) => Some(&mut self.white_knights),
            (Color::White, PieceKind::Bishop) => Some(&mut self.white_bishops),
            (Color::White, PieceKind::Rook) => Some(&mut self.white_rooks),
            (Color::White, PieceKind::Queen) => Some(&mut self.white_queens),
            (Color::White, PieceKind::King) => Some(&mut self.white_king),
            (Color::Black, PieceKind::Pawn) => Some(&mut self.black_pawns),
            (Color::Black, PieceKind::Knight) => Some(&mut self.black_knights),
            (Color::Black, PieceKind::Bishop) => Some(&mut self.black_bishops),
            (Color::Black, PieceKind::Rook) => Some(&mut self.black_rooks),
            (Color::Black, PieceKind::Queen) => Some(&mut self.black_queens),
            (Color::Black, PieceKind::King) => Some(&mut self.black_king),
            _ => None,
        }
    }

    pub fn add_piece(&mut self, piece: Piece, square: Square) {
        if let Some(bitboard) = self.get_piece_bb(piece) {
            bitboard.set_bit(square);
        }

        self.pieces[square] = piece;
    }

    pub fn get_piece(&self, square: Square) -> Piece {
        self.pieces[square]
    }

    pub fn parse_fen(&mut self, fen: &str) -> anyhow::Result<()> {
        let fields: Vec<&str> = fen.split(' ').collect();

        if fields.len() != 6 {
            bail!(
                "FEN has invalid number of fields. Expected 6 but got {}",
                fields.len()
            );
        }

        let piece_placement = fields.first().unwrap();

        // we reverse because the FEN starts at the 8th rank. it's a bit easier to understand if we
        // start with the 1st rank
        let ranks: Vec<(_, _)> = piece_placement.split('/').rev().enumerate().collect();

        if ranks.len() != 8 {
            bail!(
                "FEN has invalid number of ranks. Expected 8 but got {}",
                ranks.len()
            );
        }

        for (rank_index, rank_fen) in ranks {
            let rank: Rank = rank_index.try_into()?;
            let mut file_index = 0;

            for ch in rank_fen.chars() {
                match ch {
                    'p' | 'n' | 'b' | 'r' | 'q' | 'k' | 'P' | 'N' | 'B' | 'R' | 'Q' | 'K' => {
                        let piece: Piece = ch.try_into()?;
                        let square = Square::new(rank, file_index.try_into()?);
                        self.add_piece(piece, square);
                        file_index += 1;
                    }

                    '1'..='8' => {
                        let empty_count = ch.to_digit(10).context("Can't convert to digit")?;

                        for _ in 1..=empty_count {
                            let piece = Piece::default();
                            let square = Square::new(rank, file_index.try_into()?);
                            self.add_piece(piece, square);
                            file_index += 1;
                        }
                    }
                    _ => {}
                }
            }
        }

        // let color = fields.get(1).unwrap();
        // let castling_rights = fields.get(2).unwrap();
        // let en_passant_square = fields.get(3).unwrap();
        // let halfmove_clock = fields.get(4).unwrap();
        // let move_count = fields.get(5).unwrap();

        Ok(())
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
