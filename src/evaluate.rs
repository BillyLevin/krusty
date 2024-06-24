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

    fn middle_game_pst_value(&self, square: usize) -> i32 {
        match self.kind {
            PieceKind::Pawn => MIDDLE_GAME_PAWN_PST[square],
            PieceKind::Knight => MIDDLE_GAME_KNIGHT_PST[square],
            PieceKind::Bishop => MIDDLE_GAME_BISHOP_PST[square],
            PieceKind::Rook => MIDDLE_GAME_ROOK_PST[square],
            PieceKind::King => MIDDLE_GAME_KING_PST[square],
            // TODO: harder to determine where queen should be aiming to go. should add this after
            // doing some tuning
            PieceKind::Queen => 0,
            PieceKind::NoPiece => 0,
        }
    }

    fn end_game_pst_value(&self, square: usize) -> i32 {
        match self.kind {
            PieceKind::Pawn => END_GAME_PAWN_PST[square],
            PieceKind::Knight => END_GAME_KNIGHT_PST[square],
            PieceKind::Bishop => END_GAME_BISHOP_PST[square],
            PieceKind::Rook => END_GAME_ROOK_PST[square],
            PieceKind::King => END_GAME_KING_PST[square],
            // TODO: harder to determine where queen should be aiming to go. should add this after
            // doing some tuning
            PieceKind::Queen => 0,
            PieceKind::NoPiece => 0,
        }
    }
}

#[rustfmt::skip]
const MIDDLE_GAME_PAWN_PST: [i32; 64] = [
   0,   0,   0,   0,   0,   0,   0,   0,
  50,  50,  50,  50,  50,  50,  50,  50,
  10,  10,  20,  30,  30,  20,  10,  10,
   5,   5,  10,  25,  25,  10,   5,   5,
   0,   0,   0,  20,  20,   0,   0,   0,
   5,  -5, -10,   0,   0, -10,  -5,   5,
   5,  10,  10, -20, -20,  10,  10,   5,
   0,   0,   0,   0,   0,   0,   0,   0,
];

#[rustfmt::skip]
const END_GAME_PAWN_PST: [i32; 64] = [
      0,   0,   0,   0,   0,   0,   0,   0,
     75,  72,  65,  65,  65,  65,  72,  75,
     20,  10,  10,  10,  10,  10,  10,  20,
     -5,  -5,  -5,  -5,  -5,  -5,  -5,  -5,
    -10, -10, -10, -10, -10, -10, -10, -10,
    -10, -10, -10, -10, -10, -10, -10, -10,
    -10, -10, -10, -10, -10, -10, -10, -10,
      0,   0,   0,   0,   0,   0,   0,   0,
];

#[rustfmt::skip]
const MIDDLE_GAME_KNIGHT_PST: [i32; 64] = [
  -50, -40, -30, -30, -30, -30, -40, -50,
  -40, -20,   0,   0,   0,   0, -20, -40,
  -30,   0,  10,  15,  15,  10,   0, -30,
  -30,   5,  15,  20,  20,  15,   5, -30,
  -30,   0,  15,  20,  20,  15,   0, -30,
  -30,   5,  10,  15,  15,  10,   5, -30,
  -40, -20,   0,   5,   5,   0, -20, -40,
  -50, -40, -30, -30, -30, -30, -40, -50,
];

#[rustfmt::skip]
const END_GAME_KNIGHT_PST: [i32; 64] = [
  -50, -40, -30, -30, -30, -30, -40, -50,
  -40, -20,   0,   0,   0,   0, -20, -40,
  -30,   0,  10,  20,  20,  10,   0, -30,
  -30,   5,  15,  25,  25,  15,   5, -30,
  -30,   0,  15,  25,  25,  15,   0, -30,
  -30,   5,  10,  20,  20,  10,   5, -30,
  -40, -20,   0,   5,   5,   0, -20, -40,
  -50, -40, -30, -30, -30, -30, -40, -50,
];

#[rustfmt::skip]
const MIDDLE_GAME_BISHOP_PST: [i32; 64] = [
  -20, -10, -10, -10, -10, -10, -10, -20,
  -10,   0,   0,   0,   0,   0,   0, -10,
  -10,   0,   5,  10,  10,   5,   0, -10,
  -10,   5,   5,  25,  25,   5,   5, -10,
  -10,   0,  10,  18,  18,  10,   0, -10,
  -10,  10,  10,  10,  10,  10,  10, -10,
  -10,   5,   0,   0,   0,   0,   5, -10,
  -20, -10, -10, -10, -10, -10, -10, -20,
];

#[rustfmt::skip]
const END_GAME_BISHOP_PST: [i32; 64] = [
  -20, -10, -10, -10, -10, -10, -10, -20,
  -10,   0,   0,   0,   0,   0,   0, -10,
  -10,   0,   5,  10,  10,   5,   0, -10,
  -10,   5,   5,  10,  10,   5,   5, -10,
  -10,   0,  10,  10,  10,  10,   0, -10,
  -10,  10,  10,  10,  10,  10,  10, -10,
  -10,   7,   0,   0,   0,   0,   7, -10,
  -20, -10, -10, -10, -10, -10, -10, -20,
];

#[rustfmt::skip]
const MIDDLE_GAME_ROOK_PST: [i32; 64] =[
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
const END_GAME_ROOK_PST: [i32; 64] =[
  20,  20,  20,  20,  20,  20,  20,  20,
   8,   8,   8,   8,   8,   8,   8,   8,
  -5,   0,   0,   0,   0,   0,   0,  -5,
  -5,   0,   0,   0,   0,   0,   0,  -5,
  -5,   0,   0,   0,   0,   0,   0,  -5,
  -5,   0,   0,   0,   0,   0,   0,  -5,
  -5,   0,   0,   0,   0,   0,   0,  -5,
   0,   0,   0,   5,   5,   0,   0,   0,
];

#[rustfmt::skip]
const MIDDLE_GAME_KING_PST: [i32; 64] = [
  -30, -40, -40, -50, -50, -40, -40, -30,
  -30, -40, -40, -50, -50, -40, -40, -30,
  -30, -40, -40, -50, -50, -40, -40, -30,
  -30, -40, -40, -50, -50, -40, -40, -30,
  -20, -30, -30, -40, -40, -30, -30, -20,
  -10, -20, -20, -20, -20, -20, -20, -10,
   20,  20,   0,   0,   0,   0,  20,  20,
   20,  30,  10,   0,   0,  10,  30,  20,
];

#[rustfmt::skip]
const END_GAME_KING_PST: [i32; 64] = [
    -50, -40, -30, -20, -20, -30, -40, -50,
    -30, -20, -10,   0,   0, -10, -20, -30,
    -30, -10,  20,  30,  30,  20, -10, -30,
    -30, -10,  30,  40,  40,  30, -10, -30,
    -30, -10,  30,  40,  40,  30, -10, -30,
    -30, -10,  20,  30,  30,  20, -10, -30,
    -30, -30,   0,   0,   0,   0, -30, -30,
    -50, -30, -30, -30, -30, -30, -30, -50,
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

const KNIGHT_PHASE: i32 = 1;
const BISHOP_PHASE: i32 = 1;
const ROOK_PHASE: i32 = 2;
const QUEEN_PHASE: i32 = 4;
const TOTAL_PHASE: i32 = KNIGHT_PHASE * 4 + BISHOP_PHASE * 4 + ROOK_PHASE * 4 + QUEEN_PHASE * 2;

const BISHOP_PAIR_MIDDLE_GAME_BONUS: i32 = 25;
const BISHOP_PAIR_END_GAME_BONUS: i32 = 50;

impl Board {
    pub fn evaluate(&self) -> i32 {
        if self.has_insufficient_material() {
            return 0;
        }

        let mut white_score = 0;
        let mut black_score = 0;

        let mut white_middle_game_score = 0;
        let mut black_middle_game_score = 0;

        let mut white_end_game_score = 0;
        let mut black_end_game_score = 0;

        for (square_index, piece) in self.pieces().iter().enumerate() {
            if piece.kind == PieceKind::NoPiece {
                continue;
            }

            match piece.color {
                PieceColor::White => {
                    white_score += piece.material_value();
                    white_middle_game_score +=
                        piece.middle_game_pst_value(FLIP_SQUARE[square_index]);
                    white_end_game_score += piece.end_game_pst_value(FLIP_SQUARE[square_index]);
                }
                PieceColor::Black => {
                    black_score += piece.material_value();
                    black_middle_game_score += piece.middle_game_pst_value(square_index);
                    black_end_game_score += piece.end_game_pst_value(square_index);
                }
                PieceColor::None => panic!("found a piece with no color"),
            };
        }

        if self.piece_count(Piece::new(PieceColor::White, PieceKind::Bishop)) >= 2 {
            white_middle_game_score += BISHOP_PAIR_MIDDLE_GAME_BONUS;
            white_end_game_score += BISHOP_PAIR_END_GAME_BONUS;
        }

        if self.piece_count(Piece::new(PieceColor::Black, PieceKind::Bishop)) >= 2 {
            black_middle_game_score += BISHOP_PAIR_MIDDLE_GAME_BONUS;
            black_end_game_score += BISHOP_PAIR_END_GAME_BONUS;
        }

        let phase = self.get_game_phase();

        white_score +=
            ((white_middle_game_score * (256 - phase)) + (white_end_game_score * phase)) / 256;

        black_score +=
            ((black_middle_game_score * (256 - phase)) + (black_end_game_score * phase)) / 256;

        let multiplier = match self.side_to_move() {
            Side::White => 1,
            Side::Black => -1,
        };

        multiplier * (white_score - black_score)
    }

    // https://www.chessprogramming.org/Tapered_Eval#Implementation_example
    fn get_game_phase(&self) -> i32 {
        let mut phase = TOTAL_PHASE;

        let white_knights =
            self.piece_count(Piece::new(PieceColor::White, PieceKind::Knight)) as i32;
        let black_knights =
            self.piece_count(Piece::new(PieceColor::Black, PieceKind::Knight)) as i32;

        let white_bishops =
            self.piece_count(Piece::new(PieceColor::White, PieceKind::Bishop)) as i32;
        let black_bishops =
            self.piece_count(Piece::new(PieceColor::Black, PieceKind::Bishop)) as i32;

        let white_rooks = self.piece_count(Piece::new(PieceColor::White, PieceKind::Rook)) as i32;
        let black_rooks = self.piece_count(Piece::new(PieceColor::Black, PieceKind::Rook)) as i32;

        let white_queens = self.piece_count(Piece::new(PieceColor::White, PieceKind::Queen)) as i32;
        let black_queens = self.piece_count(Piece::new(PieceColor::Black, PieceKind::Queen)) as i32;

        phase -= (white_knights + black_knights) * KNIGHT_PHASE;
        phase -= (white_bishops + black_bishops) * BISHOP_PHASE;
        phase -= (white_rooks + black_rooks) * ROOK_PHASE;
        phase -= (white_queens + black_queens) * QUEEN_PHASE;

        phase = (phase * 256 + (TOTAL_PHASE / 2)) / TOTAL_PHASE;

        phase
    }
}
