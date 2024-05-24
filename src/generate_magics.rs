// not used by the main program. this is just a demonstration of how the magics have been
// generated. based on this example by Tord Romstad: https://www.chessprogramming.org/Looking_for_Magics#Feeding_in_Randoms

use crate::{
    bitboard::{Bitboard, EMPTY_BB},
    square::Square,
};

const ROOK_DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
const BISHOP_DIRECTIONS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

pub fn generate_rook_blocker_mask(square: Square) -> anyhow::Result<Bitboard> {
    let mut blockers = EMPTY_BB;

    let start_rank = (square.index() / 8) as i32;
    let start_file = (square.index() % 8) as i32;

    for (rank_offset, file_offset) in ROOK_DIRECTIONS {
        let mut rank = start_rank;
        let mut file = start_file;

        while rank > 0 && rank <= 6 && file > 0 && file <= 6 {
            blockers |=
                Square::new((rank as usize).try_into()?, (file as usize).try_into()?).bitboard();

            rank += rank_offset;
            file += file_offset;
        }
    }

    blockers.clear_bit(square);
    Ok(blockers)
}

pub fn generate_bishop_blocker_mask(square: Square) -> anyhow::Result<Bitboard> {
    let mut blockers = EMPTY_BB;

    let start_rank = (square.index() / 8) as i32;
    let start_file = (square.index() % 8) as i32;

    for (rank_offset, file_offset) in BISHOP_DIRECTIONS {
        let mut rank = start_rank;
        let mut file = start_file;

        while rank > 0 && rank <= 6 && file > 0 && file <= 6 {
            blockers |=
                Square::new((rank as usize).try_into()?, (file as usize).try_into()?).bitboard();

            rank += rank_offset;
            file += file_offset;
        }
    }

    blockers.clear_bit(square);
    Ok(blockers)
}

pub fn generate_rook_attack_mask(square: Square, blockers: Bitboard) -> anyhow::Result<Bitboard> {
    let mut attacks = EMPTY_BB;

    let start_rank = (square.index() / 8) as i32;
    let start_file = (square.index() % 8) as i32;

    for (rank_offset, file_offset) in ROOK_DIRECTIONS {
        let mut rank = start_rank;
        let mut file = start_file;

        while rank > 0 && rank <= 6 && file > 0 && file <= 6 {
            let current_square_bitboard =
                Square::new((rank as usize).try_into()?, (file as usize).try_into()?).bitboard();
            attacks |= current_square_bitboard;

            if blockers & current_square_bitboard != EMPTY_BB {
                break;
            }

            rank += rank_offset;
            file += file_offset;
        }
    }

    Ok(attacks)
}

pub fn generate_bishop_attack_mask(square: Square, blockers: Bitboard) -> anyhow::Result<Bitboard> {
    let mut attacks = EMPTY_BB;

    let start_rank = (square.index() / 8) as i32;
    let start_file = (square.index() % 8) as i32;

    for (rank_offset, file_offset) in BISHOP_DIRECTIONS {
        let mut rank = start_rank;
        let mut file = start_file;

        while rank > 0 && rank <= 6 && file > 0 && file <= 6 {
            let current_square_bitboard =
                Square::new((rank as usize).try_into()?, (file as usize).try_into()?).bitboard();
            attacks |= current_square_bitboard;

            if blockers & current_square_bitboard != EMPTY_BB {
                break;
            }

            rank += rank_offset;
            file += file_offset;
        }
    }

    Ok(attacks)
}