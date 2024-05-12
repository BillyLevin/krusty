use crate::square::{File, Rank, Square};
use colored::Colorize;

pub type Bitboard = u64;

pub const EMPTY_BB: Bitboard = 0u64;

pub fn set_bit(bitboard: &mut Bitboard, square: Square) {
    *bitboard |= square.bitboard();
}

pub fn clear_bit(bitboard: &mut Bitboard, square: Square) {
    *bitboard &= !(square.bitboard());
}

pub fn is_occupied(bitboard: &Bitboard, square: Square) -> bool {
    bitboard & square.bitboard() > 0
}

pub fn pop_bit(bitboard: &mut Bitboard) -> Square {
    let square: Square = get_lsb(bitboard).trailing_zeros().into();
    *bitboard ^= square.bitboard();
    square
}

fn get_lsb(bitboard: &Bitboard) -> u64 {
    bitboard & (!bitboard + 1)
}

pub fn print_bitboard(bitboard: &Bitboard, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for &rank in Rank::EVERY.iter().rev() {
        for file in File::EVERY {
            if file == File::A {
                write!(f, "  {}", (rank + 1).to_string().cyan())?;
            }

            let square = Square::new(rank, file);
            if is_occupied(bitboard, square) {
                write!(f, "  1")?;
            } else {
                write!(f, "  0")?;
            }
        }

        writeln!(f)?;
    }

    write!(f, "   ")?;

    for file in File::EVERY {
        write!(f, "  {}", file.to_string().cyan())?;
    }

    Ok(())
}
