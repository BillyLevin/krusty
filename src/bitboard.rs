use crate::square::{File, Rank, Square};
use colored::Colorize;
use std::{
    fmt::{Debug, Display},
    ops::{BitOr, BitOrAssign},
};

#[derive(Clone, Copy)]
pub struct Bitboard(pub u64);

pub const EMPTY_BB: Bitboard = Bitboard(0);

impl Bitboard {
    pub fn set_bit(&mut self, square: Square) {
        self.0 |= 1u64 << square;
    }

    pub fn is_occupied(&self, square: Square) -> bool {
        self.0 & (1u64 << square) > 0
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

impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(u64::bitor(self.0, rhs.0))
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        u64::bitor_assign(&mut self.0, rhs.0)
    }
}
