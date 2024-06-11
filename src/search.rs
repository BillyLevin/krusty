use crate::{
    board::{Board, Side, START_POSITION_FEN},
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

#[derive(Debug, Clone, Copy, Default)]
pub struct SearchInfo {
    pub depth: u8,
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
    pub fn search_position(&mut self) -> anyhow::Result<Move> {
        let depth = self.max_depth;

        let mut best_move = Move::NULL_MOVE;

        let is_maximizing = self.board.side_to_move() == Side::White;

        if is_maximizing {
            let mut best_score = i32::MIN;
            let score = self.minimax(depth, &mut best_move)?;
            if score > best_score {
                best_score = score;
            }
            println!(
                "info depth {} score {}",
                depth,
                Self::get_score_string(best_score, true)
            );
        } else {
            let mut best_score = i32::MAX;
            let score = self.minimax(depth, &mut best_move)?;
            if score < best_score {
                best_score = score;
            }
            println!(
                "info depth {} score {}",
                depth,
                Self::get_score_string(best_score, false)
            );
        }

        Ok(best_move)
    }

    fn minimax(&mut self, depth: u8, best_move: &mut Move) -> anyhow::Result<i32> {
        if depth == 0 {
            return Ok(evaluate(&self.board));
        }

        let table_entry = self.transposition_table.probe(self.board.hash());
        if let Some(score) = table_entry.get(self.board.hash(), depth, self.search_info.ply) {
            return Ok(score);
        }

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

        let is_maximizing = self.board.side_to_move() == Side::White;

        if is_maximizing {
            let mut best_score = i32::MIN;
            let mut legal_move_count = 0;

            for mv in move_list {
                if !self.board.make_move(mv)? {
                    self.board.unmake_move(mv)?;
                    continue;
                }

                self.search_info.ply += 1;
                legal_move_count += 1;

                let score = self.minimax(depth - 1, best_move)?;

                self.board.unmake_move(mv)?;
                self.search_info.ply -= 1;

                if score > best_score {
                    best_score = score;

                    if self.search_info.ply == 0 {
                        *best_move = mv;
                    }
                }
            }

            // no legal moves means it's either checkmate or stalemate
            if legal_move_count == 0 {
                if self.board.is_in_check(self.board.side_to_move()) {
                    return Ok(i32::MIN + self.search_info.ply as i32);
                } else {
                    return Ok(0);
                }
            }

            self.transposition_table.store(SearchTableEntry::new(
                self.board.hash(),
                depth,
                best_score,
                self.search_info.ply,
            ));

            Ok(best_score)
        } else {
            let mut best_score = i32::MAX;
            let mut legal_move_count = 0;

            for mv in move_list {
                if !self.board.make_move(mv)? {
                    self.board.unmake_move(mv)?;
                    continue;
                }

                self.search_info.ply += 1;
                legal_move_count += 1;

                let score = self.minimax(depth - 1, best_move)?;

                self.board.unmake_move(mv)?;
                self.search_info.ply -= 1;

                if score < best_score {
                    best_score = score;

                    if self.search_info.ply == 0 {
                        *best_move = mv;
                    }
                }
            }

            // no legal moves means it's either checkmate or stalemate
            if legal_move_count == 0 {
                if self.board.is_in_check(self.board.side_to_move()) {
                    return Ok(i32::MAX - self.search_info.ply as i32);
                } else {
                    return Ok(0);
                }
            }

            self.transposition_table.store(SearchTableEntry::new(
                self.board.hash(),
                depth,
                best_score,
                self.search_info.ply,
            ));

            Ok(best_score)
        }
    }

    pub fn get_score_string(score: i32, is_maximizing: bool) -> String {
        if score > CHECKMATE_THRESHOLD {
            let ply_to_mate = i32::MAX.abs_diff(score) as i32;
            let moves_to_mate = ply_to_mate / 2 + ply_to_mate % 2;
            let moves_to_mate = if is_maximizing {
                moves_to_mate
            } else {
                -moves_to_mate
            };
            format!("mate {}", moves_to_mate)
        } else if score < -CHECKMATE_THRESHOLD {
            let ply_to_mate = i32::MIN.abs_diff(score) as i32;
            let moves_to_mate = ply_to_mate / 2 + ply_to_mate % 2;
            let moves_to_mate = if is_maximizing {
                -moves_to_mate
            } else {
                moves_to_mate
            };
            format!("mate {}", moves_to_mate)
        } else {
            format!("cp {}", score)
        }
    }
}
