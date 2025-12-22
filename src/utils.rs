use crate::constants::*;
/// Set a bit at the given square (0-63)
#[inline(always)]
pub fn set_bit(bb: &mut Bitboard, sq: Square) {
    *bb |= 1u64 << sq;
}

/// Clear a bit at the given square (0-63)
#[inline(always)]
pub fn clear_bit(bb: &mut Bitboard, sq: Square) {
    *bb &= !(1u64 << sq);
}

/// Test if a bit is set at the given square (0-63)
#[inline(always)]
pub fn get_bit(bb: Bitboard, sq: Square) -> bool {
    bb & (1u64 << sq) != 0
}

/// Pop the least significant bit from the bitboard and return its index
#[inline(always)]
pub fn pop_lsb(bb: &mut Bitboard) -> Option<Square> {
    if *bb == 0 {
        None
    } else {
        let lsb = bb.trailing_zeros() as Square;
        *bb &= *bb - 1; // Clear the least significant bit
        Some(lsb)
    }
}

/// Get the least significant set bit's square without modifying the bitboard
#[inline(always)]
pub fn lsb(bb: Bitboard) -> Square {
    debug_assert!(bb != 0, "lsb called an empty bitboard");
    bb.trailing_zeros() as Square
}

/// Get the most significant set bit's square
#[inline(always)]
pub fn msb(bb: Bitboard) -> Square {
    debug_assert!(bb != 0, "msb called an empty bitboard");
    63 - bb.leading_zeros() as Square
}
