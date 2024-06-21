use crate::{
    board::{Board, Side},
    square::{Piece, PieceColor, PieceKind},
};

pub const PAWN_VALUE: i32 = 100;
pub const KNIGHT_VALUE: i32 = 300;
pub const BISHOP_VALUE: i32 = 300;
pub const ROOK_VALUE: i32 = 500;
pub const QUEEN_VALUE: i32 = 900;
// TODO: does this ever need to have a proper value?
// keeping it at 0 for now so it doesn't affect MVV-LVA
pub const KING_VALUE: i32 = 0;

impl Piece {
    pub fn material_value(&self) -> i32 {
        match self.kind {
            PieceKind::Pawn => PAWN_VALUE,
            PieceKind::Knight => KNIGHT_VALUE,
            PieceKind::Bishop => BISHOP_VALUE,
            PieceKind::Rook => ROOK_VALUE,
            PieceKind::Queen => QUEEN_VALUE,
            PieceKind::King => KING_VALUE,
            PieceKind::NoPiece => 0,
        }
    }

    pub fn pst_value(&self, square: usize) -> i32 {
        match self.kind {
            PieceKind::Pawn => PAWN_PST[square],
            PieceKind::Knight => KNIGHT_PST[square],
            PieceKind::Bishop => BISHOP_PST[square],
            PieceKind::Rook => ROOK_PST[square],
            PieceKind::King => KING_PST[square],
            // TODO: harder to determine where queen should be aiming to go. should add this after
            // doing some tuning
            PieceKind::Queen => 0,
            PieceKind::NoPiece => 0,
        }
    }
}

#[rustfmt::skip]
const PAWN_PST: [i32; 64] = [
   0,  0,  0,  0,  0,  0,  0,  0,
  50, 50, 50, 50, 50, 50, 50, 50,
  10, 10, 20, 30, 30, 20, 10, 10,
   5,  5, 10, 25, 25, 10,  5,  5,
   0,  0,  0, 20, 20,  0,  0,  0,
   5, -5,-10,  0,  0,-10, -5,  5,
   5, 10, 10,-20,-20, 10, 10,  5,
   0,  0,  0,  0,  0,  0,  0,  0,
];

#[rustfmt::skip]
const KNIGHT_PST: [i32; 64] = [
  -50,-40,-30,-30,-30,-30,-40,-50,
  -40,-20,  0,  0,  0,  0,-20,-40,
  -30,  0, 10, 15, 15, 10,  0,-30,
  -30,  5, 15, 20, 20, 15,  5,-30,
  -30,  0, 15, 20, 20, 15,  0,-30,
  -30,  5, 10, 15, 15, 10,  5,-30,
  -40,-20,  0,  5,  5,  0,-20,-40,
  -50,-40,-30,-30,-30,-30,-40,-50,
];

#[rustfmt::skip]
const BISHOP_PST: [i32; 64] = [
  -20,-10,-10,-10,-10,-10,-10,-20,
  -10,  0,  0,  0,  0,  0,  0,-10,
  -10,  0,  5, 10, 10,  5,  0,-10,
  -10,  5,  5, 10, 10,  5,  5,-10,
  -10,  0, 10, 10, 10, 10,  0,-10,
  -10, 10, 10, 10, 10, 10, 10,-10,
  -10,  5,  0,  0,  0,  0,  5,-10,
  -20,-10,-10,-10,-10,-10,-10,-20,
];

#[rustfmt::skip]
const ROOK_PST: [i32; 64] =[
  0,  0,  0,  0,  0,  0,  0,  0,
  5, 10, 10, 10, 10, 10, 10,  5,
 -5,  0,  0,  0,  0,  0,  0, -5,
 -5,  0,  0,  0,  0,  0,  0, -5,
 -5,  0,  0,  0,  0,  0,  0, -5,
 -5,  0,  0,  0,  0,  0,  0, -5,
 -5,  0,  0,  0,  0,  0,  0, -5,
  0,  0,  0,  5,  5,  0,  0,  0,
];

#[rustfmt::skip]
const KING_PST: [i32; 64] =[
  -30,-40,-40,-50,-50,-40,-40,-30,
  -30,-40,-40,-50,-50,-40,-40,-30,
  -30,-40,-40,-50,-50,-40,-40,-30,
  -30,-40,-40,-50,-50,-40,-40,-30,
  -20,-30,-30,-40,-40,-30,-30,-20,
  -10,-20,-20,-20,-20,-20,-20,-10,
   20, 20,  0,  0,  0,  0, 20, 20,
   20, 30, 10,  0,  0, 10, 30, 20,
];

#[rustfmt::skip]
// this ensures that the piece-square tables are from the perspective of the current player. this
// only needs to be used for white
const FLIP_SQUARE : [usize; 64] = [
    56, 57, 58, 59, 60, 61, 62, 63,
    48, 49, 50, 51, 52, 53, 54, 55,
    40, 41, 42, 43, 44, 45, 46, 47,
    32, 33, 34, 35, 36, 37, 38, 39,
    24, 25, 26, 27, 28, 29, 30, 31,
    16, 17, 18, 19, 20, 21, 22, 23,
     8,  9, 10, 11, 12, 13, 14, 15,
     0,  1,  2,  3,  4,  5,  6,  7,
];

pub fn evaluate(board: &Board) -> i32 {
    if board.has_insufficient_material() {
        return 0;
    }

    let mut white_score = 0;
    let mut black_score = 0;

    for (square_index, piece) in board.pieces().iter().enumerate() {
        if piece.kind == PieceKind::NoPiece {
            continue;
        }

        match piece.color {
            PieceColor::White => {
                white_score += piece.material_value();
                white_score += piece.pst_value(FLIP_SQUARE[square_index]);
            }
            PieceColor::Black => {
                black_score += piece.material_value();
                black_score += piece.pst_value(square_index);
            }
            PieceColor::None => panic!("found a piece with no color"),
        };
    }

    let multiplier = match board.side_to_move() {
        Side::White => 1,
        Side::Black => -1,
    };

    multiplier * (white_score - black_score)
}
