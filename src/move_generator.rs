use std::fmt::Debug;

use crate::{
    bitboard::{Bitboard, EMPTY_BB},
    board::{Board, Side},
    square::{Piece, PieceKind, Square},
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

pub struct MoveList;

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

impl MoveGenerator {
    // maps the `from` square to the `to` square when pushing a pawn
    const WHITE_PAWN_PUSHES: [Bitboard; 64] = init_white_pawn_pushes();
    const BLACK_PAWN_PUSHES: [Bitboard; 64] = init_black_pawn_pushes();

    const RANK_4_MASK: u64 = 4278190080u64;
    const RANK_5_MASK: u64 = 1095216660480u64;

    fn pawn_pushes(&self, side: Side) -> [Bitboard; 64] {
        match side {
            Side::White => Self::WHITE_PAWN_PUSHES,
            Side::Black => Self::BLACK_PAWN_PUSHES,
        }
    }

    pub fn generate_pawn_moves(&self, board: &Board) -> anyhow::Result<()> {
        let empty = board.empty_squares();
        let pawn_pushes = self.pawn_pushes(board.side_to_move());
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
                let current_move =
                    Move::new(from_square, to_square, MoveKind::Quiet, MoveFlag::None);
                dbg!(current_move);
            }

            if double_push.value() != 0 {
                let to_square = double_push.pop_bit();
                let current_move =
                    Move::new(from_square, to_square, MoveKind::Quiet, MoveFlag::None);
                dbg!(current_move);
            }
        }

        Ok(())
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
