// not used by the main program. this is just a demonstration of how the magics have been
// generated. based on this example by Tord Romstad: https://www.chessprogramming.org/Looking_for_Magics#Feeding_in_Randoms
// as well as this article by Analog Hors https://analog-hors.github.io/site/magic-bitboards/

use crate::{
    bitboard::{Bitboard, EMPTY_BB},
    move_generator::{
        generate_sliding_attack_mask, generate_sliding_blocker_mask, BISHOP_DIRECTIONS,
        ROOK_DIRECTIONS,
    },
    prng::Prng,
    square::Square,
};

struct MagicCandidate {
    magic: u64,
    mask: Bitboard,
    bits_in_mask: u8,
    prng: Prng,
}

struct DiscoveredMagic {
    magic: u64,
    shift: u8,
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

fn find_magic(square: Square, directions: [(i32, i32); 4]) -> (DiscoveredMagic, usize) {
    let blocker_mask = generate_sliding_blocker_mask(square, directions);

    let mut candidate = MagicCandidate::new(blocker_mask);

    loop {
        candidate.update_magic();

        if let Some(table_size) = check_magic(&candidate, square, directions) {
            return (
                DiscoveredMagic {
                    magic: candidate.magic,
                    shift: 64 - candidate.bits_in_mask,
                },
                table_size,
            );
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
        let moves = generate_sliding_attack_mask(square, blockers, directions);
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
        let (DiscoveredMagic { magic, shift }, size) = find_magic(square.into(), ROOK_DIRECTIONS);
        println!(
            "\tMagicNumber {{ magic: 0x{:016X}, shift: {}, offset: {} }},",
            magic, shift, total_size
        );
        total_size += size;
    }

    println!("];");

    println!("pub const ROOK_ATTACK_TABLE_SIZE: usize = {total_size};");
}

fn print_bishop_magics() {
    println!("pub const BISHOP_MAGICS: [MagicNumber; 64] = [");

    let mut total_size = 0;

    for square in 0..64usize {
        let (DiscoveredMagic { magic, shift }, size) = find_magic(square.into(), BISHOP_DIRECTIONS);
        println!(
            "\tMagicNumber {{ magic: 0x{:016X}, shift: {}, offset: {} }},",
            magic, shift, total_size
        );
        total_size += size;
    }

    println!("];");

    println!("pub const BISHOP_ATTACK_TABLE_SIZE: usize = {total_size};");
}

pub fn print_magics() {
    print_rook_magics();
    print_bishop_magics();
}
