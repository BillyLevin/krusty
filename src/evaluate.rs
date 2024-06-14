use crate::{
    board::{Board, Side},
    square::{Piece, PieceColor, PieceKind},
};

pub const PAWN_VALUE: i32 = 100;
pub const KNIGHT_VALUE: i32 = 300;
pub const BISHOP_VALUE: i32 = 300;
pub const ROOK_VALUE: i32 = 500;
pub const QUEEN_VALUE: i32 = 900;
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
            _ => 0,
        }
    }
}

pub fn evaluate(board: &Board) -> i32 {
    let mut white_score = 0;
    let mut black_score = 0;

    for piece in board.pieces() {
        if piece.kind == PieceKind::NoPiece {
            continue;
        }

        match piece.color {
            PieceColor::White => white_score += piece.material_value(),
            PieceColor::Black => black_score += piece.material_value(),
            PieceColor::None => panic!("found a piece with no color"),
        };
    }

    let multiplier = match board.side_to_move() {
        Side::White => 1,
        Side::Black => -1,
    };

    multiplier * (white_score - black_score)
}
