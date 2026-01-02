use crate::constants::*;
use crate::utils::*;

#[derive(Debug, Copy, Clone)]
pub struct Board {
    // Piece bitboards [color][piece_type]
    pub pieces: [[Bitboard; 6]; 2],
    pub occupied: Bitboard, // All occupied squares
    pub turn: Color,
    pub castling_rights: u8, // Bit flags: 0b0000WKQB (White King, White Queen, Black King, Black Queen)
    pub en_passant: Option<Square>,
    pub half_moves: u32,
    pub full_moves: u32,
    // Zobrist hash (added later)
    pub hash: u64,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            pieces: [[0; 6]; 2],
            occupied: 0,
            turn: Color::White,
            castling_rights: 0b1111, // All rights initially
            en_passant: None,
            half_moves: 0,
            full_moves: 1,
            hash: 0,
        };
        board.initialize_start_position();
        board
    }

    fn initialize_start_position(&mut self) {
        // Set white pawns on rank 2 (bits 8 - 15)
        self.pieces[Color::White as usize][PieceType::Pawn as usize] = 0xFF00;
        // White Knights on B1 and G1 (bits 1 and 6)
        self.pieces[Color::White as usize][PieceType::Knight as usize] = (1 << 1) | (1 << 6);

        // Bishops: C1 F1 -> bits 2, 5
        self.pieces[Color::White as usize][PieceType::Bishop as usize] = (1 << 2) | (1 << 5);

        // Rooks: A1 H1 -> bits 0, 7
        self.pieces[Color::White as usize][PieceType::Rook as usize] = (1 << 0) | (1 << 7);

        // Queen: D1 -> bits 3
        self.pieces[Color::White as usize][PieceType::Queen as usize] = 1 << 3;

        // King: E1 -> bits 4
        self.pieces[Color::White as usize][PieceType::King as usize] = 1 << 4;

        // Black pieces mirrored on rank 7-8 (shift by 48 for rank 7 pawns: bits 48-55)
        self.pieces[Color::Black as usize][PieceType::Pawn as usize] = 0xFF000000000000;
        // Black Knights on B8 and G8 (bits 57 and 62)
        self.pieces[Color::Black as usize][PieceType::Knight as usize] = (1 << 57) | (1 << 62);

        // Bishops: C8 F8 -> bits 58, 61
        self.pieces[Color::Black as usize][PieceType::Bishop as usize] = (1 << 58) | (1 << 61);

        // Rooks: A8 H8 -> bits 56, 63
        self.pieces[Color::Black as usize][PieceType::Rook as usize] = (1 << 56) | (1 << 63);

        // Queen: D8 -> bits 59
        self.pieces[Color::Black as usize][PieceType::Queen as usize] = 1 << 59;

        // King: E8 -> bits 60
        self.pieces[Color::Black as usize][PieceType::King as usize] = 1 << 60;

        // Update Occupied
        self.occupied = self.all_pieces(Color::White) | self.all_pieces(Color::Black);
    }

    pub fn all_pieces(&self, color: Color) -> Bitboard {
        let mut bb = 0;
        for pt in 0..6 {
            bb |= self.pieces[color as usize][pt];
        }
        bb
    }

    /// Create a board from a FEN string (only the piece placement part is required for basic use)
    /// Full FEN example: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    ///                     <placement> <turn> <castling_rights> <en_passant> <half_moves> <full_moves>
    pub fn from_fen(fen: &str) -> Result<Self, &'static str> {
        let mut board = Board {
            pieces: [[0; 6]; 2],
            occupied: 0,
            turn: Color::White,
            castling_rights: 0,
            en_passant: None,
            half_moves: 0,
            full_moves: 1,
            hash: 0, // Will be computed later if using Zobrist
        };

        let parts: Vec<&str> = fen.split(' ').collect();
        if parts.is_empty() {
            return Err("Empty FEN");
        }

        let rows: Vec<&str> = parts[0].split('/').collect();
        if rows.len() != 8 {
            return Err("FEN must have 8 ranks");
        }

        for (rank_idx, row) in rows.iter().enumerate() {
            let rank = 7 - rank_idx as u8; // FEN starts from rank 8
            let mut file = 0;
            println!("{}: {}", rank, row);

            for c in row.chars() {
                if file > 7 {
                    return Err("Too many files in rank");
                }

                if let Some(num) = c.to_digit(10) {
                    file += num as u8;
                } else {
                    let (color, piece_type) = match c {
                        'P' => (Color::White, PieceType::Pawn),
                        'N' => (Color::White, PieceType::Knight),
                        'B' => (Color::White, PieceType::Bishop),
                        'R' => (Color::White, PieceType::Rook),
                        'Q' => (Color::White, PieceType::Queen),
                        'K' => (Color::White, PieceType::King),
                        'p' => (Color::Black, PieceType::Pawn),
                        'n' => (Color::Black, PieceType::Knight),
                        'b' => (Color::Black, PieceType::Bishop),
                        'r' => (Color::Black, PieceType::Rook),
                        'q' => (Color::Black, PieceType::Queen),
                        'k' => (Color::Black, PieceType::King),
                        _ => return Err("Invalid piece character"),
                    };

                    let sq = rank * 8 + file;
                    set_bit(&mut board.pieces[color as usize][piece_type as usize], sq);
                    set_bit(&mut board.occupied, sq);

                    file += 1;
                }
            }

            if file != 8 {
                return Err("Rank does not sum to 8 files");
            }
        }

        // Optional: parse side to move
        if parts.len() > 1 {
            board.turn = match parts[1] {
                "w" => Color::White,
                "b" => Color::Black,
                _ => return Err("Invalid side to move"),
            };
        }

        // Optional: castling rights
        if parts.len() > 2 && parts[2] != "-" {
            for c in parts[2].chars() {
                board.castling_rights |= match c {
                    'K' => 0b0001, // White kingside
                    'Q' => 0b0010, // White queenside
                    'k' => 0b0100, // Black kingside
                    'q' => 0b1000, // Black queenside
                    _ => 0,
                };
            }
        }

        // Optional: en passant
        if parts.len() > 3 && parts[3] != "-" {
            board.en_passant = Some(algebraic_to_square(parts[3])?);
        }

        // Optional: halfmove and fullmove
        if parts.len() > 4 {
            board.half_moves = parts[4].parse().unwrap_or(0);
        }
        if parts.len() > 5 {
            board.full_moves = parts[5].parse().unwrap_or(1);
        }

        Ok(board)
    }
}

/// Helper: convert algebraic notation like "e4" to square index (0-63)
fn algebraic_to_square(alg: &str) -> Result<Square, &'static str> {
    if alg.len() != 2 {
        return Err("Invalid algebraic notation");
    }
    let chars: Vec<char> = alg.chars().collect();
    let file = chars[0] as u8 - b'a';
    let rank = chars[1] as u8 - b'1';
    if file > 7 || rank > 7 {
        return Err("Out of bounds");
    }
    Ok(rank * 8 + file)
}
