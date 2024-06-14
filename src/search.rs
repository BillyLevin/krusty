use crate::{
    board::{Board, START_POSITION_FEN},
    evaluate::{
        evaluate, BISHOP_VALUE, KING_VALUE, KNIGHT_VALUE, PAWN_VALUE, QUEEN_VALUE, ROOK_VALUE,
    },
    move_generator::{Move, MoveList},
    time_management::SearchTimer,
    transposition_table::{SearchTableEntry, TranspositionTable},
};

// if the score is higher than this, it's definitely checkmate
pub const CHECKMATE_THRESHOLD: i32 = PAWN_VALUE * 8
    + KNIGHT_VALUE * 8
    + BISHOP_VALUE * 8
    + ROOK_VALUE * 8
    + QUEEN_VALUE * 8
    + KING_VALUE;

#[derive(Debug, Clone, Copy)]
pub enum SearchDepth {
    Finite(u8),
    Infinite,
}

impl SearchDepth {
    const MAX: u8 = 64;
}

impl From<SearchDepth> for u8 {
    fn from(value: SearchDepth) -> Self {
        match value {
            SearchDepth::Finite(depth) => depth,
            SearchDepth::Infinite => SearchDepth::MAX,
        }
    }
}

const INFINITY: i32 = 100_000;

#[derive(Debug, Clone, Copy, Default)]
pub struct SearchInfo {
    pub ply: u8,
    pub nodes_searched: u64,
}

pub struct Search {
    transposition_table: TranspositionTable<SearchTableEntry>,
    pub board: Board,

    pub search_info: SearchInfo,
    pub timer: SearchTimer,
    pub max_depth: u8,
}

impl Default for Search {
    fn default() -> Self {
        let mut board = Board::default();
        board.parse_fen(START_POSITION_FEN).unwrap();

        Self {
            transposition_table: TranspositionTable::new(64),
            board,
            search_info: SearchInfo::default(),
            timer: SearchTimer::default(),
            max_depth: SearchDepth::MAX,
        }
    }
}

impl Search {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn search_position(&mut self) -> anyhow::Result<Move> {
        let max_depth = self.max_depth;

        let mut best_move = Move::NULL_MOVE;

        for depth in 1..=max_depth {
            let mut current_best_move = Move::NULL_MOVE;

            let score = self.negamax(depth, -INFINITY, INFINITY, &mut current_best_move)?;

            if self.timer.is_stopped() {
                break;
            }

            best_move = current_best_move;

            println!(
                "info depth {} score {} nodes {}",
                depth,
                Self::get_score_string(score,),
                self.search_info.nodes_searched
            );
        }

        Ok(best_move)
    }

    fn negamax(
        &mut self,
        depth: u8,
        alpha: i32,
        beta: i32,
        best_move: &mut Move,
    ) -> anyhow::Result<i32> {
        if depth == 0 {
            return Ok(evaluate(&self.board));
        }

        // let table_entry = self.transposition_table.probe(self.board.hash());
        // if let Some(score) = table_entry.get(self.board.hash(), depth, self.search_info.ply) {
        //     return Ok(score);
        // }

        self.search_info.nodes_searched += 1;

        // check move time expiry every 2048 nodes
        if (self.search_info.nodes_searched & 2047) == 0 {
            self.timer.check();
        }

        if self.timer.is_stopped() {
            return Ok(0);
        }

        let mut move_list = MoveList::default();
        self.board.generate_all_moves(&mut move_list)?;

        let mut legal_move_count = 0;
        let mut alpha = alpha;

        for mv in move_list {
            if !self.board.make_move(mv)? {
                self.board.unmake_move(mv)?;
                continue;
            }

            self.search_info.ply += 1;
            legal_move_count += 1;

            let score = -self.negamax(depth - 1, -beta, -alpha, best_move)?;

            self.board.unmake_move(mv)?;
            self.search_info.ply -= 1;

            // move is very good for our opponent, disregard it
            if score >= beta {
                return Ok(beta);
            }

            if score > alpha && self.search_info.ply == 0 {
                *best_move = mv;
            }

            alpha = alpha.max(score);
        }

        // no legal moves means it's either checkmate or stalemate
        if legal_move_count == 0 {
            if self.board.is_in_check(self.board.side_to_move()) {
                return Ok(-INFINITY + self.search_info.ply as i32);
            } else {
                return Ok(0);
            }
        }

        // self.transposition_table.store(SearchTableEntry::new(
        //     self.board.hash(),
        //     depth,
        //     best_score,
        //     self.search_info.ply,
        // ));

        Ok(alpha)
    }

    pub fn get_score_string(score: i32) -> String {
        if score.abs() > CHECKMATE_THRESHOLD {
            let ply_to_mate = INFINITY.abs_diff(score.abs()) as i32;
            let moves_to_mate = ply_to_mate / 2 + ply_to_mate % 2;

            // prints from engine's perspective, so if e.g. engine is being mated in 2
            // moves, we print `mate -2`
            format!("mate {}", moves_to_mate * score.signum())
        } else {
            format!("cp {}", score)
        }
    }
}
