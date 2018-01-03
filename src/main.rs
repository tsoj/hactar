
mod chess_data;
mod position;
mod search;
mod evaluation;

use search::Searcher;
use search::perft;
use position::Position;

fn main()
{
    println!("STARTED!");
    perft::test_perft();

    let mut p = Position::empty_position();
    p.set_from_fen(&"r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0".to_string());
    //p.set_from_fen(&"2r1r3/pp1k2pp/8/3b4/4P3/4K3/PP4PP/R5R1 w - - 0 26".to_string());
    //p.set_from_fen(&"5r1k/1p1b1p1p/p2ppb2/5P1B/1q6/1Pr3R1/2PQ2PP/5R1K w - - 0 1".to_string());//CHECKMATE in 4
    //p.set_from_fen(&"5r1k/1p1b1p1p/p2ppb1Q/5P1B/1q6/1Pr3R1/2P3PP/5R1K b - - 1 1".to_string());//CHECKMATED in 3.5
    //p.set_from_fen(&"2r2rk1/pp2q2b/2p5/2Pp1pp1/1P1N3P/P1Q1PP2/4RK1P/7R b - - 0 30".to_string());
    //p.set_from_fen(&position::START_POS_FEN.to_string());
    println!("{}", p.get_chess_board_string());
    let m = Searcher::go(&p, search::MAX_DEPTH);
    p.make_move(&m);
    println!("{}", p.get_chess_board_string());
}
