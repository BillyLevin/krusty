use crate::{
    board::{Board, START_POSITION_FEN},
    evaluate::{
        evaluate, BISHOP_VALUE, KING_VALUE, KNIGHT_VALUE, PAWN_VALUE, QUEEN_VALUE, ROOK_VALUE,
    },
    move_generator::{Move, MoveKind, MoveList},
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
const FIRST_KILLER_SCORE: i32 = CAPTURE_SCORE_OFFSET - 1;
const SECOND_KILLER_SCORE: i32 = CAPTURE_SCORE_OFFSET - 2;
const COUNTER_MOVE_BONUS: i32 = 1;
// history heuristic must always be lower in move ordering than killer heuristic
const MAX_HISTORY_SCORE: i32 = SECOND_KILLER_SCORE - COUNTER_MOVE_BONUS - 1;

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

    // quiet moves that caused a beta-cutoff, indexed by search ply
    pub killer_moves: [[Move; 2]; SearchDepth::MAX as usize + 1],

    // counts the number of times a move caused a cutoff (with depth used as a multipler to
    // prioritise higher depth cutoffs).
    pub history: [[[u32; 64]; 64]; 2],

    // keeps track of any cutoffs caused by a particular from-to move, the idea being that it might
    // also be a good counter move to the same from-to move in other positions
    pub counter_moves: [[[Move; 64]; 64]; 2],
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
            killer_moves: [[Move::NULL_MOVE; 2]; SearchDepth::MAX as usize + 1],
            history: [[[0; 64]; 64]; 2],
            counter_moves: [[[Move::NULL_MOVE; 64]; 64]; 2],
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
        let mut pv = Vec::new();

        for depth in 1..=max_depth {
            let score = self.negamax(depth, -INFINITY, INFINITY, &mut pv, Move::NULL_MOVE)?;

            if self.timer.is_stopped() {
                break;
            }

            best_move = match pv.first() {
                Some(mv) => *mv,
                None => Move::NULL_MOVE,
            };

            println!(
                "info depth {} score {} nodes {} pv {}",
                depth,
                Self::get_score_string(score),
                self.search_info.nodes_searched,
                Self::get_pv_string(&pv),
            );
        }

        Ok(best_move)
    }

    fn negamax(
        &mut self,
        mut depth: u8,
        alpha: i32,
        beta: i32,
        pv: &mut Vec<Move>,
        previous_move: Move,
    ) -> anyhow::Result<i32> {
        // search a bit further if in check
        if self.board.is_in_check(self.board.side_to_move()) {
            depth += 1;
        }

        if depth == 0 {
            return self.quiescence_search(alpha, beta, pv);
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

        if self.search_info.ply != 0 && self.board.is_draw() {
            return Ok(0);
        }

        let mut move_list = MoveList::default();
        self.board.generate_all_moves(&mut move_list)?;

        let mut legal_move_count = 0;
        let old_alpha = alpha;
        let mut alpha = alpha;

        let mut best_score_from_node = -INFINITY;
        let mut best_move_from_node = Move::NULL_MOVE;

        let mut pvs_enabled = false;

        self.score_moves(&mut move_list, transposition_move, previous_move);

        for i in 0..move_list.length() {
            let mv = move_list.pick_ordered_move(i);

            if !self.board.make_move(mv)? {
                self.board.unmake_move(mv)?;
                continue;
            }

            let mut current_pv = Vec::new();

            self.search_info.ply += 1;
            legal_move_count += 1;

            let score = if pvs_enabled {
                let mut pvs_score = -self.negamax(depth - 1, -alpha - 1, -alpha, pv, mv)?;

                if pvs_score > alpha && pvs_score < beta {
                    // we assumed the move would be really bad, but it wasn't, so we have to do a
                    // full-window search to verify the score
                    pvs_score = -self.negamax(depth - 1, -beta, -alpha, &mut current_pv, mv)?;
                }

                pvs_score
            } else {
                -self.negamax(depth - 1, -beta, -alpha, &mut current_pv, mv)?
            };

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

                self.store_killer_move(mv);
                self.update_history_score(mv, depth);
                self.store_counter_move(previous_move, mv);
                return Ok(beta);
            }

            if score > alpha {
                alpha = score;

                pv.clear();
                pv.push(mv);
                pv.append(&mut current_pv);

                pvs_enabled = true;
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

    fn quiescence_search(
        &mut self,
        mut alpha: i32,
        beta: i32,
        pv: &mut Vec<Move>,
    ) -> anyhow::Result<i32> {
        self.search_info.nodes_searched += 1;

        // check move time expiry every 2048 nodes
        if (self.search_info.nodes_searched & 2047) == 0 {
            self.timer.check();
        }

        if self.timer.is_stopped() {
            return Ok(0);
        }

        if self.search_info.ply >= SearchDepth::MAX {
            return Ok(evaluate(&self.board));
        }

        let stand_pat = evaluate(&self.board);

        if stand_pat >= beta {
            return Ok(beta);
        }

        alpha = alpha.max(stand_pat);

        let mut move_list = MoveList::default();
        self.board.generate_all_captures(&mut move_list)?;

        self.score_moves(&mut move_list, Move::NULL_MOVE, Move::NULL_MOVE);

        for i in 0..move_list.length() {
            let mv = move_list.pick_ordered_move(i);

            if !self.board.make_move(mv)? {
                self.board.unmake_move(mv)?;
                continue;
            }

            let mut current_pv = Vec::new();

            self.search_info.ply += 1;

            let score = -self.quiescence_search(-beta, -alpha, &mut current_pv)?;

            self.board.unmake_move(mv)?;
            self.search_info.ply -= 1;

            if score >= beta {
                return Ok(beta);
            }

            if score > alpha {
                alpha = score;

                pv.clear();
                pv.push(mv);
                pv.append(&mut current_pv);
            }
        }

        Ok(alpha)
    }

    fn score_moves(&self, move_list: &mut MoveList, transposition_move: Move, previous_move: Move) {
        for i in 0..move_list.length() {
            let mv = move_list.get_mut(i);

            let victim = self.board.get_piece(mv.to_square());

            let score = if *mv == transposition_move {
                TT_SCORE_OFFSET
            } else if victim.kind != PieceKind::NoPiece {
                let attacker = self.board.get_piece(mv.from_square());
                CAPTURE_SCORE_OFFSET + (10 * victim.material_value()) - attacker.material_value()
            } else if *mv == self.get_killer_moves()[0] {
                FIRST_KILLER_SCORE
            } else if *mv == self.get_killer_moves()[1] {
                SECOND_KILLER_SCORE
            } else {
                self.get_history_score(mv) + self.get_counter_move_bonus(previous_move, *mv)
            };

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

    fn get_pv_string(pv: &[Move]) -> String {
        let mut pv_string = String::new();

        for mv in pv {
            pv_string.push(' ');
            pv_string.push_str(&mv.to_string());
        }

        pv_string.trim().into()
    }

    fn store_killer_move(&mut self, mv: Move) {
        // quiet moves only
        if mv.kind() == MoveKind::Capture {
            return;
        }

        let ply = self.search_info.ply as usize;

        if mv != self.killer_moves[ply][0] {
            self.killer_moves[ply][1] = self.killer_moves[ply][0];
            self.killer_moves[ply][0] = mv;
        }
    }

    fn update_history_score(&mut self, mv: Move, depth: u8) {
        if mv.kind() == MoveKind::Capture {
            return;
        }

        let depth = depth as u32;

        let history = self.get_history_mut();

        history[mv.from_square().index()][mv.to_square().index()] += depth * depth;

        // ensure the score is always less than that of the killer moves
        if history[mv.from_square().index()][mv.to_square().index()] > MAX_HISTORY_SCORE as u32 {
            #[allow(clippy::needless_range_loop)]
            for from_square in 0..64 {
                for to_square in 0..64 {
                    history[from_square][to_square] /= 2;
                }
            }
        }
    }

    fn store_counter_move(&mut self, previous_move: Move, current_move: Move) {
        if current_move.kind() == MoveKind::Capture {
            return;
        }

        let counters = self.get_counter_moves_mut();

        counters[previous_move.from_square().index()][previous_move.to_square().index()] =
            current_move;
    }

    fn get_killer_moves(&self) -> &[Move] {
        &self.killer_moves[self.search_info.ply as usize]
    }

    fn get_history(&self) -> &[[u32; 64]; 64] {
        &self.history[self.board.side_to_move().index()]
    }

    fn get_history_mut(&mut self) -> &mut [[u32; 64]; 64] {
        &mut self.history[self.board.side_to_move().index()]
    }

    fn get_history_score(&self, mv: &Move) -> i32 {
        let history = self.get_history();
        history[mv.from_square().index()][mv.to_square().index()] as i32
    }

    fn get_counter_moves(&self) -> &[[Move; 64]; 64] {
        &self.counter_moves[self.board.side_to_move().index()]
    }

    fn get_counter_moves_mut(&mut self) -> &mut [[Move; 64]; 64] {
        &mut self.counter_moves[self.board.side_to_move().index()]
    }

    fn get_counter_move_bonus(&self, previous_move: Move, mv: Move) -> i32 {
        let counter = self.get_counter_moves();

        if counter[previous_move.from_square().index()][previous_move.to_square().index()] == mv {
            COUNTER_MOVE_BONUS
        } else {
            0
        }
    }
}
