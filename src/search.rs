use crate::{
    board::{Board, Side},
    evaluate::evaluate,
    move_generator::{Move, MoveList},
};

pub struct Search;

impl Search {
    pub fn search_position(&self, board: &mut Board, depth: u8) -> anyhow::Result<Option<Move>> {
        let mut best_score = i32::MIN;
        let mut best_move = None;

        let mut move_list = MoveList::default();
        board.generate_all_moves(&mut move_list)?;

        for mv in move_list {
            if board.make_move(mv)? {
                let score = self.minimax(board, depth - 1)?;
                if score > best_score {
                    best_score = score;
                    best_move = Some(mv);
                }
            }

            board.unmake_move(mv)?;
        }

        Ok(best_move)
    }

    fn minimax(&self, board: &mut Board, depth: u8) -> anyhow::Result<i32> {
        if depth == 0 {
            return Ok(evaluate(board));
        }

        let mut move_list = MoveList::default();
        board.generate_all_moves(&mut move_list)?;

        let is_maximizing = board.side_to_move() == Side::White;

        if is_maximizing {
            let mut best_score = i32::MIN;

            for mv in move_list {
                if board.make_move(mv)? {
                    let score = self.minimax(board, depth - 1)?;
                    best_score = best_score.max(score);
                }
                board.unmake_move(mv)?;
            }

            Ok(best_score)
        } else {
            let mut best_score = i32::MAX;

            for mv in move_list {
                if board.make_move(mv)? {
                    let score = self.minimax(board, depth - 1)?;
                    best_score = best_score.min(score);
                }
                board.unmake_move(mv)?;
            }

            Ok(best_score)
        }
    }
}
