use crate::initialize_board::Board;
use std::io::Read;

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

#[inline(always)]
pub fn popcount(bb: Bitboard) -> i32 {
    bb.count_ones() as i32
}

// Returns array of legal destinations for a piece
pub fn legal_destinations(board: &Board, from: Square) -> Vec<Square> {
    board
        .generate_legal_moves()
        .iter()
        .filter(|m| m.from == from)
        .map(|m| m.to)
        .collect::<Vec<Square>>()
}
