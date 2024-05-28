use anyhow::bail;

use crate::{
    bitboard::EMPTY_BB,
    board::{Board, Side},
    move_generator::{pawn_attacks, Move, MoveFlag, MoveKind},
    square::{Piece, PieceKind, Square},
};

impl Board {
    pub fn make_move(&mut self, mv: Move) -> anyhow::Result<()> {
        let from_square = mv.from_square();
        let moved_piece = self.remove_piece(from_square)?;

        self.increment_clock();

        self.set_en_passant_square(Square::None);

        match mv.kind() {
            MoveKind::Quiet => self.add_piece(moved_piece, mv.to_square())?,
            MoveKind::Capture => {
                // captures (and pawn pushes, handled lower down) reset halfmove clock for 50-move
                // rule
                self.reset_clock();

                if mv.flag() == MoveFlag::EnPassant {
                    let captured_square = match self.side_to_move() {
                        Side::White => mv.to_square().south(),
                        Side::Black => mv.to_square().north(),
                    };

                    self.remove_piece(captured_square)?;
                    self.add_piece(moved_piece, mv.to_square())?;
                } else {
                    self.remove_piece(mv.to_square())?;
                    self.add_piece(moved_piece, mv.to_square())?;
                }
            }
            MoveKind::Castle => {
                self.add_piece(moved_piece, mv.to_square())?;

                let (rook_from, rook_to) = match mv.to_square() {
                    Square::G1 => (Square::H1, Square::F1),
                    Square::C1 => (Square::A1, Square::D1),
                    Square::G8 => (Square::H8, Square::F8),
                    Square::C8 => (Square::A8, Square::D8),
                    _ => bail!("tried to castle to illegal square: {:?}", mv.to_square()),
                };
                let rook = self.remove_piece(rook_from)?;
                self.add_piece(rook, rook_to)?;
            }
            MoveKind::Promotion => {
                if self.get_piece(mv.to_square()).kind != PieceKind::NoPiece {
                    self.remove_piece(mv.to_square())?;
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

                self.add_piece(promotion_piece, mv.to_square())?;
            }
        };

        if moved_piece.kind == PieceKind::Pawn {
            self.reset_clock();

            let is_double_push = mv.from_square().distance_between(mv.to_square()) == 16;

            if is_double_push {
                let ep_square = match self.side_to_move() {
                    Side::White => mv.from_square().north(),
                    Side::Black => mv.from_square().south(),
                };

                // we only set en passant square if a pawn can actually capture, as per
                // https://github.com/fsmosca/PGN-Standard/blob/61a82dab3ff62d79dea82c15a8cc773f80f3a91e/PGN-Standard.txt#L2231-L2242
                let enemy_pawns =
                    self.get_piece_bb(Piece::new((!self.side_to_move()).into(), PieceKind::Pawn))?;

                let attacks = pawn_attacks(self.side_to_move())[ep_square.index()];

                if attacks & enemy_pawns != EMPTY_BB {
                    self.set_en_passant_square(ep_square);
                }
            }
        }

        self.switch_side();

        Ok(())
    }
}
