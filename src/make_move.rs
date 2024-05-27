use anyhow::bail;

use crate::{
    board::Board,
    move_generator::{Move, MoveKind},
    square::Square,
};

impl Board {
    pub fn make_move(&mut self, mv: Move) -> anyhow::Result<()> {
        let from_square = mv.from_square();
        let moved_piece = self.remove_piece(from_square)?;

        self.switch_side();
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
            MoveKind::Promotion => todo!(),
        };

        Ok(())
    }
}
