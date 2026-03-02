use crate::{Board, Color, Move};

const MATE: i32 = 30_000;
const INF: i32 = 1_000_000;
pub fn alpha_beta(board: &mut Board, depth: u8, mut alpha: i32, mut beta: i32) -> i32 {
    if depth == 0 {
        return board.evaluate();
    }

    let moves = board.generate_legal_moves();

    if moves.is_empty() {
        if board.is_in_check(board.turn) {
            return if board.turn == Color::White {
                -MATE
            } else {
                MATE
            };
        }
        return 0;
    }

    if board.turn == Color::White {
        let mut value = -INF;

        for mv in moves {
            let mut new_board = board.make_move(&mv);
            value = value.max(alpha_beta(&mut new_board, depth - 1, alpha, beta));
            alpha = alpha.max(value);
            if alpha >= beta {
                break;
            }
        }
        value
    } else {
        let mut value = INF;
        for mv in moves {
            let mut new_board = board.make_move(&mv);
            value = value.min(alpha_beta(&mut new_board, depth - 1, alpha, beta));
            beta = beta.min(value);
            if beta <= alpha {
                break;
            }
        }
        value
    }
}

pub fn find_best_move(board: &mut Board, depth: u8) -> Move {
    let mut best_move = None;
    let mut best_score = if board.turn == Color::White {
        -INF
    } else {
        INF
    };

    for mv in board.generate_legal_moves() {
        let mut new_board = board.make_move(&mv);
        let score = alpha_beta(&mut new_board, depth - 1, -INF, INF);

        if board.turn == Color::White {
            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }
        } else {
            if score < best_score {
                best_score = score;
                best_move = Some(mv);
            }
        }
    }
    best_move.expect("No legal moves")
}
