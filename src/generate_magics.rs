// not used by the main program. this is just a demonstration of how the magics have been
// generated. based on this example by Tord Romstad: https://www.chessprogramming.org/Looking_for_Magics#Feeding_in_Randoms

use crate::{
    bitboard::{Bitboard, EMPTY_BB},
    square::Square,
};

pub fn generate_rook_blocker_mask(square: Square) -> anyhow::Result<Bitboard> {
    let mut blockers = EMPTY_BB;

    let start_rank = square.index() / 8;
    let start_file = square.index() % 8;

    dbg!(start_rank);
    dbg!(start_file);

    for rank in (start_rank + 1)..=6 {
        blockers |= Square::new(rank.try_into()?, start_file.try_into()?).bitboard();
    }

    for rank in (1..=(start_rank - 1)).rev() {
        blockers |= Square::new(rank.try_into()?, start_file.try_into()?).bitboard();
    }

    for file in (start_file + 1)..=6 {
        blockers |= Square::new(start_rank.try_into()?, file.try_into()?).bitboard();
    }

    for file in (1..=(start_file - 1)).rev() {
        blockers |= Square::new(start_rank.try_into()?, file.try_into()?).bitboard();
    }

    Ok(blockers)
}
