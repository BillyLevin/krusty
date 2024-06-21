use crate::square::{File, Rank, Square};
use colored::Colorize;
use std::{
    fmt::{Debug, Display},
    ops::{
        Add, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr, Sub,
    },
};

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bitboard(pub u64);

pub const EMPTY_BB: Bitboard = Bitboard(0u64);

impl Bitboard {
    pub fn set_bit(&mut self, square: Square) {
        *self |= square.bitboard();
    }

    pub fn clear_bit(&mut self, square: Square) {
        *self &= !square.bitboard();
    }

    pub fn is_occupied(self, square: Square) -> bool {
        self & square.bitboard() != EMPTY_BB
    }

    pub fn pop_bit(&mut self) -> Square {
        let square = self.get_lsb_square();
        *self ^= square.bitboard();
        square
    }

    pub fn get_lsb(self) -> u64 {
        (self & (!self + 1)).into()
    }

    pub fn get_lsb_square(self) -> Square {
        self.get_lsb().trailing_zeros().into()
    }
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;

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

        writeln!(f)?;

        Ok(())
    }
}

impl BitOr<Bitboard> for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl<Rhs> BitOrAssign<Rhs> for Bitboard
where
    Bitboard: BitOr<Rhs, Output = Bitboard>,
{
    fn bitor_assign(&mut self, rhs: Rhs) {
        *self = *self | rhs;
    }
}

impl BitAnd<Bitboard> for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl<Rhs> BitAndAssign<Rhs> for Bitboard
where
    Bitboard: BitAnd<Rhs, Output = Bitboard>,
{
    fn bitand_assign(&mut self, rhs: Rhs) {
        *self = *self & rhs;
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl BitXor<Bitboard> for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl<Rhs> BitXorAssign<Rhs> for Bitboard
where
    Bitboard: BitXor<Rhs, Output = Bitboard>,
{
    fn bitxor_assign(&mut self, rhs: Rhs) {
        *self = *self ^ rhs
    }
}

impl Shl<usize> for Bitboard {
    type Output = Bitboard;

    fn shl(self, rhs: usize) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

impl Shr<usize> for Bitboard {
    type Output = Bitboard;

    fn shr(self, rhs: usize) -> Self::Output {
        Bitboard(self.0 >> rhs)
    }
}

impl Add<u64> for Bitboard {
    type Output = Bitboard;

    fn add(self, rhs: u64) -> Self::Output {
        Bitboard(self.0 + rhs)
    }
}

impl From<Bitboard> for u64 {
    fn from(bitboard: Bitboard) -> Self {
        bitboard.0
    }
}

impl Sub<Bitboard> for Bitboard {
    type Output = Bitboard;

    fn sub(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0.wrapping_sub(rhs.0))
    }
}
