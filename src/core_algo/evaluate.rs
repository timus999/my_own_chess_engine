use crate::constants::*;
use crate::{initialize_board::Board, popcount};

impl Board {
    pub fn evaluate(&self) -> i32 {
        let mut score = 0;

        score += 100 * popcount(self.pieces[Color::White as usize][PieceType::Pawn as usize]);
        score -= 100 * popcount(self.pieces[Color::Black as usize][PieceType::Pawn as usize]);

        score += 320 * popcount(self.pieces[Color::White as usize][PieceType::Knight as usize]);
        score -= 320 * popcount(self.pieces[Color::Black as usize][PieceType::Knight as usize]);

        score += 330 * popcount(self.pieces[Color::White as usize][PieceType::Bishop as usize]);
        score -= 330 * popcount(self.pieces[Color::Black as usize][PieceType::Bishop as usize]);

        score += 500 * popcount(self.pieces[Color::White as usize][PieceType::Rook as usize]);
        score -= 500 * popcount(self.pieces[Color::Black as usize][PieceType::Rook as usize]);

        score += 900 * popcount(self.pieces[Color::White as usize][PieceType::Queen as usize]);
        score -= 900 * popcount(self.pieces[Color::Black as usize][PieceType::Queen as usize]);
        score
    }
}
