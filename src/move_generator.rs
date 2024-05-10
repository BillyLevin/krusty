use std::fmt::Debug;

use crate::{
    bitboard::{Bitboard, EMPTY_BB},
    board::{Board, Side},
    square::{Piece, PieceKind, Rank, Square},
};

#[derive(Debug)]
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

#[derive(Debug)]
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
// the remaining 15 bits will be used for move ordering later on
#[derive(Clone, Copy)]
pub struct Move(u32);

impl Move {
    pub const NULL_MOVE: Move = Move(0);

    const SQUARE_MASK: u32 = 0b00111111;
    const MOVE_KIND_MASK: u32 = 0b00000011;
    const MOVE_FLAG_MASK: u32 = 0b00000111;

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
}

const MAX_MOVES: usize = 512;

#[derive(Clone, Copy)]
pub struct MoveList {
    moves: [Move; MAX_MOVES],
    count: usize,
}

impl MoveList {
    pub fn new() -> Self {
        Self {
            moves: [Move::NULL_MOVE; 512],
            count: 0,
        }
    }

    pub fn push(&mut self, mv: Move) {
        self.moves[self.count] = mv;
        self.count += 1;
    }
}

impl Default for MoveList {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MoveGenerator;

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

impl MoveGenerator {
    // maps the `from` square to the `to` square when pushing a pawn
    const WHITE_PAWN_PUSHES: [Bitboard; 64] = init_white_pawn_pushes();
    const BLACK_PAWN_PUSHES: [Bitboard; 64] = init_black_pawn_pushes();

    const WHITE_PAWN_ATTACKS: [Bitboard; 64] = init_white_pawn_attacks();
    const BLACK_PAWN_ATTACKS: [Bitboard; 64] = init_black_pawn_attacks();

    const RANK_4_MASK: u64 = 4278190080u64;
    const RANK_5_MASK: u64 = 1095216660480u64;

    pub fn generate_pawn_moves(
        &self,
        board: &Board,
        move_list: &mut MoveList,
    ) -> anyhow::Result<()> {
        let empty = board.empty_squares();
        let pawn_pushes = Self::pawn_pushes(board.side_to_move());
        let mut pawns =
            board.get_piece_bb(Piece::new(board.side_to_move().into(), PieceKind::Pawn))?;

        while pawns.value() != 0 {
            let from_square = pawns.pop_bit();
            let mut single_push =
                Bitboard(pawn_pushes[from_square.index()].value() & empty.value());

            let mut double_push = match board.side_to_move() {
                Side::White => {
                    Bitboard((single_push.value() << 8) & Self::RANK_4_MASK & empty.value())
                }
                Side::Black => {
                    Bitboard((single_push.value() >> 8) & Self::RANK_5_MASK & empty.value())
                }
            };

            if single_push.value() != 0 {
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

            if double_push.value() != 0 {
                let to_square = double_push.pop_bit();

                move_list.push(Move::new(
                    from_square,
                    to_square,
                    MoveKind::Quiet,
                    MoveFlag::None,
                ));
            }

            let en_passant_bb = match board.en_passant_square {
                Square::None => EMPTY_BB,
                square => square.bitboard(),
            };

            let enemy_side = match board.side_to_move() {
                Side::White => Side::Black,
                Side::Black => Side::White,
            };

            let enemy = board.occupancy(enemy_side).value() | en_passant_bb.value();
            let pawn_attack_mask =
                Self::pawn_attacks(board.side_to_move())[from_square.index()].value();

            let mut attacks = Bitboard(pawn_attack_mask & enemy);

            while attacks.value() != 0 {
                let attacked_square = attacks.pop_bit();

                if Self::is_promotion(board.side_to_move(), attacked_square)? {
                    Self::push_all_promotions(move_list, from_square, attacked_square);
                } else {
                    move_list.push(Move::new(
                        from_square,
                        attacked_square,
                        MoveKind::Capture,
                        MoveFlag::None,
                    ));
                }
            }
        }

        Ok(())
    }

    fn pawn_pushes(side: Side) -> [Bitboard; 64] {
        match side {
            Side::White => Self::WHITE_PAWN_PUSHES,
            Side::Black => Self::BLACK_PAWN_PUSHES,
        }
    }

    fn pawn_attacks(side: Side) -> [Bitboard; 64] {
        match side {
            Side::White => Self::WHITE_PAWN_ATTACKS,
            Side::Black => Self::BLACK_PAWN_ATTACKS,
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

        for i in 0..self.count {
            writeln!(f, "{:?}", self.moves[i])?;
        }

        writeln!(f, "Move list size: {}", self.count)
    }
}
