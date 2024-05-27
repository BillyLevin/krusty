use anyhow::bail;

use crate::{
    board::Board,
    move_generator::{Move, MoveFlag, MoveKind},
    square::{Piece, PieceKind, Square},
};

impl Board {
    pub fn make_move(&mut self, mv: Move) -> anyhow::Result<()> {
        let from_square = mv.from_square();
        let moved_piece = self.remove_piece(from_square)?;

        self.increment_clock();

        match mv.kind() {
            MoveKind::Quiet => self.add_piece(moved_piece, mv.to_square())?,
            MoveKind::Capture => {
                self.remove_piece(mv.to_square())?;
                self.add_piece(moved_piece, mv.to_square())?;
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

        self.switch_side();

        Ok(())
    }
}
