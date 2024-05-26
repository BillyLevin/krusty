// not used by the main program. this is just a demonstration of how the magics have been
// generated. based on this example by Tord Romstad: https://www.chessprogramming.org/Looking_for_Magics#Feeding_in_Randoms
// as well as this article by Analog Hors https://analog-hors.github.io/site/magic-bitboards/

use crate::{
    bitboard::{Bitboard, EMPTY_BB},
    magics::generate_blocker_mask,
    prng::Prng,
    square::Square,
};

const ROOK_DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
const BISHOP_DIRECTIONS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

struct MagicCandidate {
    magic: u64,
    mask: Bitboard,
    bits_in_mask: u8,
    prng: Prng,
}

impl MagicCandidate {
    fn new(mask: Bitboard) -> Self {
        Self {
            magic: 0,
            mask,
            bits_in_mask: mask.0.count_ones() as u8,
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
        (hash >> shift) as usize
    }
}

fn generate_attack_mask(
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

            if (0..=7).contains(&rank) && (0..=7).contains(&file) {
                attacks |= next_square;
            } else {
                break;
            }

            if blockers & next_square != EMPTY_BB {
                break;
            }
        }
    }

    attacks.clear_bit(square);
    attacks
}

fn find_magic(square: Square, directions: [(i32, i32); 4]) -> (u64, usize) {
    let blocker_mask = generate_blocker_mask(square, directions);

    let mut candidate = MagicCandidate::new(blocker_mask);

    loop {
        candidate.update_magic();

        if let Some(table_size) = check_magic(&candidate, square, directions) {
            return (candidate.magic, table_size);
        }
    }
}

fn check_magic(
    candidate: &MagicCandidate,
    square: Square,
    directions: [(i32, i32); 4],
) -> Option<usize> {
    let max_table_size = 1 << candidate.bits_in_mask;
    let mut attack_table = vec![EMPTY_BB; max_table_size];

    let mut blockers = EMPTY_BB;

    loop {
        let moves = generate_attack_mask(square, blockers, directions);
        let index = candidate.get_magic_index(blockers);

        let entry = attack_table.get_mut(index).unwrap();

        if *entry == EMPTY_BB {
            *entry = moves;
        } else if *entry != moves {
            return None;
        }

        blockers = (blockers - candidate.mask) & candidate.mask;
        if blockers == EMPTY_BB {
            break;
        }
    }

    Some(attack_table.len())
}

fn print_rook_magics() {
    println!("pub const ROOK_MAGICS: [MagicNumber; 64] = [");

    let mut total_size = 0;

    for square in 0..64usize {
        let (magic, size) = find_magic(square.into(), ROOK_DIRECTIONS);
        println!(
            "\tMagicNumber {{ magic: 0x{:016X}, offset: {} }},",
            magic, total_size
        );
        total_size += size;
    }

    println!("];");

    println!("const ROOK_ATTACK_TABLE_SIZE: usize = {total_size};");
}

fn print_bishop_magics() {
    println!("pub const BISHOP_MAGICS: [MagicNumber; 64] = [");

    let mut total_size = 0;

    for square in 0..64usize {
        let (magic, size) = find_magic(square.into(), BISHOP_DIRECTIONS);
        println!(
            "\tMagicNumber {{ magic: 0x{:016X}, offset: {} }},",
            magic, total_size
        );
        total_size += size;
    }

    println!("];");

    println!("const BISHOP_ATTACK_TABLE_SIZE: usize = {total_size};");
}

pub fn print_magics() {
    print_rook_magics();
    print_bishop_magics();
}
