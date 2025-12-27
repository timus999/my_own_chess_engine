# Building a Chess Engine in Rust: A Step-by-Step Roadmap

## Motivation and Reason for Starting This Project

The development of a chess engine is an exciting endeavor that combines computer science, artificial intelligence, and game theory. My motivation for starting this project stems from a passion for Rust programming and a desire to deepen my understanding of efficient algorithms in game AI. Chess engines provide a perfect sandbox for exploring concepts like bit manipulation, search optimization, and protocol integration, which are transferable to other domains such as robotics, decision-making systems, and even financial modeling.

Reasons for embarking on this:
- **Learning Rust**: Rust's emphasis on safety, performance, and concurrency makes it ideal for performance-critical applications like chess engines. This project allows hands-on experience with Rust's ownership model, enums, and bit operations.
- **AI and Game Theory**: Implementing minimax and alpha-beta pruning offers insights into adversarial search, which is foundational in AI.
- **Open-Source Contribution**: Inspired by engines like Stockfish and AlphaZero, this project aims to create a simple, educational engine that beginners can build upon.
- **Personal Challenge**: Chess is a complex game with 10^120 possible positions (Shannon number), pushing the limits of computational efficiency.
- **Practical Applications**: Beyond games, the techniques (e.g., hashing, transposition) apply to optimization problems in real-world scenarios.

This roadmap outlines the key components, building incrementally from board setup to a fully functional engine.

## 1. Board Representation with Bitboards

Bitboards represent the chessboard using 64-bit integers (u64 in Rust), where each bit corresponds to a square (A1=bit 0, H8=bit 63). This is efficient for operations like moves and attacks via bitwise operations.

### Key Implementation:
- **Structures**: Use enums for `Color` (White/Black) and `PieceType` (Pawn, Knight, etc.). The `Board` struct holds 12 bitboards (6 piece types per color), occupied squares, turn, castling rights, en passant, and counters.
- **Initialization**: Set starting positions using bitmasks (e.g., white pawns: `0xFF00`).
- **Helpers**: Functions like `set_bit`, `get_bit`, `pop_lsb` for manipulation.
- **FEN Parsing**: Optional `from_fen` method to load positions.

Visual Representation:

[![basic of neural network](https://miro.medium.com/v2/resize:fit:1100/format:webp/0*c2AdbOlPnjPMprwC)](https://ryan-leong.medium.com/the-basics-of-neural-networks-for-chess-nerds-and-anyone-interested-in-ai-e76e03124867)


This compact representation enables fast queries and updates, crucial for deep searches.

## 2. Basic Move Generation (Pseudo-Legal)

Pseudo-legal moves generate all possible moves ignoring king safety (e.g., pins, checks). They are filtered later for legality.

### Key Implementation:
- **Move Struct**: `{ from: Square, to: Square, promotion: Option<PieceType> }`.
- **Per-Piece Generation**:
  - **Pawns**: Single/double pushes (`bb << 8`), captures (`<<7/<<9`), promotions, en passant.
  - **Knights/Kings**: Precomputed attack tables (arrays of bitboards).
  - **Sliders (Bishops/Rooks/Queens)**: Ray attacks in directions, stopping at blockers.
- **Output**: Vec<Move> for the current turn.

Example Code Snippet:
```rust
impl Board {
    pub fn generate_pseudo_moves(&self) -> Vec<Move> {
        // ... (as provided earlier, with pawn pushes, captures, knight attacks, etc.)
    }
}
```

This step focuses on efficiency, using bit shifts and masks to avoid loops where possible.

## 3. Legal Move Generation

Legal moves filter pseudo-legal ones to ensure the king isn't left in check.

### Key Implementation:
- Generate pseudo-moves.
- For each: Clone board, apply move, check if king is attacked (`is_in_check`).
- **is_in_check**: Verify if opponent attacks hit the king square using attack generators.
- **apply_move**: Update bitboards, handle specials (castling, en passant), update rights/counters.

Example:
```rust
impl Board {
    pub fn generate_legal_moves(&self) -> Vec<Move> {
        let pseudo = self.generate_pseudo_moves();
        pseudo.into_iter().filter(|m| {
            let mut temp = self.clone();
            temp.apply_move(m);
            !temp.is_in_check(self.turn)
        }).collect()
    }
}
```

This ensures valid gameplay, though cloning adds overhead (optimize later with undo).

## 4. Search Algorithm (Minimax & Alpha-Beta)

Search finds the best move by exploring game trees.

### Minimax
Recursively evaluates positions: Maximizer (white) chooses max score, minimizer (black) chooses min.

Example: At depth 3, evaluate leaves with material score, backpropagate.

Visual Representation:

![graph](https://muens.io/img/chess-minimax-2-800w.jpg) ![chess_graph](http://zackmdavis.net/blog/wp-content/uploads/2019/05/game_tree.png)






Code:
```rust
fn minimax(&mut self, depth: u32) -> i32 {
    if depth == 0 { return self.evaluate(); }
    // Generate moves, recurse, max/min scores
}
```

### Alpha-Beta Pruning
Optimizes minimax by pruning branches that can't improve the score (alpha: best max, beta: best min).

Example: If a max node finds a value > beta, prune; saves ~50-90% computations.

Visual Representation:

![alhpa_beta_pruning](https://daxg39y63pxwu.cloudfront.net/images/blog/alpha-beta-pruning-in-ai/Alpha_Beta_Purning_in_AI.webp) 
![alpha_beta_pruning_in_chess](https://figures.semanticscholar.org/3c5cdc5fb590fc56ac429884216f8e0ce31c8164/3-Figure2-1.png`)






Code (Negamax variant):
```rust
fn alpha_beta(&mut self, depth: u32, mut alpha: i32, mut beta: i32) -> i32 {
    // Probe TT, base case, generate moves, recurse with -beta/-alpha
}
```

## 5. Optimizations (Zobrist Hashing and Transposition Table)

To handle exponential tree growth.

### Zobrist Hashing
Idea: Unique 64-bit key for each board state via XOR of random keys for pieces/squares, castling, etc. Updated incrementally.

Why: Detects repeated positions for three-fold repetition; keys transposition tables.

Visual (Game Tree with Hashes):

![game_tree](https://www.dogeystamp.com/public/img/chess4/example_gametree.svg)


Code: Precompute keys, XOR in `apply_move`.

### Transposition Table
Idea: HashMap<hash, (score, depth, flag)> to cache evaluations.

Why: Avoids recomputing identical subtrees; speeds up by 10-100x.

Code: Probe/store in alpha-beta.

## 6. Interface (UCI Protocol)

Brief Introduction: Universal Chess Interface is a text protocol for engine-GUI communication (e.g., CuteChess). Commands like "go" trigger search, output "bestmove".

Implementation: Loop reading stdin, parse commands, respond (e.g., search and print move in algebraic notation).

Code Snippet:
```rust
fn main() {
    // Read lines, handle "uci", "position", "go", etc.
}
```

This makes the engine pluggable into any UCI-compatible GUI.

## 7. References

- Chess Programming Wiki: https://www.chessprogramming.org/
- Open-Source Engines: Pleco (Rust), Rustic (Rust), Stockfish (C++).
- Tutorials: "Chess Engine in Rust" series on Medium.
- Books: "Game Programming Patterns" by Robert Nystrom.
- Images sourced from web searches for educational purposes.
