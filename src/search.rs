use crate::{
    board::{Board, Side, START_POSITION_FEN},
    evaluate::evaluate,
    move_generator::{Move, MoveList},
    transposition_table::{SearchTableEntry, TranspositionTable},
};

#[derive(Debug, Clone, Copy)]
pub enum SearchDepth {
    Finite(u8),
    Infinite,
}

#[derive(Debug, Clone, Copy)]
pub enum TimeRemaining {
    Finite(u64),
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

#[derive(Debug, Clone, Copy)]
pub struct SearchInfo {
    pub depth: SearchDepth,
    pub time_remaining: TimeRemaining,
}

pub struct Search {
    transposition_table: TranspositionTable<SearchTableEntry>,
    pub board: Board,

    search_info: SearchInfo,
}

impl Default for SearchInfo {
    fn default() -> Self {
        Self {
            depth: SearchDepth::Infinite,
            time_remaining: TimeRemaining::Infinite,
        }
    }
}

impl Default for Search {
    fn default() -> Self {
        let mut board = Board::default();
        board.parse_fen(START_POSITION_FEN).unwrap();

        Self {
            transposition_table: TranspositionTable::new(64),
            board,
            search_info: SearchInfo::default(),
        }
    }
}

impl Search {
    pub fn search_position(&mut self, depth: SearchDepth) -> anyhow::Result<Option<Move>> {
        let depth: u8 = depth.into();

        let mut best_score = i32::MIN;
        let mut best_move = None;

        let mut move_list = MoveList::default();
        self.board.generate_all_moves(&mut move_list)?;

        for mv in move_list {
            if self.board.make_move(mv)? {
                let score = self.minimax(depth - 1)?;
                if score > best_score {
                    best_score = score;
                    best_move = Some(mv);
                }
            }

            self.board.unmake_move(mv)?;
        }

        println!("info depth {} score {}", depth, best_score);
        Ok(best_move)
    }

    fn minimax(&mut self, depth: u8) -> anyhow::Result<i32> {
        if depth == 0 {
            return Ok(evaluate(&self.board));
        }

        let table_entry = self.transposition_table.probe(self.board.hash());
        if table_entry.hash == self.board.hash() && table_entry.depth == depth {
            return Ok(table_entry.score);
        }

        let mut move_list = MoveList::default();
        self.board.generate_all_moves(&mut move_list)?;

        let is_maximizing = self.board.side_to_move() == Side::White;

        if is_maximizing {
            let mut best_score = i32::MIN;

            for mv in move_list {
                if self.board.make_move(mv)? {
                    let score = self.minimax(depth - 1)?;
                    best_score = best_score.max(score);
                }
                self.board.unmake_move(mv)?;
            }

            self.transposition_table.store(SearchTableEntry::new(
                self.board.hash(),
                depth,
                best_score,
            ));

            Ok(best_score)
        } else {
            let mut best_score = i32::MAX;

            for mv in move_list {
                if self.board.make_move(mv)? {
                    let score = self.minimax(depth - 1)?;
                    best_score = best_score.min(score);
                }
                self.board.unmake_move(mv)?;
            }

            self.transposition_table.store(SearchTableEntry::new(
                self.board.hash(),
                depth,
                best_score,
            ));

            Ok(best_score)
        }
    }

    pub fn set_search_info(&mut self, search_info: SearchInfo) {
        self.search_info = search_info
    }
}
