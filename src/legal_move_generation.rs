use crate::initialize_board::*;
use crate::pseudo_legal_move_generation::Move;

impl Board {
    pub fn generate_legal_moves(&self) -> Vec<Move> {
        let pseudo_moves = self.generate_pseudo_moves();
        let mut legal_moves = Vec::with_capacity(pseudo_moves.len());

        let original_turn = self.turn;

        for &m in &pseudo_moves {
            // Create a temporary copy of the board
            let mut temp_board = *self;

            temp_board.apply_move(&m);

            // After the move, it's the opponent's turn - check if our king is in check
            if !temp_board.is_in_check(original_turn) {
                legal_moves.push(m);
            }
        }
        legal_moves
    }
}
