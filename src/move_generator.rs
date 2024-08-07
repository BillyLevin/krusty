use std::fmt::{Debug, Display};

use crate::{
    bitboard::{Bitboard, EMPTY_BB},
    board::{Board, CastlingKind, Side},
    magics::{BISHOP_ATTACK_TABLE_SIZE, BISHOP_MAGICS, ROOK_ATTACK_TABLE_SIZE, ROOK_MAGICS},
    square::{Piece, PieceKind, Rank, Square},
};

#[derive(Debug, PartialEq, Eq)]
pub enum MoveKind {
    Quiet = 0b00,
    Capture = 0b01,
    Castle = 0b10,
    Promotion = 0b11,
}

impl From<u32> for MoveKind {
    fn from(value: u32) -> Self {
        match value {
            x if x == (MoveKind::Quiet as u32) => MoveKind::Quiet,
            x if x == (MoveKind::Capture as u32) => MoveKind::Capture,
            x if x == (MoveKind::Castle as u32) => MoveKind::Castle,
            x if x == (MoveKind::Promotion as u32) => MoveKind::Promotion,
            _ => panic!("invalid move kind"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MoveFlag {
    None = 0b000,
    EnPassant = 0b001,
    KnightPromotion = 0b010,
    BishopPromotion = 0b011,
    RookPromotion = 0b100,
    QueenPromotion = 0b101,
}

impl From<u32> for MoveFlag {
    fn from(value: u32) -> Self {
        match value {
            x if x == (MoveFlag::None as u32) => MoveFlag::None,
            x if x == (MoveFlag::EnPassant as u32) => MoveFlag::EnPassant,
            x if x == (MoveFlag::KnightPromotion as u32) => MoveFlag::KnightPromotion,
            x if x == (MoveFlag::BishopPromotion as u32) => MoveFlag::BishopPromotion,
            x if x == (MoveFlag::RookPromotion as u32) => MoveFlag::RookPromotion,
            x if x == (MoveFlag::QueenPromotion as u32) => MoveFlag::QueenPromotion,
            _ => panic!("invalid move flag"),
        }
    }
}

// 6 bits: from square
// 6 bits: to square
// 2 bits: move type
// 3 bits: move flag
// = 17 bits to represent the move
// the remaining 15 bits provide the move with a score for use in sorting
#[derive(Clone, Copy, Default)]
pub struct Move(u32);

impl Move {
    pub const NULL_MOVE: Move = Move(0);

    const SQUARE_MASK: u32 = 0b111111;
    const MOVE_KIND_MASK: u32 = 0b11;
    const MOVE_FLAG_MASK: u32 = 0b111;
    const MOVE_SCORE_MASK: u32 = 0xfffe0000;

    pub fn new(from: Square, to: Square, kind: MoveKind, flag: MoveFlag) -> Self {
        let from = from as u32;
        let to = to as u32;
        let kind = kind as u32;
        let flag = flag as u32;

        Self(from | (to << 6) | (kind << 12) | (flag << 14))
    }

    pub fn from_square(&self) -> Square {
        (self.0 & Self::SQUARE_MASK).into()
    }

    pub fn to_square(&self) -> Square {
        ((self.0 >> 6) & Self::SQUARE_MASK).into()
    }

    pub fn kind(&self) -> MoveKind {
        ((self.0 >> 12) & Self::MOVE_KIND_MASK).into()
    }

    pub fn flag(&self) -> MoveFlag {
        ((self.0 >> 14) & Self::MOVE_FLAG_MASK).into()
    }

    pub fn score(&self) -> u32 {
        (self.0 & Self::MOVE_SCORE_MASK) >> 17
    }

    pub fn set_score(&mut self, score: u32) {
        self.0 |= score << 17 & Self::MOVE_SCORE_MASK
    }

    pub fn is_null(&self) -> bool {
        *self == Self::NULL_MOVE
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_null() {
            return write!(f, "0000");
        }

        let squares = format!("{:?}{:?}", self.from_square(), self.to_square());
        write!(f, "{}", squares.to_lowercase())?;

        let promotion = match self.flag() {
            MoveFlag::KnightPromotion => "n",
            MoveFlag::BishopPromotion => "b",
            MoveFlag::RookPromotion => "r",
            MoveFlag::QueenPromotion => "q",
            _ => "",
        };

        write!(f, "{}", promotion)
    }
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        (self.0 & !Self::MOVE_SCORE_MASK) == (other.0 & !Self::MOVE_SCORE_MASK)
    }
}

impl Eq for Move {}

#[derive(Clone)]
pub struct MoveList {
    moves: Vec<Move>,
}

impl MoveList {
    pub fn new() -> Self {
        Self { moves: Vec::new() }
    }

    pub fn push(&mut self, mv: Move) {
        self.moves.push(mv);
    }

    pub fn get(&self, index: usize) -> Move {
        self.moves[index]
    }

    pub fn length(&self) -> usize {
        self.moves.len()
    }

    pub fn get_mut(&mut self, index: usize) -> &mut Move {
        &mut self.moves[index]
    }

    pub fn pick_ordered_move(&mut self, current_index: usize) -> Move {
        let mut best_index = current_index;
        let mut best_score = self.get(current_index).score();

        for index in (current_index + 1)..self.length() {
            if self.get(index).score() > best_score {
                best_index = index;
                best_score = self.get(index).score();
            }
        }

        self.swap_moves(current_index, best_index);
        self.get(current_index)
    }

    fn swap_moves(&mut self, index_a: usize, index_b: usize) {
        let tmp = self.get(index_a);
        self.moves[index_a] = self.get(index_b);
        self.moves[index_b] = tmp;
    }
}

impl Default for MoveList {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for MoveList {
    type Item = Move;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.moves.into_iter()
    }
}

pub struct MoveGenerator {
    rook_attacks: Vec<Bitboard>,
    bishop_attacks: Vec<Bitboard>,
}

const fn init_white_pawn_pushes() -> [Bitboard; 64] {
    let mut square_idx: usize = 0;

    let mut pawn_pushes: [Bitboard; 64] = [EMPTY_BB; 64];

    while square_idx < 64 {
        let square_bb = 1u64 << square_idx;
        pawn_pushes[square_idx] = Bitboard(square_bb << 8);
        square_idx += 1;
    }

    pawn_pushes
}

const NOT_A_FILE: u64 = 18374403900871474942u64;
const NOT_H_FILE: u64 = 9187201950435737471u64;
const NOT_AB_FILE: u64 = 18229723555195321596u64;
const NOT_GH_FILE: u64 = 4557430888798830399u64;

const fn init_black_pawn_pushes() -> [Bitboard; 64] {
    let mut square_idx: usize = 0;

    let mut pawn_pushes: [Bitboard; 64] = [EMPTY_BB; 64];

    while square_idx < 64 {
        let square_bb = 1u64 << square_idx;
        pawn_pushes[square_idx] = Bitboard(square_bb >> 8);
        square_idx += 1;
    }

    pawn_pushes
}

const fn init_white_pawn_attacks() -> [Bitboard; 64] {
    let mut square_idx: usize = 0;

    let mut pawn_attacks: [Bitboard; 64] = [EMPTY_BB; 64];

    while square_idx < 64 {
        let square_bb = 1u64 << square_idx;

        let north_east = (square_bb << 9) & NOT_A_FILE;
        let north_west = (square_bb << 7) & NOT_H_FILE;

        pawn_attacks[square_idx] = Bitboard(north_west | north_east);

        square_idx += 1;
    }

    pawn_attacks
}

const fn init_black_pawn_attacks() -> [Bitboard; 64] {
    let mut square_idx: usize = 0;

    let mut pawn_attacks: [Bitboard; 64] = [EMPTY_BB; 64];

    while square_idx < 64 {
        let square_bb = 1u64 << square_idx;

        let south_east = (square_bb >> 7) & NOT_A_FILE;
        let south_west = (square_bb >> 9) & NOT_H_FILE;

        pawn_attacks[square_idx] = Bitboard(south_west | south_east);

        square_idx += 1;
    }

    pawn_attacks
}

const fn init_knight_attacks() -> [Bitboard; 64] {
    let mut square_idx: usize = 0;

    let mut knight_attacks: [Bitboard; 64] = [EMPTY_BB; 64];

    while square_idx < 64 {
        let square_bb = 1u64 << square_idx;

        // NORTH: << 8
        // SOUTH: >> 8
        // EAST: << 1
        // WEST >> 1

        let north_north_east = (square_bb << 17) & NOT_A_FILE;
        let north_north_west = (square_bb << 15) & NOT_H_FILE;
        let north_east_east = (square_bb << 10) & NOT_AB_FILE;
        let north_west_west = (square_bb << 6) & NOT_GH_FILE;

        let south_south_east = (square_bb >> 15) & NOT_A_FILE;
        let south_south_west = (square_bb >> 17) & NOT_H_FILE;
        let south_east_east = (square_bb >> 6) & NOT_AB_FILE;
        let south_west_west = (square_bb >> 10) & NOT_GH_FILE;

        knight_attacks[square_idx] = Bitboard(
            north_north_east
                | north_north_west
                | north_east_east
                | north_west_west
                | south_south_east
                | south_south_west
                | south_east_east
                | south_west_west,
        );

        square_idx += 1;
    }

    knight_attacks
}

const fn north_one(bits: u64) -> u64 {
    bits << 8
}
const fn south_one(bits: u64) -> u64 {
    bits >> 8
}

const fn east_one(bits: u64) -> u64 {
    (bits << 1) & NOT_A_FILE
}

const fn west_one(bits: u64) -> u64 {
    (bits >> 1) & NOT_H_FILE
}

const fn init_king_attacks() -> [Bitboard; 64] {
    let mut square_idx: usize = 0;

    let mut king_attacks: [Bitboard; 64] = [EMPTY_BB; 64];

    while square_idx < 64 {
        // "parallel prefix" method described here: https://www.chessprogramming.org/King_Pattern#by_Calculation
        let mut king_bb = 1u64 << square_idx;
        let mut attacks = east_one(king_bb) | west_one(king_bb);
        king_bb |= attacks;
        attacks |= north_one(king_bb) | south_one(king_bb);

        king_attacks[square_idx] = Bitboard(attacks);

        square_idx += 1;
    }

    king_attacks
}

pub const ROOK_DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
pub const BISHOP_DIRECTIONS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

pub fn generate_sliding_blocker_mask(square: Square, directions: [(i32, i32); 4]) -> Bitboard {
    let mut blockers = EMPTY_BB;

    let start_rank = (square.index() / 8) as i32;
    let start_file = (square.index() % 8) as i32;

    for (rank_offset, file_offset) in directions {
        let mut rank = start_rank;
        let mut file = start_file;

        loop {
            let next_square = Square::new(
                (rank as usize).try_into().unwrap(),
                (file as usize).try_into().unwrap(),
            )
            .bitboard();

            rank += rank_offset;
            file += file_offset;

            if (0..=7).contains(&rank) && (0..=7).contains(&file) {
                blockers |= next_square;
            } else {
                break;
            }
        }
    }

    blockers.clear_bit(square);
    blockers
}

pub fn generate_sliding_attack_mask(
    square: Square,
    blockers: Bitboard,
    directions: [(i32, i32); 4],
) -> Bitboard {
    let mut attacks = EMPTY_BB;

    let start_rank = (square.index() / 8) as i32;
    let start_file = (square.index() % 8) as i32;

    for (rank_offset, file_offset) in directions {
        let mut rank = start_rank;
        let mut file = start_file;

        loop {
            let next_square = Square::new(
                (rank as usize).try_into().unwrap(),
                (file as usize).try_into().unwrap(),
            )
            .bitboard();

            rank += rank_offset;
            file += file_offset;

            attacks |= next_square;

            if !(0..=7).contains(&rank) || !(0..=7).contains(&file) {
                break;
            }

            if (blockers & next_square) != EMPTY_BB {
                break;
            }
        }
    }

    attacks.clear_bit(square);
    attacks
}

fn init_rook_attacks() -> Vec<Bitboard> {
    let mut rook_attacks = vec![EMPTY_BB; ROOK_ATTACK_TABLE_SIZE];

    for (square, magic) in ROOK_MAGICS.iter().enumerate() {
        let mask = Bitboard(magic.blocker_mask);
        let mut blockers = EMPTY_BB;

        loop {
            let moves = generate_sliding_attack_mask(square.into(), blockers, ROOK_DIRECTIONS);
            rook_attacks[magic.get_magic_index(blockers)] = moves;

            blockers = (blockers - mask) & mask;
            if blockers == EMPTY_BB {
                break;
            }
        }
    }

    rook_attacks
}

fn init_bishop_attacks() -> Vec<Bitboard> {
    let mut bishop_attacks = vec![EMPTY_BB; BISHOP_ATTACK_TABLE_SIZE];

    for (square, magic) in BISHOP_MAGICS.iter().enumerate() {
        let mask = Bitboard(magic.blocker_mask);
        let mut blockers = EMPTY_BB;

        loop {
            let moves = generate_sliding_attack_mask(square.into(), blockers, BISHOP_DIRECTIONS);
            bishop_attacks[magic.get_magic_index(blockers)] = moves;

            blockers = (blockers - mask) & mask;
            if blockers == EMPTY_BB {
                break;
            }
        }
    }

    bishop_attacks
}

// maps the `from` square to the `to` square when pushing a pawn
pub const WHITE_PAWN_PUSHES: [Bitboard; 64] = init_white_pawn_pushes();
pub const BLACK_PAWN_PUSHES: [Bitboard; 64] = init_black_pawn_pushes();

pub const WHITE_PAWN_ATTACKS: [Bitboard; 64] = init_white_pawn_attacks();
pub const BLACK_PAWN_ATTACKS: [Bitboard; 64] = init_black_pawn_attacks();

pub const KNIGHT_ATTACKS: [Bitboard; 64] = init_knight_attacks();

pub const KING_ATTACKS: [Bitboard; 64] = init_king_attacks();

pub fn pawn_attacks(side: Side) -> [Bitboard; 64] {
    match side {
        Side::White => WHITE_PAWN_ATTACKS,
        Side::Black => BLACK_PAWN_ATTACKS,
    }
}

impl MoveGenerator {
    const RANK_4_MASK: Bitboard = Bitboard(4278190080u64);
    const RANK_5_MASK: Bitboard = Bitboard(1095216660480u64);

    pub fn generate_all_moves(
        &self,
        board: &Board,
        move_list: &mut MoveList,
    ) -> anyhow::Result<()> {
        self.generate_pawn_moves(board, move_list)?;
        self.generate_king_moves(board, move_list)?;
        self.generate_castling_moves(board, move_list)?;
        self.generate_knight_moves(board, move_list)?;
        self.generate_bishop_moves(board, move_list)?;
        self.generate_rook_moves(board, move_list)?;
        self.generate_queen_moves(board, move_list)?;

        Ok(())
    }

    fn generate_pawn_moves(&self, board: &Board, move_list: &mut MoveList) -> anyhow::Result<()> {
        let empty = board.empty_squares();
        let pawn_pushes = Self::pawn_pushes(board.side_to_move());
        let mut pawns =
            board.get_piece_bb(Piece::new(board.side_to_move().into(), PieceKind::Pawn))?;

        while pawns != EMPTY_BB {
            let from_square = pawns.pop_bit();
            let mut single_push = pawn_pushes[from_square.index()] & empty;

            let mut double_push = match board.side_to_move() {
                Side::White => (single_push << 8) & Self::RANK_4_MASK & empty,
                Side::Black => (single_push >> 8) & Self::RANK_5_MASK & empty,
            };

            if single_push != EMPTY_BB {
                let to_square = single_push.pop_bit();

                if Self::is_promotion(board.side_to_move(), to_square)? {
                    Self::push_all_promotions(move_list, from_square, to_square);
                } else {
                    move_list.push(Move::new(
                        from_square,
                        to_square,
                        MoveKind::Quiet,
                        MoveFlag::None,
                    ));
                }
            }

            if double_push != EMPTY_BB {
                let to_square = double_push.pop_bit();

                move_list.push(Move::new(
                    from_square,
                    to_square,
                    MoveKind::Quiet,
                    MoveFlag::None,
                ));
            }

            let en_passant_bb = match board.en_passant_square() {
                Square::None => EMPTY_BB,
                square => square.bitboard(),
            };

            let enemy_side = match board.side_to_move() {
                Side::White => Side::Black,
                Side::Black => Side::White,
            };

            let enemy = board.occupancy(enemy_side) | en_passant_bb;
            let pawn_attack_mask = pawn_attacks(board.side_to_move())[from_square.index()];

            let mut attacks = pawn_attack_mask & enemy;

            while attacks != EMPTY_BB {
                let attacked_square = attacks.pop_bit();

                if Self::is_promotion(board.side_to_move(), attacked_square)? {
                    Self::push_all_promotions(move_list, from_square, attacked_square);
                } else {
                    let flag = if attacked_square == board.en_passant_square() {
                        MoveFlag::EnPassant
                    } else {
                        MoveFlag::None
                    };

                    move_list.push(Move::new(
                        from_square,
                        attacked_square,
                        MoveKind::Capture,
                        flag,
                    ));
                }
            }
        }

        Ok(())
    }

    fn generate_knight_moves(&self, board: &Board, move_list: &mut MoveList) -> anyhow::Result<()> {
        let mut knights =
            board.get_piece_bb(Piece::new(board.side_to_move().into(), PieceKind::Knight))?;

        let (current_side_occupancy, enemy_occupancy) = match board.side_to_move() {
            Side::White => (board.occupancy(Side::White), board.occupancy(Side::Black)),
            Side::Black => (board.occupancy(Side::Black), board.occupancy(Side::White)),
        };

        while knights != EMPTY_BB {
            let from_square = knights.pop_bit();

            let possible_attacks = KNIGHT_ATTACKS[from_square.index()];

            let mut knight_moves = possible_attacks & !current_side_occupancy;

            while knight_moves != EMPTY_BB {
                let to_square = knight_moves.pop_bit();

                let is_capture = to_square.bitboard() & enemy_occupancy != EMPTY_BB;

                let move_kind = if is_capture {
                    MoveKind::Capture
                } else {
                    MoveKind::Quiet
                };

                move_list.push(Move::new(from_square, to_square, move_kind, MoveFlag::None));
            }
        }
        Ok(())
    }

    fn generate_king_moves(&self, board: &Board, move_list: &mut MoveList) -> anyhow::Result<()> {
        let mut king =
            board.get_piece_bb(Piece::new(board.side_to_move().into(), PieceKind::King))?;

        let (current_side_occupancy, enemy_occupancy) = match board.side_to_move() {
            Side::White => (board.occupancy(Side::White), board.occupancy(Side::Black)),
            Side::Black => (board.occupancy(Side::Black), board.occupancy(Side::White)),
        };

        let from_square = king.pop_bit();

        let possible_attacks = KING_ATTACKS[from_square.index()];

        let mut king_moves = possible_attacks & !current_side_occupancy;

        while king_moves != EMPTY_BB {
            let to_square = king_moves.pop_bit();

            let is_capture = (to_square.bitboard() & enemy_occupancy) != EMPTY_BB;

            let move_kind = if is_capture {
                MoveKind::Capture
            } else {
                MoveKind::Quiet
            };

            move_list.push(Move::new(from_square, to_square, move_kind, MoveFlag::None));
        }

        Ok(())
    }

    fn generate_rook_moves(&self, board: &Board, move_list: &mut MoveList) -> anyhow::Result<()> {
        let mut rooks =
            board.get_piece_bb(Piece::new(board.side_to_move().into(), PieceKind::Rook))?;

        let (current_side_occupancy, enemy_occupancy) = match board.side_to_move() {
            Side::White => (board.occupancy(Side::White), board.occupancy(Side::Black)),
            Side::Black => (board.occupancy(Side::Black), board.occupancy(Side::White)),
        };

        while rooks != EMPTY_BB {
            let from_square = rooks.pop_bit();

            let magic = ROOK_MAGICS[from_square.index()];

            let occupancies = current_side_occupancy | enemy_occupancy;

            let possible_attacks = self.get_rook_attacks(magic.get_magic_index(occupancies));

            let mut rook_moves = possible_attacks & !current_side_occupancy;

            while rook_moves != EMPTY_BB {
                let to_square = rook_moves.pop_bit();

                let is_capture = to_square.bitboard() & enemy_occupancy != EMPTY_BB;

                let move_kind = if is_capture {
                    MoveKind::Capture
                } else {
                    MoveKind::Quiet
                };

                move_list.push(Move::new(from_square, to_square, move_kind, MoveFlag::None));
            }
        }
        Ok(())
    }

    fn generate_bishop_moves(&self, board: &Board, move_list: &mut MoveList) -> anyhow::Result<()> {
        let mut bishops =
            board.get_piece_bb(Piece::new(board.side_to_move().into(), PieceKind::Bishop))?;

        let (current_side_occupancy, enemy_occupancy) = match board.side_to_move() {
            Side::White => (board.occupancy(Side::White), board.occupancy(Side::Black)),
            Side::Black => (board.occupancy(Side::Black), board.occupancy(Side::White)),
        };

        while bishops != EMPTY_BB {
            let from_square = bishops.pop_bit();

            let magic = BISHOP_MAGICS[from_square.index()];

            let occupancies = current_side_occupancy | enemy_occupancy;

            let possible_attacks = self.get_bishop_attacks(magic.get_magic_index(occupancies));

            let mut bishop_moves = possible_attacks & !current_side_occupancy;

            while bishop_moves != EMPTY_BB {
                let to_square = bishop_moves.pop_bit();

                let is_capture = to_square.bitboard() & enemy_occupancy != EMPTY_BB;

                let move_kind = if is_capture {
                    MoveKind::Capture
                } else {
                    MoveKind::Quiet
                };

                move_list.push(Move::new(from_square, to_square, move_kind, MoveFlag::None));
            }
        }
        Ok(())
    }

    fn generate_queen_moves(&self, board: &Board, move_list: &mut MoveList) -> anyhow::Result<()> {
        let mut queens =
            board.get_piece_bb(Piece::new(board.side_to_move().into(), PieceKind::Queen))?;

        let (current_side_occupancy, enemy_occupancy) = match board.side_to_move() {
            Side::White => (board.occupancy(Side::White), board.occupancy(Side::Black)),
            Side::Black => (board.occupancy(Side::Black), board.occupancy(Side::White)),
        };

        while queens != EMPTY_BB {
            let from_square = queens.pop_bit();

            let bishop_magic = BISHOP_MAGICS[from_square.index()];
            let rook_magic = ROOK_MAGICS[from_square.index()];

            let occupancies = current_side_occupancy | enemy_occupancy;

            let possible_attacks = self
                .get_bishop_attacks(bishop_magic.get_magic_index(occupancies))
                | self.get_rook_attacks(rook_magic.get_magic_index(occupancies));

            let mut queen_moves = possible_attacks & !current_side_occupancy;

            while queen_moves != EMPTY_BB {
                let to_square = queen_moves.pop_bit();

                let is_capture = to_square.bitboard() & enemy_occupancy != EMPTY_BB;

                let move_kind = if is_capture {
                    MoveKind::Capture
                } else {
                    MoveKind::Quiet
                };

                move_list.push(Move::new(from_square, to_square, move_kind, MoveFlag::None));
            }
        }
        Ok(())
    }

    fn generate_castling_moves(
        &self,
        board: &Board,
        move_list: &mut MoveList,
    ) -> anyhow::Result<()> {
        let occupancies = board.occupancy(Side::White) | board.occupancy(Side::Black);
        let side = board.side_to_move();

        if side == Side::White {
            if board.can_castle(CastlingKind::WhiteKing)
                && !occupancies.is_occupied(Square::F1)
                && !occupancies.is_occupied(Square::G1)
                && !self.is_square_attacked(board, Square::E1, !side)
                && !self.is_square_attacked(board, Square::F1, !side)
            {
                move_list.push(Move::new(
                    Square::E1,
                    Square::G1,
                    MoveKind::Castle,
                    MoveFlag::None,
                ));
            }

            if board.can_castle(CastlingKind::WhiteQueen)
                && !occupancies.is_occupied(Square::D1)
                && !occupancies.is_occupied(Square::C1)
                && !occupancies.is_occupied(Square::B1)
                && !self.is_square_attacked(board, Square::E1, !side)
                && !self.is_square_attacked(board, Square::D1, !side)
            {
                move_list.push(Move::new(
                    Square::E1,
                    Square::C1,
                    MoveKind::Castle,
                    MoveFlag::None,
                ));
            }
        } else {
            if board.can_castle(CastlingKind::BlackKing)
                && !occupancies.is_occupied(Square::F8)
                && !occupancies.is_occupied(Square::G8)
                && !self.is_square_attacked(board, Square::E8, !side)
                && !self.is_square_attacked(board, Square::F8, !side)
            {
                move_list.push(Move::new(
                    Square::E8,
                    Square::G8,
                    MoveKind::Castle,
                    MoveFlag::None,
                ));
            }

            if board.can_castle(CastlingKind::BlackQueen)
                && !occupancies.is_occupied(Square::D8)
                && !occupancies.is_occupied(Square::C8)
                && !occupancies.is_occupied(Square::B8)
                && !self.is_square_attacked(board, Square::E8, !side)
                && !self.is_square_attacked(board, Square::D8, !side)
            {
                move_list.push(Move::new(
                    Square::E8,
                    Square::C8,
                    MoveKind::Castle,
                    MoveFlag::None,
                ));
            }
        }

        Ok(())
    }

    fn pawn_pushes(side: Side) -> [Bitboard; 64] {
        match side {
            Side::White => WHITE_PAWN_PUSHES,
            Side::Black => BLACK_PAWN_PUSHES,
        }
    }

    fn push_all_promotions(move_list: &mut MoveList, from: Square, to: Square) {
        move_list.push(Move::new(
            from,
            to,
            MoveKind::Promotion,
            MoveFlag::KnightPromotion,
        ));
        move_list.push(Move::new(
            from,
            to,
            MoveKind::Promotion,
            MoveFlag::BishopPromotion,
        ));
        move_list.push(Move::new(
            from,
            to,
            MoveKind::Promotion,
            MoveFlag::RookPromotion,
        ));
        move_list.push(Move::new(
            from,
            to,
            MoveKind::Promotion,
            MoveFlag::QueenPromotion,
        ));
    }

    fn is_promotion(side: Side, to_square: Square) -> anyhow::Result<bool> {
        match side {
            Side::White => Ok(to_square.rank()? == Rank::Eighth),
            Side::Black => Ok(to_square.rank()? == Rank::First),
        }
    }

    pub fn is_square_attacked(&self, board: &Board, square: Square, attacker_side: Side) -> bool {
        let pawns = board
            .get_piece_bb(Piece::new(attacker_side.into(), PieceKind::Pawn))
            .unwrap();

        if pawn_attacks(!attacker_side)[square.index()] & pawns != EMPTY_BB {
            return true;
        }

        let king = board
            .get_piece_bb(Piece::new(attacker_side.into(), PieceKind::King))
            .unwrap();

        if KING_ATTACKS[square.index()] & king != EMPTY_BB {
            return true;
        }

        let knights = board
            .get_piece_bb(Piece::new(attacker_side.into(), PieceKind::Knight))
            .unwrap();

        if KNIGHT_ATTACKS[square.index()] & knights != EMPTY_BB {
            return true;
        }

        let occupancies = board.occupancy(Side::White) | board.occupancy(Side::Black);

        let bishops = board
            .get_piece_bb(Piece::new(attacker_side.into(), PieceKind::Bishop))
            .unwrap();

        let bishop_magic = BISHOP_MAGICS[square.index()];
        let bishop_attacks = self.get_bishop_attacks(bishop_magic.get_magic_index(occupancies));

        if bishop_attacks & bishops != EMPTY_BB {
            return true;
        }

        let rooks = board
            .get_piece_bb(Piece::new(attacker_side.into(), PieceKind::Rook))
            .unwrap();

        let rook_magic = ROOK_MAGICS[square.index()];
        let rook_attacks = self.get_rook_attacks(rook_magic.get_magic_index(occupancies));

        if rook_attacks & rooks != EMPTY_BB {
            return true;
        }

        let queens = board
            .get_piece_bb(Piece::new(attacker_side.into(), PieceKind::Queen))
            .unwrap();

        if (rook_attacks | bishop_attacks) & queens != EMPTY_BB {
            return true;
        }

        false
    }

    pub fn generate_all_captures(
        &self,
        board: &Board,
        move_list: &mut MoveList,
    ) -> Result<(), anyhow::Error> {
        self.generate_pawn_captures(board, move_list)?;
        self.generate_king_captures(board, move_list)?;
        self.generate_knight_captures(board, move_list)?;
        self.generate_bishop_captures(board, move_list)?;
        self.generate_rook_captures(board, move_list)?;
        self.generate_queen_captures(board, move_list)?;

        Ok(())
    }

    fn generate_pawn_captures(
        &self,
        board: &Board,
        move_list: &mut MoveList,
    ) -> anyhow::Result<()> {
        let mut pawns =
            board.get_piece_bb(Piece::new(board.side_to_move().into(), PieceKind::Pawn))?;

        while pawns != EMPTY_BB {
            let from_square = pawns.pop_bit();

            let en_passant_bb = match board.en_passant_square() {
                Square::None => EMPTY_BB,
                square => square.bitboard(),
            };

            let enemy_side = match board.side_to_move() {
                Side::White => Side::Black,
                Side::Black => Side::White,
            };

            let enemy = board.occupancy(enemy_side) | en_passant_bb;
            let pawn_attack_mask = pawn_attacks(board.side_to_move())[from_square.index()];

            let mut attacks = pawn_attack_mask & enemy;

            while attacks != EMPTY_BB {
                let attacked_square = attacks.pop_bit();

                if Self::is_promotion(board.side_to_move(), attacked_square)? {
                    Self::push_all_promotions(move_list, from_square, attacked_square);
                } else {
                    let flag = if attacked_square == board.en_passant_square() {
                        MoveFlag::EnPassant
                    } else {
                        MoveFlag::None
                    };

                    move_list.push(Move::new(
                        from_square,
                        attacked_square,
                        MoveKind::Capture,
                        flag,
                    ));
                }
            }
        }

        Ok(())
    }

    fn generate_knight_captures(
        &self,
        board: &Board,
        move_list: &mut MoveList,
    ) -> anyhow::Result<()> {
        let mut knights =
            board.get_piece_bb(Piece::new(board.side_to_move().into(), PieceKind::Knight))?;

        let (current_side_occupancy, enemy_occupancy) = match board.side_to_move() {
            Side::White => (board.occupancy(Side::White), board.occupancy(Side::Black)),
            Side::Black => (board.occupancy(Side::Black), board.occupancy(Side::White)),
        };

        while knights != EMPTY_BB {
            let from_square = knights.pop_bit();

            let possible_attacks = KNIGHT_ATTACKS[from_square.index()];

            let mut knight_moves = possible_attacks & !current_side_occupancy;

            while knight_moves != EMPTY_BB {
                let to_square = knight_moves.pop_bit();

                let is_capture = to_square.bitboard() & enemy_occupancy != EMPTY_BB;

                if is_capture {
                    move_list.push(Move::new(
                        from_square,
                        to_square,
                        MoveKind::Capture,
                        MoveFlag::None,
                    ));
                }
            }
        }
        Ok(())
    }

    fn generate_king_captures(
        &self,
        board: &Board,
        move_list: &mut MoveList,
    ) -> anyhow::Result<()> {
        let mut king =
            board.get_piece_bb(Piece::new(board.side_to_move().into(), PieceKind::King))?;

        let (current_side_occupancy, enemy_occupancy) = match board.side_to_move() {
            Side::White => (board.occupancy(Side::White), board.occupancy(Side::Black)),
            Side::Black => (board.occupancy(Side::Black), board.occupancy(Side::White)),
        };

        let from_square = king.pop_bit();

        let possible_attacks = KING_ATTACKS[from_square.index()];

        let mut king_moves = possible_attacks & !current_side_occupancy;

        while king_moves != EMPTY_BB {
            let to_square = king_moves.pop_bit();

            let is_capture = (to_square.bitboard() & enemy_occupancy) != EMPTY_BB;

            if is_capture {
                move_list.push(Move::new(
                    from_square,
                    to_square,
                    MoveKind::Capture,
                    MoveFlag::None,
                ));
            }
        }

        Ok(())
    }

    fn generate_rook_captures(
        &self,
        board: &Board,
        move_list: &mut MoveList,
    ) -> anyhow::Result<()> {
        let mut rooks =
            board.get_piece_bb(Piece::new(board.side_to_move().into(), PieceKind::Rook))?;

        let (current_side_occupancy, enemy_occupancy) = match board.side_to_move() {
            Side::White => (board.occupancy(Side::White), board.occupancy(Side::Black)),
            Side::Black => (board.occupancy(Side::Black), board.occupancy(Side::White)),
        };

        while rooks != EMPTY_BB {
            let from_square = rooks.pop_bit();

            let magic = ROOK_MAGICS[from_square.index()];

            let occupancies = current_side_occupancy | enemy_occupancy;

            let possible_attacks = self.get_rook_attacks(magic.get_magic_index(occupancies));

            let mut rook_moves = possible_attacks & !current_side_occupancy;

            while rook_moves != EMPTY_BB {
                let to_square = rook_moves.pop_bit();

                let is_capture = to_square.bitboard() & enemy_occupancy != EMPTY_BB;

                if is_capture {
                    move_list.push(Move::new(
                        from_square,
                        to_square,
                        MoveKind::Capture,
                        MoveFlag::None,
                    ));
                }
            }
        }
        Ok(())
    }

    fn generate_bishop_captures(
        &self,
        board: &Board,
        move_list: &mut MoveList,
    ) -> anyhow::Result<()> {
        let mut bishops =
            board.get_piece_bb(Piece::new(board.side_to_move().into(), PieceKind::Bishop))?;

        let (current_side_occupancy, enemy_occupancy) = match board.side_to_move() {
            Side::White => (board.occupancy(Side::White), board.occupancy(Side::Black)),
            Side::Black => (board.occupancy(Side::Black), board.occupancy(Side::White)),
        };

        while bishops != EMPTY_BB {
            let from_square = bishops.pop_bit();

            let magic = BISHOP_MAGICS[from_square.index()];

            let occupancies = current_side_occupancy | enemy_occupancy;

            let possible_attacks = self.get_bishop_attacks(magic.get_magic_index(occupancies));

            let mut bishop_moves = possible_attacks & !current_side_occupancy;

            while bishop_moves != EMPTY_BB {
                let to_square = bishop_moves.pop_bit();

                let is_capture = to_square.bitboard() & enemy_occupancy != EMPTY_BB;

                if is_capture {
                    move_list.push(Move::new(
                        from_square,
                        to_square,
                        MoveKind::Capture,
                        MoveFlag::None,
                    ));
                }
            }
        }
        Ok(())
    }

    fn generate_queen_captures(
        &self,
        board: &Board,
        move_list: &mut MoveList,
    ) -> anyhow::Result<()> {
        let mut queens =
            board.get_piece_bb(Piece::new(board.side_to_move().into(), PieceKind::Queen))?;

        let (current_side_occupancy, enemy_occupancy) = match board.side_to_move() {
            Side::White => (board.occupancy(Side::White), board.occupancy(Side::Black)),
            Side::Black => (board.occupancy(Side::Black), board.occupancy(Side::White)),
        };

        while queens != EMPTY_BB {
            let from_square = queens.pop_bit();

            let bishop_magic = BISHOP_MAGICS[from_square.index()];
            let rook_magic = ROOK_MAGICS[from_square.index()];

            let occupancies = current_side_occupancy | enemy_occupancy;

            let possible_attacks = self
                .get_bishop_attacks(bishop_magic.get_magic_index(occupancies))
                | self.get_rook_attacks(rook_magic.get_magic_index(occupancies));

            let mut queen_moves = possible_attacks & !current_side_occupancy;

            while queen_moves != EMPTY_BB {
                let to_square = queen_moves.pop_bit();

                let is_capture = to_square.bitboard() & enemy_occupancy != EMPTY_BB;

                if is_capture {
                    move_list.push(Move::new(
                        from_square,
                        to_square,
                        MoveKind::Capture,
                        MoveFlag::None,
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn get_bishop_attacks(&self, magic_index: usize) -> Bitboard {
        self.bishop_attacks[magic_index]
    }

    pub fn get_rook_attacks(&self, magic_index: usize) -> Bitboard {
        self.rook_attacks[magic_index]
    }
}

impl Default for MoveGenerator {
    fn default() -> Self {
        Self {
            rook_attacks: init_rook_attacks(),
            bishop_attacks: init_bishop_attacks(),
        }
    }
}

impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "from: {:?}, to: {:?}, kind: {:?}, flag: {:?}",
            self.from_square(),
            self.to_square(),
            self.kind(),
            self.flag()
        )
    }
}

impl Debug for MoveList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;

        for i in 0..self.length() {
            writeln!(f, "{:?}", self.moves[i])?;
        }

        writeln!(f, "Move list size: {}", self.length())
    }
}
