// not used by the main program. this is just a demonstration of how the magics have been
// generated. based on this example by Tord Romstad: https://www.chessprogramming.org/Looking_for_Magics#Feeding_in_Randoms

use crate::{
    bitboard::{Bitboard, EMPTY_BB},
    prng::Prng,
    square::Square,
};

const ROOK_DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
const BISHOP_DIRECTIONS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

struct MagicCandidate {
    magic: u64,
    mask: Bitboard,
    bits_in_mask: u32,
    prng: Prng,
}

impl MagicCandidate {
    fn new(mask: Bitboard) -> Self {
        Self {
            magic: 0,
            mask,
            bits_in_mask: mask.0.count_ones(),
            prng: Prng::new(123),
        }
    }

    fn update_magic(&mut self) {
        self.magic = self.prng.sparse_random_u64();
    }

    fn get_magic_index(&self, blockers: Bitboard) -> usize {
        let blockers = self.mask & blockers;
        let hash = self.magic.wrapping_mul(blockers.0);
        let shift = 64 - self.bits_in_mask;
        hash.wrapping_shr(shift) as usize
    }
}

fn generate_rook_blocker_mask(square: Square) -> anyhow::Result<Bitboard> {
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

fn generate_bishop_blocker_mask(square: Square) -> anyhow::Result<Bitboard> {
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

fn generate_rook_attack_mask(square: Square, blockers: Bitboard) -> anyhow::Result<Bitboard> {
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

fn generate_bishop_attack_mask(square: Square, blockers: Bitboard) -> anyhow::Result<Bitboard> {
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

fn check_rook_magic(candidate: &MagicCandidate, square: Square) -> Option<usize> {
    let max_table_size = 1 << candidate.bits_in_mask;
    let mut attack_table = vec![EMPTY_BB; max_table_size];

    let mut blockers = EMPTY_BB;

    loop {
        let moves = generate_rook_attack_mask(square, blockers).unwrap();
        let index = candidate.get_magic_index(blockers);

        match attack_table.get(index).unwrap() {
            &EMPTY_BB => attack_table[index] = moves,
            bb if *bb != moves => return None,
            _ => (),
        };

        blockers = (blockers - candidate.mask) & candidate.mask;
        if blockers == EMPTY_BB {
            break;
        }
    }

    Some(attack_table.len())
}

fn find_rook_magic(square: Square) -> anyhow::Result<(u64, usize)> {
    let blocker_mask = generate_rook_blocker_mask(square)?;

    let mut candidate = MagicCandidate::new(blocker_mask);

    loop {
        candidate.update_magic();

        if let Some(table_size) = check_rook_magic(&candidate, square) {
            return Ok((candidate.magic, table_size));
        }
    }
}

pub fn print_rook_magics() -> anyhow::Result<()> {
    println!("const ROOK_MAGICS: [u64; 64] = [");

    let mut total_size = 0;

    for square in 0..64usize {
        let (magic, size) = find_rook_magic(square.into())?;
        println!("\t0x{:016X}", magic);
        total_size += size;
    }

    println!("];");

    Ok(())
}
