use crate::square::Square;

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
