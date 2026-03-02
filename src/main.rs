use std::io;

use my_own_chess_engine::attack::*;
use my_own_chess_engine::constants::*;
use my_own_chess_engine::core_algo::alpha_beta::find_best_move;
use my_own_chess_engine::initialize_board::*;
use my_own_chess_engine::utils::*;
use my_own_chess_engine::Move;

fn main() {
    // let mut bb: Bitboard = 0;
    // set_bit(&mut bb, 0); // Set A1
    // assert!(get_bit(bb, 0));

    // Initial chess position
    // let mut board =
    //     Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

    let mut board =
        Board::from_fen("r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 1 3")
            .unwrap();
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to input move");

        if input == "q" {
            break;
        }
        println!("Your move: {}", input);
        board.turn = Color::White;

        let user_move = Move::from_algebric(input.as_str(), &board).unwrap();
        board.apply_move(&user_move);

        board.turn = Color::Black;
        let best_move = find_best_move(&mut board, 3);
        println!("Black best move : {}", best_move.to_long_algebraic(&board));
        board.apply_move(&best_move);
    }

    // Lone kine in the center
    // let board = Board::from_fen("8/8/8/3k4/8/8/8/8 b - - 0 1").unwrap();

    // Bishop sliding test (blockers)
    // let board = Board::from_fen("8/8/8/3p4/4B3/8/8/4K3 w - - 0 1").unwrap();

    // Rook open file
    // let board = Board::from_fen("8/8/8/8/4R3/8/8/4K3 w - - 0 1").unwrap();

    // Pawn promotion
    // let board = Board::from_fen("3K4/8/8/8/8/8/7P/8 w - - 0 1").unwrap();

    // en passant
    // let board = Board::from_fen("8/8/8/3pP3/8/8/8/3K4 w - d6 0 1").unwrap();
    // let mut board =
    //     Board::from_fen("r1bqk2r/ppp2ppp/2n2n2/3pp3/3PP3/2NB1N2/PPPQ1PPP/R3K2R w KQkq - 0 1")
    //         .unwrap();

    // let mut board =
    //     Board::from_fen("r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 1 3")
    //         .unwrap();
    // println!("Starting position loaded!");
    // board.print_board();
    // let moves = board.generate_legal_moves();
    // let qh5 = moves
    //     .iter()
    //     .find(|m| m.from == 3 && m.to == 41 && m.promotion.is_none()); // d1h5

    // for m in moves.clone() {
    //     print!("{}, ", m.to_long_algebraic(&board));
    // }

    // let best_move = moves[0];
    // println!("Best move: {}", best_move.to_long_algebraic(&board));

    // apply the best move

    // println!("Board after move:");
    // board.print_board();

    // board.apply_move(&best_move);
    // println!("{:?}", pseudo_moves);
}
