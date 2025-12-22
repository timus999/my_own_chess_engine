use crate::attack::*;
use crate::constants::*;
use crate::initialize_board::*;
use crate::pawn_directions::*;
use crate::utils::*;

#[derive(Debug, Clone, Copy)]
pub struct Move {
    from: Square,
    to: Square,
    promotion: Option<PieceType>, // For pawns
}
impl Move {
    pub fn moving_piece(&self, board: &Board) -> Option<PieceType> {
        let color = board.turn;
        for pt in 0..6 {
            if get_bit(board.pieces[color as usize][pt], self.from) {
                return PieceType::from_usize(pt); // you'll need this helper
            }
        }
        None
    }

    pub fn captured_piece(&self, board: &Board) -> Option<PieceType> {
        let opp_color = if board.turn == Color::White {
            Color::Black
        } else {
            Color::White
        };
        for pt in 0..6 {
            if get_bit(board.pieces[opp_color as usize][pt], self.to) {
                // Special case: en passant capture
                if board.en_passant == Some(self.to)
                    && self.moving_piece(board) == Some(PieceType::Pawn)
                {
                    return Some(PieceType::Pawn);
                }
                return PieceType::from_usize(pt);
            }
        }
        None
    }
    pub fn to_long_algebraic(&self, board: &Board) -> String {
        if self.moving_piece(board).is_none() {
            return String::new();
        }
        let piece_char = match self.moving_piece(board).unwrap() {
            PieceType::Pawn => "",
            PieceType::Knight => "N",
            PieceType::Bishop => "B",
            PieceType::Rook => "R",
            PieceType::Queen => "Q",
            PieceType::King => "K",
        };

        let from_str = square_to_algebraic(self.from);
        let to_str = square_to_algebraic(self.to);
        let promo_str = self
            .promotion
            .map(|p| match p {
                PieceType::Queen => "q",
                PieceType::Rook => "r",
                PieceType::Bishop => "b",
                PieceType::Knight => "n",
                _ => "",
            })
            .unwrap_or("");

        let capture = if self.captured_piece(board).is_some() {
            "x"
        } else {
            ""
        };

        format!(
            "{}{}{}{}{}",
            piece_char, from_str, capture, to_str, promo_str
        )
    }
}

fn square_to_algebraic(sq: Square) -> String {
    let file = (sq % 8) as u8 + b'a';
    let rank = (sq / 8) + 1;
    format!("{}{}", file as char, rank)
}
impl Board {
    pub fn generate_pseudo_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        let color = self.turn;

        let opp_color = if color == Color::White {
            Color::Black
        } else {
            Color::White
        };

        let own_pieces = self.all_pieces(color);
        let opp_pieces = self.all_pieces(opp_color);
        let occupied = self.occupied;
        let empty = !occupied;

        // helper to add normal moves
        macro_rules! add_moves {
            ($from:expr, $targets:expr) => {
                let mut targets = $targets;
                while let Some(to) = pop_lsb(&mut targets) {
                    moves.push(Move {
                        from: $from,
                        to,
                        promotion: None,
                    });
                }
            };
        }

        // Pawns
        let pawns = self.pieces[color as usize][PieceType::Pawn as usize];
        let direction: i8 = if color == Color::White { 8 } else { -8 };

        let promotion_rank = if color == Color::White {
            RANK_8
        } else {
            RANK_1
        };
        let double_push_rank = if color == Color::White {
            RANK_4
        } else {
            RANK_5
        };

        // single pushes
        let single_push = shift(pawns, direction) & empty;
        let mut push_from = shift(single_push, -direction); // back to origin

        // Promotion via push
        let promo_pushes = single_push & promotion_rank;
        let mut promo_from = shift(promo_pushes, -direction);

        while let Some(from) = pop_lsb(&mut promo_from) {
            let to = (from as i8 + direction) as Square;
            for &promo in &[
                PieceType::Queen,
                PieceType::Rook,
                PieceType::Bishop,
                PieceType::Knight,
            ] {
                moves.push(Move {
                    from,
                    to,
                    promotion: Some(promo),
                });
            }
        }
        // Normal single pushes (non-promotion)
        let normal_pushes = single_push & !promotion_rank;
        push_from = shift(normal_pushes, -direction);
        while let Some(from) = pop_lsb(&mut push_from) {
            let to = (from as i8 + direction) as Square;
            moves.push(Move {
                from,
                to,
                promotion: None,
            });
        }

        // Double pushes
        let double_possible = shift(single_push, direction) & empty & double_push_rank;
        let mut double_from = shift(double_possible, -direction * 2);
        while let Some(from) = pop_lsb(&mut double_from) {
            let to = (from as i8 + direction * 2) as Square;
            moves.push(Move {
                from,
                to,
                promotion: None,
            });
        }
        // =====================
        // Pawn Captures
        // =====================

        // Capture targets include en passant square
        let capture_targets = opp_pieces | self.en_passant.map_or(0, |sq| 1u64 << sq);

        // ---------- LEFT DIAGONAL CAPTURES ----------
        let left_attacks = if color == Color::White {
            (pawns << 7) & NOT_A_FILE
        } else {
            (pawns >> 9) & NOT_H_FILE
        };

        let left_caps = left_attacks & capture_targets;

        // Recover FROM square
        let left_from_shift = if color == Color::White { -7 } else { 9 };
        // Compute TO square
        let left_delta = if color == Color::White { 7 } else { -9 };

        let mut left_from = shift(left_caps, left_from_shift);

        while let Some(from) = pop_lsb(&mut left_from) {
            let to = (from as i16 + left_delta) as Square;

            if get_bit(promotion_rank, to) {
                for &promo in &[
                    PieceType::Queen,
                    PieceType::Rook,
                    PieceType::Bishop,
                    PieceType::Knight,
                ] {
                    moves.push(Move {
                        from,
                        to,
                        promotion: Some(promo),
                    });
                }
            } else {
                moves.push(Move {
                    from,
                    to,
                    promotion: None,
                });
            }
        }

        // ---------- RIGHT DIAGONAL CAPTURES ----------
        let right_attacks = if color == Color::White {
            (pawns << 9) & NOT_H_FILE
        } else {
            (pawns >> 7) & NOT_A_FILE
        };

        let right_caps = right_attacks & capture_targets;

        // Recover FROM square
        let right_from_shift = if color == Color::White { -9 } else { 7 };
        // Compute TO square
        let right_delta = if color == Color::White { 9 } else { -7 };

        let mut right_from = shift(right_caps, right_from_shift);

        while let Some(from) = pop_lsb(&mut right_from) {
            let to = (from as i16 + right_delta) as Square;

            if get_bit(promotion_rank, to) {
                for &promo in &[
                    PieceType::Queen,
                    PieceType::Rook,
                    PieceType::Bishop,
                    PieceType::Knight,
                ] {
                    moves.push(Move {
                        from,
                        to,
                        promotion: Some(promo),
                    });
                }
            } else {
                moves.push(Move {
                    from,
                    to,
                    promotion: None,
                });
            }
        }

        // === KNIGHTS ===
        let knights = self.pieces[color as usize][PieceType::Knight as usize];
        let mut knight_bb = knights;
        while let Some(from) = pop_lsb(&mut knight_bb) {
            let attacks = KNIGHT_ATTACKS[from as usize] & !own_pieces;
            add_moves!(from, attacks);
        }
        // === BISHOPS ===
        let bishops = self.pieces[color as usize][PieceType::Bishop as usize];
        let mut bishop_bb = bishops;
        println!("bishop: {:b}", bishop_bb);
        while let Some(from) = pop_lsb(&mut bishop_bb) {
            let attacks = self.bishop_attacks(from, occupied) & !own_pieces;
            add_moves!(from, attacks);
        }

        // === ROOKS ===
        let rooks = self.pieces[color as usize][PieceType::Rook as usize];
        let mut rook_bb = rooks;
        while let Some(from) = pop_lsb(&mut rook_bb) {
            let attacks = self.rook_attacks(from, occupied) & !own_pieces;
            add_moves!(from, attacks);
        }

        // === QUEENS ===
        let queens = self.pieces[color as usize][PieceType::Queen as usize];
        let mut queen_bb = queens;
        while let Some(from) = pop_lsb(&mut queen_bb) {
            let attacks = self.queen_attacks(from, occupied) & !own_pieces;
            add_moves!(from, attacks);
        }
        // === KING ===
        let king_sq = lsb(self.pieces[color as usize][PieceType::King as usize]);
        let king_attacks = KING_ATTACKS[king_sq as usize] & !own_pieces;
        add_moves!(king_sq, king_attacks);

        // === CASTLING (pseudo-legal only) ===
        if !self.is_in_check(color) {
            let kingside = if color == Color::White {
                0b0001
            } else {
                0b0100
            };
            let queenside = if color == Color::White {
                0b0010
            } else {
                0b1000
            };

            let back_rank = if color == Color::White {
                RANK_1
            } else {
                RANK_8
            };

            if self.castling_rights & kingside != 0 {
                let path = if color == Color::White {
                    0b01100000
                } else {
                    0b01100000 << 56
                };
                if occupied & path == 0 {
                    moves.push(Move {
                        from: king_sq,
                        to: king_sq + 2,
                        promotion: None,
                    }); // kingside
                }
            }

            if self.castling_rights & queenside != 0 {
                let path = if color == Color::White {
                    0b00011100
                } else {
                    0b00011100 << 56
                };
                if occupied & path == 0 {
                    moves.push(Move {
                        from: king_sq,
                        to: king_sq - 2,
                        promotion: None,
                    }); // queenside
                }
            }
        }

        moves
    }

    // Helper ray attack functions (simple but correct)
    // fn bishop_attacks(&self, sq: Square, occupied: Bitboard) -> Bitboard {
    //     self.diagonal_attacks(sq, occupied)
    // }

    // fn rook_attacks(&self, sq: Square, occupied: Bitboard) -> Bitboard {
    //     self.rank_attacks(sq, occupied) | self.file_attacks(sq, occupied)
    // }
    fn bishop_attacks(&self, sq: Square, occupied: Bitboard) -> Bitboard {
        self.diagonal_attacks(sq, occupied) | self.antidiagonal_attacks(sq, occupied)
    }

    fn rook_attacks(&self, sq: Square, occupied: Bitboard) -> Bitboard {
        self.rank_attacks(sq, occupied) | self.file_attacks(sq, occupied)
    }
    fn queen_attacks(&self, sq: Square, occupied: Bitboard) -> Bitboard {
        self.bishop_attacks(sq, occupied) | self.rook_attacks(sq, occupied)
    }

    // fn diagonal_attacks(&self, sq: Square, occupied: Bitboard) -> Bitboard {
    //     let mut attacks = 0;

    //     let file = (sq % 8) as i8;
    //     let rank = (sq / 8) as i8;

    //     // NE
    //     for i in 1..8 {
    //         if file + i > 7 || rank + i > 7 {
    //             break;
    //         }
    //         let target = sq + i as Square * 9;
    //         set_bit(&mut attacks, target);
    //         if get_bit(occupied, target) {
    //             break;
    //         }
    //     }
    //     // NW
    //     for i in 1..8 {
    //         if file - i < 0 || rank + i > 7 {
    //             break;
    //         }
    //         let target = sq + i as Square * 7;
    //         set_bit(&mut attacks, target);
    //         if get_bit(occupied, target) {
    //             break;
    //         }
    //     }
    //     // SE, SW similarly...
    //     //
    //     // SE
    //     for i in 1..8 {
    //         if file + i > 7 || rank - i < 0 {
    //             break;
    //         }
    //         let target = sq - i as Square * 7;
    //         set_bit(&mut attacks, target);
    //         if get_bit(occupied, target) {
    //             break;
    //         }
    //     }
    //     // SW
    //     for i in 1..8 {
    //         if file - i < 0 || rank - i < 0 {
    //             break;
    //         }
    //         let target = sq - i as Square * 9;
    //         set_bit(&mut attacks, target);
    //         if get_bit(occupied, target) {
    //             break;
    //         }
    //     }
    //     attacks
    // }

    // fn rank_attacks(&self, sq: Square, occupied: Bitboard) -> Bitboard {
    //     let mut attacks = 0;

    //     let rank = (sq / 8) as i8;

    //     // Right | East
    //     for i in 1..8 {
    //         if rank + i > 7 {
    //             break;
    //         }
    //         let target = sq + i as Square;
    //         set_bit(&mut attacks, target);
    //         if get_bit(occupied, target) {
    //             break;
    //         }
    //     }
    //     // Left | West
    //     for i in 1..8 {
    //         if rank - i < 0 {
    //             break;
    //         }
    //         let target = sq - i as Square;
    //         set_bit(&mut attacks, target);
    //         if get_bit(occupied, target) {
    //             break;
    //         }
    //     }
    //     attacks
    // }
    // fn file_attacks(&self, sq: Square, occupied: Bitboard) -> Bitboard {
    //     let mut attacks = 0;

    //     let file = (sq % 8) as i8;

    //     // Up | North
    //     //
    //     for i in 1..8 {
    //         if file + i > 7 {
    //             break;
    //         }
    //         let target = sq + i as Square * 8;
    //         set_bit(&mut attacks, target);
    //         if get_bit(occupied, target) {
    //             break;
    //         }
    //     }
    //     // Down | South
    //     for i in 1..8 {
    //         if file - i < 0 {
    //             break;
    //         }
    //         let target = sq - i as Square * 8;
    //         set_bit(&mut attacks, target);
    //         if get_bit(occupied, target) {
    //             break;
    //         }
    //     }
    //     attacks
    // }
    fn diagonal_attacks(&self, sq: Square, occupied: Bitboard) -> Bitboard {
        self.ray_attack(sq as i16, 9, occupied) | self.ray_attack(sq as i16, -9, occupied)
    }

    fn antidiagonal_attacks(&self, sq: Square, occupied: Bitboard) -> Bitboard {
        self.ray_attack(sq as i16, 7, occupied) | self.ray_attack(sq as i16, -7, occupied)
    }

    fn rank_attacks(&self, sq: Square, occupied: Bitboard) -> Bitboard {
        self.ray_attack(sq as i16, 1, occupied) | self.ray_attack(sq as i16, -1, occupied)
    }

    fn file_attacks(&self, sq: Square, occupied: Bitboard) -> Bitboard {
        self.ray_attack(sq as i16, 8, occupied) | self.ray_attack(sq as i16, -8, occupied)
    }

    /// Generic ray in one direction until blocked or edge
    fn ray_attack(&self, sq: i16, direction: i8, occupied: Bitboard) -> Bitboard {
        let mut attacks: Bitboard = 0;
        let mut cur = sq;
        loop {
            let prev_file = cur % 8;
            cur += direction as i16;
            if cur < 0 || cur >= 64 {
                break;
            }
            let new_file = cur % 8;

            // file wrap detection
            if (prev_file - new_file).abs() > 2 {
                break;
            }

            let cur_sq = cur as Square;
            set_bit(&mut attacks, cur_sq);

            if get_bit(occupied, cur_sq) {
                break; // blocked by any piece
            }
        }
        attacks
    }
    /// Returns true if the king of the given color is in check
    pub fn is_in_check(&self, color: Color) -> bool {
        let king_sq = self.king_square(color);
        let opp_color = if color == Color::White {
            Color::Black
        } else {
            Color::White
        };

        self.is_square_attacked(king_sq, opp_color)
    }

    /// Helper: get the square of the king for the given color
    /// Assumes there is exactly one king (panics otherwise – safe in valid positions)
    fn king_square(&self, color: Color) -> Square {
        lsb(self.pieces[color as usize][PieceType::King as usize])
    }

    /// Returns true if the given square is attacked by the given color
    fn is_square_attacked(&self, sq: Square, by_color: Color) -> bool {
        let occupied = self.occupied;
        let own_pieces = self.all_pieces(by_color); // not needed for blockers in pawn/knight/king
        let opp_pieces = self.all_pieces(if by_color == Color::White {
            Color::Black
        } else {
            Color::White
        });

        // Pawn attacks (direction depends on attacker color)
        let pawn_attacks = if by_color == Color::White {
            // White pawns attack upwards
            ((1u64 << sq) >> 7) & NOT_H_FILE | ((1u64 << sq) >> 9) & NOT_A_FILE
        } else {
            // Black pawns attack downwards
            ((1u64 << sq) << 7) & NOT_A_FILE | ((1u64 << sq) << 9) & NOT_H_FILE
        };

        if pawn_attacks & self.pieces[by_color as usize][PieceType::Pawn as usize] != 0 {
            return true;
        }

        // Knight attacks
        if KNIGHT_ATTACKS[sq as usize] & self.pieces[by_color as usize][PieceType::Knight as usize]
            != 0
        {
            return true;
        }

        // King attacks (adjacent)
        if KING_ATTACKS[sq as usize] & self.pieces[by_color as usize][PieceType::King as usize] != 0
        {
            return true;
        }

        // Bishop / Queen diagonal attacks
        if self.bishop_attacks(sq, occupied)
            & (self.pieces[by_color as usize][PieceType::Bishop as usize]
                | self.pieces[by_color as usize][PieceType::Queen as usize])
            != 0
        {
            return true;
        }

        // Rook / Queen rank/file attacks
        if self.rook_attacks(sq, occupied)
            & (self.pieces[by_color as usize][PieceType::Rook as usize]
                | self.pieces[by_color as usize][PieceType::Queen as usize])
            != 0
        {
            return true;
        }

        false
    }
}
