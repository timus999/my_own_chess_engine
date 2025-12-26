use crate::constants::*;
use crate::initialize_board::Board;
use crate::pseudo_legal_move_generation::Move;
use crate::utils::*;

impl Board {
    pub fn apply_move(&mut self, m: &Move) {
        let color = self.turn;
        let opp_color = if color == Color::White {
            Color::Black
        } else {
            Color::White
        };

        let moving_piece = m.moving_piece(&self);

        // Clear 'from' square
        clear_bit(
            &mut self.pieces[color as usize][moving_piece.unwrap() as usize],
            m.from,
        );

        clear_bit(&mut self.occupied, m.from);

        // handle promotion
        let placed_piece = if let Some(promo) = m.promotion {
            promo
        } else {
            moving_piece.unwrap()
        };

        // handle capture (including en passant)
        let mut captured_piece: Option<PieceType> = None;
        let captured_sq =
            if moving_piece.unwrap() == PieceType::Pawn && self.en_passant == Some(m.to) {
                // En passant capture happens on the square behind 'to'
                let ep_capture_sq = if color == Color::White {
                    m.to - 8
                } else {
                    m.to + 8
                };
                // Find opponent's pawn there
                clear_bit(
                    &mut self.pieces[opp_color as usize][PieceType::Pawn as usize],
                    ep_capture_sq,
                );
                clear_bit(&mut self.occupied, ep_capture_sq);
                captured_piece = Some(PieceType::Pawn);
                ep_capture_sq
            } else {
                // Normal capture
                for pt_idx in 0..6 {
                    if get_bit(self.pieces[opp_color as usize][pt_idx], m.to) {
                        captured_piece = PieceType::from_usize(pt_idx);
                        clear_bit(&mut self.pieces[opp_color as usize][pt_idx], m.to);
                        break;
                    }
                }
                clear_bit(&mut self.occupied, m.to);
                m.to
            };

        // place piece on 'to' square
        set_bit(
            &mut self.pieces[color as usize][placed_piece as usize],
            m.to,
        );
        set_bit(&mut self.occupied, m.to);

        // Special: Castling
        if moving_piece.unwrap() == PieceType::King && (m.from as i8 - m.to as i8).abs() == 2 {
            // Determine rook move
            let (rook_from, rook_to) = if m.to > m.from {
                // king side
                (m.from + 3, m.from + 1)
            } else {
                // queen side
                (m.from - 4, m.from - 1)
            };

            clear_bit(
                &mut self.pieces[color as usize][PieceType::Rook as usize],
                rook_from,
            );
            set_bit(
                &mut self.pieces[color as usize][PieceType::Rook as usize],
                rook_to,
            );
            clear_bit(&mut self.occupied, rook_from);
            set_bit(&mut self.occupied, rook_to);
        }

        // Update castling rights
        if moving_piece.unwrap() == PieceType::King {
            // King moved -> lose both rights for this color
            if color == Color::White {
                self.castling_rights &= !0b0011; // Clear white KQ
            } else {
                self.castling_rights &= !0b1100; // Clear black KQ
            }
        } else if moving_piece.unwrap() == PieceType::Rook {
            // Rook moved from corner -> lose corressponding right
            if color == Color::White {
                if m.from == 0 {
                    // A1
                    self.castling_rights &= !0b0010; // Clear white queenside
                } else if m.from == 7 {
                    // H1
                    self.castling_rights &= !0b0001; // Clear white kingside
                }
            } else {
                if m.from == 56 {
                    // A8
                    self.castling_rights &= !0b1000; // Clear black queenside
                } else if m.from == 63 {
                    // H8
                    self.castling_rights &= !0b0100; // Clear black kingside
                }
            }
        }

        // if opponent rook was captured on corner, remove their castling right
        if let Some(capt_pt) = captured_piece {
            if capt_pt == PieceType::Rook {
                if color == Color::White {
                    if captured_sq == 56 {
                        self.castling_rights &= !0b1000;
                    } // Black queenside
                    if captured_sq == 63 {
                        self.castling_rights &= !0b0100;
                    } // Black kingside
                } else {
                    if captured_sq == 0 {
                        self.castling_rights &= !0b0010;
                    } // White queenside
                    if captured_sq == 7 {
                        self.castling_rights &= !0b0001;
                    } // White kingside
                }
            }
        }

        // update en passant target
        self.en_passant = None;
        if moving_piece.unwrap() == PieceType::Pawn && (m.from as i8 - m.to as i8).abs() == 16 {
            // Double pawn push
            let ep_sq = ((m.from as i8 + m.to as i8) / 2) as Square;
            self.en_passant = Some(ep_sq);
        }

        // Update_counters
        //
        if moving_piece.unwrap() == PieceType::Pawn || captured_piece.is_some() {
            self.half_moves = 0;
        } else {
            self.half_moves += 1;
        }

        if color == Color::Black {
            self.full_moves += 1;
        }

        // Flip turn
        self.turn = opp_color;
    }
}
