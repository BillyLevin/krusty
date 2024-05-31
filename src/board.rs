use std::{
    fmt::{Debug, Display},
    ops::{Index, IndexMut, Not},
};

use anyhow::{bail, Context};
use colored::Colorize;

use crate::{
    bitboard::{Bitboard, EMPTY_BB},
    move_generator::{MoveGenerator, MoveList},
    square::{File, Piece, PieceColor, PieceKind, Rank, Square},
    zobrist_hash::{ZobristHasher, ZobristKey},
};

type BoardPieces = [Piece; 64];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    White,
    Black,
}

impl TryFrom<PieceColor> for Side {
    type Error = anyhow::Error;

    fn try_from(value: PieceColor) -> Result<Self, Self::Error> {
        match value {
            PieceColor::White => Ok(Side::White),
            PieceColor::Black => Ok(Side::Black),
            PieceColor::None => bail!("piece has no color"),
        }
    }
}

impl Not for Side {
    type Output = Side;

    fn not(self) -> Self::Output {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }
}

#[repr(u8)]
pub enum CastlingKind {
    WhiteKing = 0b0001,
    WhiteQueen = 0b0010,
    BlackKing = 0b0100,
    BlackQueen = 0b1000,
}

pub type CastlingRights = u8;

impl TryFrom<char> for CastlingKind {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'K' => Ok(Self::WhiteKing),
            'Q' => Ok(Self::WhiteQueen),
            'k' => Ok(Self::BlackKing),
            'q' => Ok(Self::BlackQueen),
            _ => bail!("Invalid castling rights character: {}", value),
        }
    }
}

pub struct HistoryItem {
    pub castling_rights: CastlingRights,
    pub en_passant_square: Square,
    pub halfmove_clock: usize,
    pub moved_piece: Piece,
    pub captured_piece: Piece,
    pub hash: u64,
}

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

    white_occupancies: Bitboard,
    black_occupancies: Bitboard,

    side: Side,

    halfmove_clock: usize,

    castling_rights: CastlingRights,

    en_passant_square: Square,

    history: Vec<HistoryItem>,

    move_generator: MoveGenerator,

    hasher: ZobristHasher,
    hash: u64,
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

            white_occupancies: EMPTY_BB,
            black_occupancies: EMPTY_BB,

            side: Side::White,
            castling_rights: 0,

            halfmove_clock: 0,

            en_passant_square: Square::None,

            history: Vec::new(),

            move_generator: MoveGenerator::default(),

            hasher: ZobristHasher::default(),
            hash: 0,
        }
    }
}

impl Board {
    fn reset(&mut self) {
        self.white_pawns = EMPTY_BB;
        self.white_knights = EMPTY_BB;
        self.white_bishops = EMPTY_BB;
        self.white_rooks = EMPTY_BB;
        self.white_queens = EMPTY_BB;
        self.white_king = EMPTY_BB;

        self.black_pawns = EMPTY_BB;
        self.black_knights = EMPTY_BB;
        self.black_bishops = EMPTY_BB;
        self.black_rooks = EMPTY_BB;
        self.black_queens = EMPTY_BB;
        self.black_king = EMPTY_BB;

        self.pieces = [Piece::default(); 64];

        self.white_occupancies = EMPTY_BB;
        self.black_occupancies = EMPTY_BB;

        self.side = Side::White;
        self.castling_rights = 0;

        self.halfmove_clock = 0;

        self.en_passant_square = Square::None;

        self.history = Vec::new();
    }

    pub fn get_piece_bb(&self, piece: Piece) -> anyhow::Result<Bitboard> {
        match (piece.color, piece.kind) {
            (PieceColor::White, PieceKind::Pawn) => Ok(self.white_pawns),
            (PieceColor::White, PieceKind::Knight) => Ok(self.white_knights),
            (PieceColor::White, PieceKind::Bishop) => Ok(self.white_bishops),
            (PieceColor::White, PieceKind::Rook) => Ok(self.white_rooks),
            (PieceColor::White, PieceKind::Queen) => Ok(self.white_queens),
            (PieceColor::White, PieceKind::King) => Ok(self.white_king),
            (PieceColor::Black, PieceKind::Pawn) => Ok(self.black_pawns),
            (PieceColor::Black, PieceKind::Knight) => Ok(self.black_knights),
            (PieceColor::Black, PieceKind::Bishop) => Ok(self.black_bishops),
            (PieceColor::Black, PieceKind::Rook) => Ok(self.black_rooks),
            (PieceColor::Black, PieceKind::Queen) => Ok(self.black_queens),
            (PieceColor::Black, PieceKind::King) => Ok(self.black_king),
            _ => bail!("cannot get bitboard for invalid piece"),
        }
    }

    pub fn get_piece_bb_mut(&mut self, piece: Piece) -> anyhow::Result<&mut Bitboard> {
        match (piece.color, piece.kind) {
            (PieceColor::White, PieceKind::Pawn) => Ok(&mut self.white_pawns),
            (PieceColor::White, PieceKind::Knight) => Ok(&mut self.white_knights),
            (PieceColor::White, PieceKind::Bishop) => Ok(&mut self.white_bishops),
            (PieceColor::White, PieceKind::Rook) => Ok(&mut self.white_rooks),
            (PieceColor::White, PieceKind::Queen) => Ok(&mut self.white_queens),
            (PieceColor::White, PieceKind::King) => Ok(&mut self.white_king),
            (PieceColor::Black, PieceKind::Pawn) => Ok(&mut self.black_pawns),
            (PieceColor::Black, PieceKind::Knight) => Ok(&mut self.black_knights),
            (PieceColor::Black, PieceKind::Bishop) => Ok(&mut self.black_bishops),
            (PieceColor::Black, PieceKind::Rook) => Ok(&mut self.black_rooks),
            (PieceColor::Black, PieceKind::Queen) => Ok(&mut self.black_queens),
            (PieceColor::Black, PieceKind::King) => Ok(&mut self.black_king),
            _ => bail!("cannot get bitboard for invalid piece"),
        }
    }

    pub fn add_piece(&mut self, piece: Piece, square: Square) -> anyhow::Result<()> {
        self.get_piece_bb_mut(piece)?.set_bit(square);
        self.occupancy_mut(piece.color.try_into()?).set_bit(square);
        self.pieces[square] = piece;

        Ok(())
    }

    pub fn add_piece_and_hash(&mut self, piece: Piece, square: Square) -> anyhow::Result<()> {
        self.add_piece(piece, square)?;
        self.update_hash(ZobristKey::Piece(piece, square));

        Ok(())
    }

    pub fn remove_piece(&mut self, square: Square) -> anyhow::Result<Piece> {
        let piece = self.pieces[square];

        match piece.kind {
            PieceKind::NoPiece => bail!("cannot remove empty piece"),
            _ => {
                self.get_piece_bb_mut(piece)?.clear_bit(square);
                self.occupancy_mut(piece.color.try_into()?)
                    .clear_bit(square);
                self.pieces[square] = Piece::default();
                Ok(piece)
            }
        }
    }

    pub fn remove_piece_and_hash(&mut self, square: Square) -> anyhow::Result<Piece> {
        let piece = self.remove_piece(square)?;
        self.update_hash(ZobristKey::Piece(piece, square));

        Ok(piece)
    }

    pub fn get_piece(&self, square: Square) -> Piece {
        self.pieces[square]
    }

    pub fn parse_fen(&mut self, fen: &str) -> anyhow::Result<()> {
        self.reset();

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
                        self.add_piece(piece, square)?;
                        file_index += 1;
                    }

                    '1'..='8' => {
                        let empty_count: usize = ch
                            .to_digit(10)
                            .context("Can't convert to digit")?
                            .try_into()?;

                        file_index += empty_count;
                    }

                    _ => bail!("FEN has invalid character in piece placement data: {}", ch),
                }
            }
        }

        let side = fields.get(1).unwrap();

        match side.chars().nth(0) {
            Some('w') => self.side = Side::White,
            Some('b') => self.side = Side::Black,
            _ => bail!("FEN has invalid side notation. Expected `w` or `b`",),
        }

        let castling_rights = fields.get(2).unwrap();

        for ch in castling_rights.chars() {
            if ch == '-' {
                continue;
            }

            let castling_kind: CastlingKind = ch.try_into()?;
            self.castling_rights |= castling_kind as u8;
        }

        let en_passant = fields.get(3).unwrap().chars();

        let ep_file = en_passant.clone().nth(0).unwrap_or('-');
        let ep_rank = en_passant.clone().nth(1).unwrap_or('-');

        let en_passant_square = match (Rank::try_from(ep_rank), File::try_from(ep_file)) {
            (Ok(rank), Ok(file)) => Square::new(rank, file),
            _ => Square::None,
        };

        self.en_passant_square = en_passant_square;

        let halfmove_clock = fields.get(4).unwrap();

        self.halfmove_clock = halfmove_clock.parse()?;

        self.hash = self.hasher.hash_position(self);

        Ok(())
    }

    pub fn can_castle(&self, castling_kind: CastlingKind) -> bool {
        self.castling_rights & (castling_kind as u8) != 0
    }

    pub fn castling_rights(&self) -> u8 {
        self.castling_rights
    }

    pub fn hash_castling_rights(&mut self) {
        self.update_hash(ZobristKey::Castling(self.castling_rights));
    }

    pub fn set_castling_rights(&mut self, rights: u8) {
        self.castling_rights = rights;
    }

    pub fn empty_squares(&self) -> Bitboard {
        !(self.occupancy(Side::White) | self.occupancy(Side::Black))
    }

    pub fn side_to_move(&self) -> Side {
        self.side
    }

    pub fn switch_side(&mut self) {
        self.side = match self.side {
            Side::White => Side::Black,
            Side::Black => Side::White,
        };
    }

    pub fn switch_side_and_hash(&mut self) {
        self.switch_side();
        self.update_hash(ZobristKey::Side);
    }

    pub fn reset_clock(&mut self) {
        self.halfmove_clock = 0;
    }

    pub fn increment_clock(&mut self) {
        self.halfmove_clock += 1;
    }

    pub fn occupancy(&self, side: Side) -> Bitboard {
        match side {
            Side::White => self.white_occupancies,
            Side::Black => self.black_occupancies,
        }
    }

    fn occupancy_mut(&mut self, side: Side) -> &mut Bitboard {
        match side {
            Side::White => &mut self.white_occupancies,
            Side::Black => &mut self.black_occupancies,
        }
    }

    pub fn en_passant_square(&self) -> Square {
        self.en_passant_square
    }

    pub fn hash_en_passant_square(&mut self) {
        self.update_hash(ZobristKey::EnPassantFile(self.en_passant_square));
    }

    pub fn set_en_passant_square(&mut self, square: Square) {
        self.en_passant_square = square;
    }

    pub fn halfmove_clock(&self) -> usize {
        self.halfmove_clock
    }

    pub fn set_halfmove_clock(&mut self, halfmove_clock: usize) {
        self.halfmove_clock = halfmove_clock;
    }

    fn get_king_square(&self, side: Side) -> Square {
        let king_bitboard = self
            .get_piece_bb(Piece::new(side.into(), PieceKind::King))
            .unwrap();

        king_bitboard.0.trailing_zeros().into()
    }

    pub fn is_in_check(&self, side: Side) -> bool {
        self.move_generator
            .is_square_attacked(self, self.get_king_square(side), !side)
    }

    pub fn push_history(&mut self, history_item: HistoryItem) {
        self.history.push(history_item);
    }

    pub fn pop_history(&mut self) -> HistoryItem {
        self.history.pop().unwrap()
    }

    pub fn generate_all_moves(&self, move_list: &mut MoveList) -> anyhow::Result<()> {
        self.move_generator.generate_all_moves(self, move_list)
    }

    pub fn pieces(&self) -> &[Piece] {
        &self.pieces
    }

    pub fn update_hash(&mut self, cause: ZobristKey) {
        self.hash ^= self.hasher.get_key_part(cause);
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }

    pub fn set_hash(&mut self, hash: u64) {
        self.hash = hash
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

    writeln!(f)?;
    writeln!(f)?;

    writeln!(f, "Side to play: {:?}", board.side)?;

    writeln!(
        f,
        "White king castle: {}",
        if board.can_castle(CastlingKind::WhiteKing) {
            "yes"
        } else {
            "no"
        }
    )?;

    writeln!(
        f,
        "White queen castle: {}",
        if board.can_castle(CastlingKind::WhiteQueen) {
            "yes"
        } else {
            "no"
        }
    )?;

    writeln!(
        f,
        "Black king castle: {}",
        if board.can_castle(CastlingKind::BlackKing) {
            "yes"
        } else {
            "no"
        }
    )?;

    writeln!(
        f,
        "Black queen castle: {}",
        if board.can_castle(CastlingKind::BlackQueen) {
            "yes"
        } else {
            "no"
        }
    )?;

    writeln!(f, "Halfmove clock: {:?}", board.halfmove_clock)?;

    writeln!(f, "En passant square: {:?}", board.en_passant_square)?;

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
