use anyhow::bail;

use crate::{
    bitboard::EMPTY_BB,
    board::{Board, CastlingKind, HistoryItem, Side},
    move_generator::{pawn_attacks, Move, MoveFlag, MoveKind, MoveList},
    square::{Piece, PieceKind, Square},
};

/// this is the information that can be extracted from a move string in long algebraic notation,
/// e.g. `e2e4`, `e7e8q`
#[derive(Debug, Clone, Copy)]
pub struct MoveMetadata {
    from: Square,
    to: Square,
    promotion: Option<PieceKind>,
}

const fn init_castling_permissions_table() -> [u8; 64] {
    let mut table = [15; 64];

    let white_queen = CastlingKind::WhiteQueen as u8;
    let white_king = CastlingKind::WhiteKing as u8;
    let black_queen = CastlingKind::BlackQueen as u8;
    let black_king = CastlingKind::BlackKing as u8;

    table[Square::A1 as usize] = 15 - white_queen;
    table[Square::E1 as usize] = 15 - white_queen - white_king;
    table[Square::H1 as usize] = 15 - white_king;

    table[Square::A8 as usize] = 15 - black_queen;
    table[Square::E8 as usize] = 15 - black_queen - black_king;
    table[Square::H8 as usize] = 15 - black_king;

    table
}

/// this table allows us to update the castling rights after each move is made
/// each castling permission is represented by its own bit in a 4-bit int (see `CastlingKind` def)
///
/// the values in this table represent what the new castling rights would be after removing the
/// relevant rights (assuming you started with full rights)
///
/// as an example, if the rook on A1 moves or is captured, white can no longer castle queenside, so
/// we subtract the value of that right from 15 (0b1111)
///
/// in practice, the value of this table will be bitwise AND'd with the current castling rights to
/// get the updated rights
const CASTLING_PERMISSIONS_TABLE: [u8; 64] = init_castling_permissions_table();

impl Board {
    pub fn make_move(&mut self, mv: Move) -> anyhow::Result<bool> {
        let old_hash = self.hash();
        let from_square = mv.from_square();
        let to_square = mv.to_square();
        let moved_piece = self.remove_piece_and_hash(from_square)?;

        let mut history_item = HistoryItem {
            castling_rights: self.castling_rights(),
            en_passant_square: self.en_passant_square(),
            halfmove_clock: self.halfmove_clock(),
            moved_piece,
            captured_piece: self.get_piece(to_square),
            hash: old_hash,
        };

        self.increment_clock();

        // due to the behaviour of XOR (a ^ b ^ b == a), calling the hash function with the same square will
        // remove its value from the hash, then we can hash it again later when we get the new en
        // passant square
        self.hash_en_passant_square();
        self.set_en_passant_square(Square::None);

        match mv.kind() {
            MoveKind::Quiet => self.add_piece_and_hash(moved_piece, to_square)?,
            MoveKind::Capture => {
                // captures (and pawn pushes, handled lower down) reset halfmove clock for 50-move
                // rule
                self.reset_clock();

                if mv.flag() == MoveFlag::EnPassant {
                    let captured_square = match self.side_to_move() {
                        Side::White => to_square.south(),
                        Side::Black => to_square.north(),
                    };

                    history_item.captured_piece = self.remove_piece_and_hash(captured_square)?;
                    self.add_piece_and_hash(moved_piece, to_square)?;
                } else {
                    self.remove_piece_and_hash(to_square)?;
                    self.add_piece_and_hash(moved_piece, to_square)?;
                }
            }
            MoveKind::Castle => {
                self.add_piece_and_hash(moved_piece, to_square)?;

                let (rook_from, rook_to) = match to_square {
                    Square::G1 => (Square::H1, Square::F1),
                    Square::C1 => (Square::A1, Square::D1),
                    Square::G8 => (Square::H8, Square::F8),
                    Square::C8 => (Square::A8, Square::D8),
                    _ => bail!("tried to castle to illegal square: {:?}", to_square),
                };
                let rook = self.remove_piece_and_hash(rook_from)?;
                self.add_piece_and_hash(rook, rook_to)?;
            }
            MoveKind::Promotion => {
                if self.get_piece(to_square).kind != PieceKind::NoPiece {
                    self.remove_piece_and_hash(to_square)?;
                }

                let promotion_piece = match mv.flag() {
                    MoveFlag::KnightPromotion => {
                        Piece::new(self.side_to_move().into(), PieceKind::Knight)
                    }
                    MoveFlag::BishopPromotion => {
                        Piece::new(self.side_to_move().into(), PieceKind::Bishop)
                    }
                    MoveFlag::RookPromotion => {
                        Piece::new(self.side_to_move().into(), PieceKind::Rook)
                    }
                    MoveFlag::QueenPromotion => {
                        Piece::new(self.side_to_move().into(), PieceKind::Queen)
                    }
                    _ => bail!("tried to make promotion move without providing a promotion flag"),
                };

                self.add_piece_and_hash(promotion_piece, to_square)?;
            }
        };

        if moved_piece.kind == PieceKind::Pawn {
            self.reset_clock();

            let is_double_push = from_square.distance_between(to_square) == 16;

            if is_double_push {
                let ep_square = match self.side_to_move() {
                    Side::White => from_square.north(),
                    Side::Black => from_square.south(),
                };

                // we only set en passant square if a pawn can actually capture, as per
                // https://github.com/fsmosca/PGN-Standard/blob/61a82dab3ff62d79dea82c15a8cc773f80f3a91e/PGN-Standard.txt#L2231-L2242
                let enemy_pawns =
                    self.get_piece_bb(Piece::new((!self.side_to_move()).into(), PieceKind::Pawn))?;

                let attacks = pawn_attacks(self.side_to_move())[ep_square.index()];

                if attacks & enemy_pawns != EMPTY_BB {
                    self.set_en_passant_square(ep_square);
                    self.hash_en_passant_square();
                }
            }
        }

        // clear current castling hash
        self.hash_castling_rights();
        let new_castling_rights = self.castling_rights()
            & CASTLING_PERMISSIONS_TABLE[from_square.index()]
            & CASTLING_PERMISSIONS_TABLE[to_square.index()];
        self.set_castling_rights(new_castling_rights);
        // hash new castling rights
        self.hash_castling_rights();

        self.switch_side_and_hash();

        self.push_history(history_item);

        // we return `true` if the move was legal, `false` if not
        // not that we just switched the side to the next player, so we pass the opposite side into
        // this function
        Ok(!self.is_in_check(!self.side_to_move()))
    }

    pub fn unmake_move(&mut self, mv: Move) -> anyhow::Result<()> {
        let history_item: HistoryItem = self.pop_history();

        self.set_castling_rights(history_item.castling_rights);
        self.set_en_passant_square(history_item.en_passant_square);
        self.set_halfmove_clock(history_item.halfmove_clock);
        self.set_hash(history_item.hash);

        self.switch_side();

        let from_square = mv.from_square();
        let to_square = mv.to_square();

        self.add_piece(history_item.moved_piece, from_square)?;

        match mv.kind() {
            MoveKind::Quiet => {
                self.remove_piece(to_square)?;
            }
            MoveKind::Capture => {
                if mv.flag() == MoveFlag::EnPassant {
                    let captured_square = match self.side_to_move() {
                        Side::White => to_square.south(),
                        Side::Black => to_square.north(),
                    };

                    self.remove_piece(to_square)?;
                    self.add_piece(history_item.captured_piece, captured_square)?;
                } else {
                    self.remove_piece(to_square)?;
                    self.add_piece(history_item.captured_piece, to_square)?;
                }
            }
            MoveKind::Castle => {
                // remove the king
                self.remove_piece(to_square)?;

                // put the rook back to its original square
                let (rook_from, rook_to) = match to_square {
                    Square::G1 => (Square::H1, Square::F1),
                    Square::C1 => (Square::A1, Square::D1),
                    Square::G8 => (Square::H8, Square::F8),
                    Square::C8 => (Square::A8, Square::D8),
                    _ => bail!(
                        "tried to unmake illegal castling move with `to_square`: {:?}",
                        to_square
                    ),
                };
                let rook = self.remove_piece(rook_to)?;
                self.add_piece(rook, rook_from)?;
            }
            MoveKind::Promotion => {
                self.remove_piece(to_square)?;
                if history_item.captured_piece.kind != PieceKind::NoPiece {
                    self.add_piece(history_item.captured_piece, to_square)?;
                }
            }
        };

        Ok(())
    }

    pub fn make_null_move(&mut self) {
        let old_hash = self.hash();

        let history_item = HistoryItem {
            castling_rights: self.castling_rights(),
            en_passant_square: self.en_passant_square(),
            halfmove_clock: self.halfmove_clock(),
            moved_piece: Piece::default(),
            captured_piece: Piece::default(),
            hash: old_hash,
        };

        self.reset_clock();

        self.hash_en_passant_square();
        self.set_en_passant_square(Square::None);

        self.switch_side_and_hash();

        self.push_history(history_item);
    }

    pub fn unmake_null_move(&mut self) {
        let history_item: HistoryItem = self.pop_history();

        self.set_castling_rights(history_item.castling_rights);
        self.set_en_passant_square(history_item.en_passant_square);
        self.set_halfmove_clock(history_item.halfmove_clock);
        self.set_hash(history_item.hash);

        self.switch_side();
    }

    pub fn get_move_metadata(&self, move_str: &str) -> anyhow::Result<MoveMetadata> {
        if move_str.len() < 4 {
            bail!("Move string `{}` is invalid", move_str)
        }

        let mut chars = move_str.chars();
        let from_file = chars.next().unwrap().try_into()?;
        let from_rank = chars.next().unwrap().try_into()?;
        let to_file = chars.next().unwrap().try_into()?;
        let to_rank = chars.next().unwrap().try_into()?;

        let from = Square::new(from_rank, from_file);
        let to = Square::new(to_rank, to_file);

        let promotion: Option<PieceKind> = match chars.next() {
            Some(piece) => Some(piece.try_into()?),
            None => None,
        };

        Ok(MoveMetadata {
            from,
            to,
            promotion,
        })
    }

    pub fn find_matching_move(&self, move_metadata: MoveMetadata) -> Option<Move> {
        let MoveMetadata {
            from,
            to,
            promotion,
        } = move_metadata;

        let mut possible_moves = MoveList::default();
        self.generate_all_moves(&mut possible_moves).unwrap();

        for possible_move in possible_moves {
            if possible_move.from_square() == from && possible_move.to_square() == to {
                if possible_move.kind() == MoveKind::Promotion {
                    let promotion_piece = promotion.expect("promotion piece missing");
                    if promotion_piece == possible_move.flag().into() {
                        return Some(possible_move);
                    }
                } else {
                    return Some(possible_move);
                }
            }
        }

        None
    }
}
