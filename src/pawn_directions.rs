use crate::constants::*;

/// Directions for pawn pushes
pub const NORTH: i8 = 8;
pub const SOUTH: i8 = -8;
pub const NORTH_WEST: i8 = 7;
pub const NORTH_EAST: i8 = 9;
pub const SOUTH_WEST: i8 = -9;
pub const SOUTH_EAST: i8 = -7;

/// Safe shift that avoids wrapping around the board edges
/// (Useful for pawn captures that shouldn't wrap from H-file to A-file)
#[inline]
pub fn shift(bb: Bitboard, dir: i8) -> Bitboard {
    if dir >= 0 {
        bb << dir as u32
    } else {
        bb >> (-dir) as u32
    }
}

/// Shift while masking out file wraps (e.g., for pawn captures)
/// NOT_A_FILE and NOT_H_FILE are masks excluding those files
pub const NOT_A_FILE: Bitboard = 0xfefefefefefefefe; // ~0x0101010101010101 << 0
pub const NOT_H_FILE: Bitboard = 0x7f7f7f7f7f7f7f7f; // ~0x8080808080808080

#[inline]
pub fn shift_north_west(bb: Bitboard) -> Bitboard {
    (bb << 7) & NOT_A_FILE
}

#[inline]
pub fn shift_north_east(bb: Bitboard) -> Bitboard {
    (bb << 9) & NOT_H_FILE
}

#[inline]
pub fn shift_south_west(bb: Bitboard) -> Bitboard {
    (bb >> 9) & NOT_A_FILE
}

#[inline]
pub fn shift_south_east(bb: Bitboard) -> Bitboard {
    (bb >> 7) & NOT_H_FILE
}
