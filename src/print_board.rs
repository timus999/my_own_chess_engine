use crate::constants::*;
use crate::initialize_board::Board;

impl Board {
    pub fn print_board(&self) {
        const LAST_BIT: u64 = 63;
        for rank in 0..8 {
            for file in (0..8).rev() {
                let mask = 1u64 << (LAST_BIT - rank * 8 - file);
                let char = if self.occupied & mask != 0 {
                    if self.pieces[Color::White as usize][PieceType::Pawn as usize] & mask != 0 {
                        'P'
                    } else if self.pieces[Color::Black as usize][PieceType::Pawn as usize] & mask
                        != 0
                    {
                        'p'
                    } else if self.pieces[Color::White as usize][PieceType::Knight as usize] & mask
                        != 0
                    {
                        'N'
                    } else if self.pieces[Color::Black as usize][PieceType::Knight as usize] & mask
                        != 0
                    {
                        'n'
                    } else if self.pieces[Color::White as usize][PieceType::Bishop as usize] & mask
                        != 0
                    {
                        'B'
                    } else if self.pieces[Color::Black as usize][PieceType::Bishop as usize] & mask
                        != 0
                    {
                        'b'
                    } else if self.pieces[Color::White as usize][PieceType::Rook as usize] & mask
                        != 0
                    {
                        'R'
                    } else if self.pieces[Color::Black as usize][PieceType::Rook as usize] & mask
                        != 0
                    {
                        'r'
                    } else if self.pieces[Color::White as usize][PieceType::Queen as usize] & mask
                        != 0
                    {
                        'Q'
                    } else if self.pieces[Color::Black as usize][PieceType::Queen as usize] & mask
                        != 0
                    {
                        'q'
                    } else if self.pieces[Color::White as usize][PieceType::King as usize] & mask
                        != 0
                    {
                        'K'
                    } else if self.pieces[Color::Black as usize][PieceType::King as usize] & mask
                        != 0
                    {
                        'k'
                    } else {
                        '_'
                    }
                } else {
                    '_'
                };
                print!("{} ", char);
            }
            println!();
        }
    }
}
