use crate::attack::*;
use crate::constants::*;
use crate::initialize_board::*;
use crate::pawn_directions::*;
use crate::utils::*;

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<PieceType>, // For pawns
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

    pub fn from_algebric(s: &str, board: &Board) -> Result<Self, String> {
        let s = s.trim().to_lowercase();

        // Handle castling first (special case)
        if s == "o-o" || s == "O-O" || s == "oo" {
            return Self::parse_castling(board, true); // kingside
        }

        if s == "o-o-o" || s == "O-O-O" || s == "ooo" {
            return Self::parse_castling(board, false); // queenside
        }

        // Long algebraic: e2e4, e7e4, b1c3 etc....

        if s.len() >= 4 && s.len() <= 5 && s.chars().nth(1).unwrap().is_ascii_digit() {
            return Self::parse_long_algebraic(&s, board);
        }

        // Otherwise assume it's SAN ( short algebraic): Nf3, Bxc4, e8=Q, dxe8Q

        Self::parse_san(&s, board)
    }

    fn parse_long_algebraic(s: &str, board: &Board) -> Result<Self, String> {
        if s.len() < 4 {
            return Err("Long algebraic too short".to_string());
        }

        let capture = s.len() == 5;
        let mut idx = 0;

        let from_file = s.chars().nth(idx).ok_or("Missing from file")?;
        let from_rank = s.chars().nth(idx + 1).ok_or("Missing from rank")?;
        if capture {
            idx += 1;
        }
        let to_file = s.chars().nth(idx + 2).ok_or("Missing to file")?;
        let to_rank = s.chars().nth(idx + 3).ok_or("Missing to rank")?;

        let from = algebraic_to_square(&format!("{}{}", from_file, from_rank))?;
        let to = algebraic_to_square(&format!("{}{}", to_file, to_rank))?;

        // let promotion = if s.len() == 5 {
        //     let p = s.chars().nth(4).unwrap();
        //     Some(char_to_promotion(p)?)
        // } else {
        //     None
        // };

        // Basic validation: there should be a piece on 'from'
        if board.get_piece_at(from, board.turn).is_none() {
            return Err(format!("No piece on {}", square_to_algebraic(from)));
        }

        Ok(Move {
            from,
            to,
            promotion: None,
        })
    }

    fn parse_castling(board: &Board, kingside: bool) -> Result<Self, String> {
        let color = board.turn;
        let king_sq = board.king_square(color);

        let expected_to = if kingside {
            if color == Color::White {
                6
            } else {
                62
            }
        } else {
            if color == Color::White {
                2
            } else {
                58
            }
        };

        // we don't check legality here - caller should verify it's in legal moves
        Ok(Move {
            from: king_sq,
            to: expected_to,
            promotion: None,
        })
    }

    fn parse_san(s: &str, board: &Board) -> Result<Self, String> {
        // This is more complex -  we need to:
        // 1. Parse piece type (optional -> pawn)
        // 2. Parse file/rank disambiguation
        // 3. Parse capture 'x'
        // 4. Parse destination square
        // 5. Parse promotion '=Q' or 'q'
        //

        let mut chars = s.chars().peekable();

        // Piece Type
        let piece_char = chars.peek().cloned().unwrap_or('p');
        let piece = match piece_char.to_ascii_uppercase() {
            'N' => PieceType::Knight,
            'B' => PieceType::Bishop,
            'R' => PieceType::Rook,
            'Q' => PieceType::Queen,
            'K' => PieceType::King,
            _ => PieceType::Pawn,
        };

        if piece != PieceType::Pawn {
            chars.next(); // consume piece letter
        }

        // Optional file disambiguation
        let mut from_file: Option<char> = None;
        if let Some(c) = chars.peek() {
            if c.is_ascii_lowercase() && ('a'..='h').contains(c) {
                from_file = Some(*c);
                chars.next();
            }
        }

        // Optional rank disambiguation
        let mut from_rank: Option<char> = None;
        if let Some(c) = chars.peek() {
            if c.is_ascii_digit() {
                from_rank = Some(*c);
                chars.next();
            }
        }

        // Capture 'x' ?
        let is_capture = chars.peek() == Some(&'x');
        if is_capture {
            chars.next();
        }

        // Destination square (required)
        let dest_file = chars.next().ok_or("Missing destination file")?;
        let dest_rank = chars.next().ok_or("Missing destination rank")?;

        let to_square = algebraic_to_square(&format!("{}{}", dest_file, dest_rank))?;

        // promotion
        let mut promotion = None;
        if let Some(c) = chars.next() {
            if c == '=' || c == '+' || c == '#' {
                // skip = or check/checkmate symbols for now
                if let Some(p) = chars.next() {
                    promotion = Some(char_to_promotion(p)?);
                }
            } else {
                promotion = Some(char_to_promotion(c)?);
            }
        }

        // Now find matching move among legal moves
        let legal_moves = board.generate_legal_moves();

        let candidates: Vec<&Move> = legal_moves
            .iter()
            .filter(|m| {
                if m.to != to_square {
                    return false;
                }
                if board.get_piece_at(m.from, board.turn) != Some(piece) {
                    return false;
                }

                // Disambiguation
                if let Some(f) = from_file {
                    if square_to_file(m.from) != f {
                        return false;
                    }
                }

                if let Some(r) = from_rank {
                    if square_to_rank(m.from) != r {
                        return false;
                    }
                }

                // Promotion match
                if m.promotion != promotion {
                    return false;
                }
                true
            })
            .collect();

        match candidates.len() {
            0 => Err(format!("No matching move found for {}", s)),
            1 => Ok(*candidates[0]),
            _ => Err(format!(
                "Ambiguous move : {} ({} possibilities)",
                s,
                candidates.len()
            )),
        }
    }
}

fn square_to_algebraic(sq: Square) -> String {
    let file = (sq % 8) as u8 + b'a';
    let rank = (sq / 8) + 1;
    format!("{}{}", file as char, rank)
}

fn algebraic_to_square(s: &str) -> Result<Square, String> {
    if s.len() != 2 {
        return Err("Algebraic notation must be 2 chars (e4)".to_string());
    }

    let file = s.chars().nth(0).unwrap() as u8;
    let rank = s.chars().nth(1).unwrap() as u8;

    if !('a'..='h').contains(&(file as char)) || !('1'..='8').contains(&(rank as char)) {
        return Err(format!("Invalid square: {}", s));
    }

    let file_idx = file - b'a';
    let rank_idx = rank - b'1';

    Ok(rank_idx * 8 + file_idx)
}

fn char_to_promotion(c: char) -> Result<PieceType, String> {
    match c.to_ascii_lowercase() {
        'q' => Ok(PieceType::Queen),
        'r' => Ok(PieceType::Rook),
        'b' => Ok(PieceType::Bishop),
        'n' => Ok(PieceType::Knight),
        _ => Err(format!("Invalid promotion piece: {}", c)),
    }
}

fn square_to_file(sq: Square) -> char {
    (b'a' + (sq % 8)) as char
}

fn square_to_rank(sq: Square) -> char {
    (b'1' + (sq / 8)) as char
}

impl Board {
    pub fn get_piece_at(&self, sq: Square, color: Color) -> Option<PieceType> {
        let idx = color as usize;
        for pt in 0..6 {
            if get_bit(self.pieces[idx][pt], sq) {
                return PieceType::from_usize(pt);
            }
        }
        None
    }
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
