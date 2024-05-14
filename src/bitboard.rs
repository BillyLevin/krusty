use crate::square::{File, Rank, Square};
use colored::Colorize;
use std::fmt::{Debug, Display};

#[derive(Clone, Copy, Default)]
pub struct Bitboard(pub u64);

pub const EMPTY_BB: Bitboard = Bitboard(0u64);

impl Bitboard {
    pub fn set_bit(&mut self, square: Square) {
        self.0 |= square.bitboard().value();
    }

    pub fn clear_bit(&mut self, square: Square) {
        self.0 &= !(square.bitboard().value());
    }

    pub fn is_occupied(&self, square: Square) -> bool {
        self.0 & (square.bitboard().value()) > 0
    }

    pub fn value(&self) -> u64 {
        self.0
    }

    pub fn pop_bit(&mut self) -> Square {
        let square: Square = self.get_lsb().trailing_zeros().into();
        self.0 ^= square.bitboard().value();
        square
    }

    fn get_lsb(&self) -> u64 {
        self.value() & (!self.value() + 1)
    }
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &rank in Rank::EVERY.iter().rev() {
            for file in File::EVERY {
                if file == File::A {
                    write!(f, "  {}", (rank + 1).to_string().cyan())?;
                }

                let square = Square::new(rank, file);
                if self.is_occupied(square) {
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
}
