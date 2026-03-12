use crate::constants::*;
use crate::evaluate::pst_table::*;

use crate::utils::pop_lsb;
use crate::{initialize_board::Board, utils::popcount};

const PST_MG: [[i32; 64]; 6] = [
    PAWN_MG_PST,
    KNIGHT_MG_PST,
    BISHOP_MG_PST,
    ROOK_MG_PST,
    QUEEN_MG_PST,
    KING_MG_PST,
];

const PST_EG: [[i32; 64]; 6] = [
    PAWN_EG_PST,
    KNIGHT_EG_PST,
    BISHOP_EG_PST,
    ROOK_EG_PST,
    QUEEN_EG_PST,
    KING_EG_PST,
];

impl Board {
    pub fn evaluate(&self) -> i32 {
        let mut mg_score: i32 = 0;
        let mut eg_score: i32 = 0;

        for piece in 0..6 {
            let mg_val = MG_VALUE[piece];
            let eg_val = EG_VALUE[piece];

            // White (+)
            let mut bb = self.pieces[Color::White as usize][piece];
            while let Some(sq) = pop_lsb(&mut bb) {
                mg_score += mg_val + PST_MG[piece][sq as usize];
                eg_score += eg_val + PST_EG[piece][sq as usize];
            }

            // Black (-)
            let mut bb = self.pieces[Color::Black as usize][piece];
            while let Some(sq) = pop_lsb(&mut bb) {
                let flipped = mirror(sq as usize);
                mg_score -= mg_val + PST_MG[piece][flipped];
                eg_score -= eg_val + PST_EG[piece][flipped];
            }
        }

        let phase = self.game_phase().min(24); // clamp
        let score = (mg_score * phase + eg_score * (24 - phase)) / 24;

        // Tempo bonus (small advantage for moving)
        let score = score + if self.turn == Color::White { 20 } else { -20 };

        // Return from side-to-move perspective (positive = good for current player)
        score
    }

    fn game_phase(&self) -> i32 {
        let mut phase = 0;
        // your existing code is fine, just add .min(24) in evaluate
        phase += 4 * popcount(self.pieces[Color::White as usize][PieceType::Queen as usize]);
        phase += 4 * popcount(self.pieces[Color::Black as usize][PieceType::Queen as usize]);
        phase += 2 * popcount(self.pieces[Color::White as usize][PieceType::Rook as usize]);
        phase += 2 * popcount(self.pieces[Color::Black as usize][PieceType::Rook as usize]);
        phase += popcount(self.pieces[Color::White as usize][PieceType::Bishop as usize]);
        phase += popcount(self.pieces[Color::Black as usize][PieceType::Bishop as usize]);
        phase += popcount(self.pieces[Color::White as usize][PieceType::Knight as usize]);
        phase += popcount(self.pieces[Color::Black as usize][PieceType::Knight as usize]);
        phase
    }
}
fn mirror(sq: usize) -> usize {
    sq ^ 56
}
