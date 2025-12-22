use my_own_chess_engine::attack::*;
use my_own_chess_engine::constants::*;
use my_own_chess_engine::initialize_board::*;
use my_own_chess_engine::utils::*;

fn main() {
    let mut bb: Bitboard = 0;
    set_bit(&mut bb, 0); // Set A1
    assert!(get_bit(bb, 0));

    let mut bb2 = RANK_2;
    while let Some(sq) = pop_lsb(&mut bb2) {
        println!("Pawn on square {}", sq);
    }

    // Initial chess position
    // let board =
    //     Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

    // Lone kine in the center
    // let board = Board::from_fen("8/8/8/3k4/8/8/8/8 b - - 0 1").unwrap();

    // Bishop sliding test (blockers)
    // let board = Board::from_fen("8/8/8/3p4/4B3/8/8/4K3 w - - 0 1").unwrap();

    // Rook open file
    // let board = Board::from_fen("8/8/8/8/4R3/8/8/4K3 w - - 0 1").unwrap();

    // Pawn promotion
    // let board = Board::from_fen("3K4/8/8/8/8/8/7P/8 w - - 0 1").unwrap();

    // en passant
    let board = Board::from_fen("8/8/8/3pP3/8/8/8/3K4 w - d6 0 1").unwrap();
    println!("Starting position loaded!");
    println!("{:?}", board);
    let pseudo_moves = board.generate_pseudo_moves();
    for m in pseudo_moves.clone() {
        print!("{}, ", m.to_long_algebraic(&board));
    }

    // println!("");
    // println!("{:?}", pseudo_moves);
}
