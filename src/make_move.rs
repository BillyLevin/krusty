use crate::{
    board::Board,
    move_generator::{Move, MoveKind},
};

impl Board {
    pub fn make_move(&mut self, mv: Move) -> anyhow::Result<()> {
        let from_square = mv.from_square();
        let moved_piece = self.remove_piece(from_square)?;

        self.switch_side();

        match mv.kind() {
            MoveKind::Quiet => self.add_piece(moved_piece, mv.to_square())?,
            MoveKind::Capture => todo!(),
            MoveKind::Castle => todo!(),
            MoveKind::Promotion => todo!(),
        };

        Ok(())
    }
}
