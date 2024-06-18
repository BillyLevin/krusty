use crate::{
    board::{Board, START_POSITION_FEN},
    evaluate::{
        evaluate, BISHOP_VALUE, KING_VALUE, KNIGHT_VALUE, PAWN_VALUE, QUEEN_VALUE, ROOK_VALUE,
    },
    move_generator::{Move, MoveList},
    square::PieceKind,
    time_management::SearchTimer,
    transposition_table::{SearchEntryFlag, SearchTableEntry, TranspositionTable},
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
const CAPTURE_SCORE_OFFSET: i32 = 1000;
const TT_SCORE_OFFSET: i32 = CAPTURE_SCORE_OFFSET + 10000;

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
            transposition_table: TranspositionTable::new(256),
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
        self.search_info.nodes_searched = 0;
        self.search_info.ply = 0;

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
        mut depth: u8,
        alpha: i32,
        beta: i32,
        best_move: &mut Move,
    ) -> anyhow::Result<i32> {
        // search a bit further if in check
        if self.board.is_in_check(self.board.side_to_move()) {
            depth += 1;
        }

        if depth == 0 {
            return self.quiescence_search(alpha, beta);
        }

        let table_entry = self.transposition_table.probe(self.board.hash());
        let (transposition_score, transposition_move) =
            table_entry.get(self.board.hash(), depth, self.search_info.ply, alpha, beta);

        if let Some(score) = transposition_score {
            if self.search_info.ply != 0 {
                return Ok(score);
            }
        }

        self.search_info.nodes_searched += 1;

        // check move time expiry every 2048 nodes
        if (self.search_info.nodes_searched & 2047) == 0 {
            self.timer.check();
        }

        if self.timer.is_stopped() {
            return Ok(0);
        }

        if self.board.is_draw() {
            return Ok(0);
        }

        let mut move_list = MoveList::default();
        self.board.generate_all_moves(&mut move_list)?;

        let mut legal_move_count = 0;
        let old_alpha = alpha;
        let mut alpha = alpha;

        let mut best_score_from_node = -INFINITY;
        let mut best_move_from_node = Move::NULL_MOVE;

        self.score_moves(&mut move_list, transposition_move);

        for i in 0..move_list.length() {
            let mv = move_list.pick_ordered_move(i);

            if !self.board.make_move(mv)? {
                self.board.unmake_move(mv)?;
                continue;
            }

            self.search_info.ply += 1;
            legal_move_count += 1;

            let score = -self.negamax(depth - 1, -beta, -alpha, best_move)?;

            self.board.unmake_move(mv)?;
            self.search_info.ply -= 1;

            if score > best_score_from_node {
                best_score_from_node = score;
                best_move_from_node = mv;
            }

            // move is very good for our opponent, disregard it
            if score >= beta {
                self.transposition_table.store(SearchTableEntry::new(
                    self.board.hash(),
                    depth,
                    beta,
                    self.search_info.ply,
                    SearchEntryFlag::Beta,
                    best_move_from_node,
                ));
                return Ok(beta);
            }

            if score > alpha {
                alpha = score;

                if self.search_info.ply == 0 {
                    *best_move = mv;
                }
            }
        }

        // no legal moves means it's either checkmate or stalemate
        if legal_move_count == 0 {
            if self.board.is_in_check(self.board.side_to_move()) {
                return Ok(-INFINITY + self.search_info.ply as i32);
            } else {
                return Ok(0);
            }
        }

        self.transposition_table.store(SearchTableEntry::new(
            self.board.hash(),
            depth,
            alpha,
            self.search_info.ply,
            if alpha == old_alpha {
                SearchEntryFlag::Alpha
            } else {
                SearchEntryFlag::Exact
            },
            best_move_from_node,
        ));

        Ok(alpha)
    }

    pub fn quiescence_search(&mut self, mut alpha: i32, beta: i32) -> anyhow::Result<i32> {
        self.search_info.nodes_searched += 1;

        // check move time expiry every 2048 nodes
        if (self.search_info.nodes_searched & 2047) == 0 {
            self.timer.check();
        }

        if self.timer.is_stopped() {
            return Ok(0);
        }

        if self.board.is_draw() {
            return Ok(0);
        }

        let stand_pat = evaluate(&self.board);

        if stand_pat >= beta {
            return Ok(beta);
        }

        alpha = alpha.max(stand_pat);

        let mut move_list = MoveList::default();
        self.board.generate_all_captures(&mut move_list)?;

        self.score_moves(&mut move_list, Move::NULL_MOVE);

        for i in 0..move_list.length() {
            let mv = move_list.pick_ordered_move(i);

            if !self.board.make_move(mv)? {
                self.board.unmake_move(mv)?;
                continue;
            }

            self.search_info.ply += 1;

            let score = -self.quiescence_search(-beta, -alpha)?;

            self.board.unmake_move(mv)?;
            self.search_info.ply -= 1;

            alpha = alpha.max(score);

            if score >= beta {
                break;
            }
        }

        Ok(alpha)
    }

    fn score_moves(&self, move_list: &mut MoveList, transposition_move: Move) {
        for i in 0..move_list.length() {
            let mv = move_list.get_mut(i);

            let mut score = 0;

            let victim = self.board.get_piece(mv.to_square());

            if *mv == transposition_move {
                score = TT_SCORE_OFFSET;
            } else if victim.kind != PieceKind::NoPiece {
                let attacker = self.board.get_piece(mv.from_square());

                score = CAPTURE_SCORE_OFFSET + (10 * victim.material_value())
                    - attacker.material_value();
            }

            assert!(score >= 0, "score must be above 0, got {}", score);
            assert!(
                score <= (1 << 15),
                "score must be below {}, got {}",
                1 << 15,
                score
            );

            mv.set_score(score as u32);
        }
    }

    fn get_score_string(score: i32) -> String {
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
