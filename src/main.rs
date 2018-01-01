#![allow(unused_must_use)]
mod chess_data;
mod position;
mod search;
mod evaluation;
use std::io;
use std::io::Write;

fn main()
{
    println!("STARTED!");
    print!("TESTING..."); io::stdout().flush();
    if !search::perft::test_perft()
    {
        panic!();
    }
    println!("DONE!");

    let mut p = position::Position::empty_position();
    p.set_from_fen(&"r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0".to_string());
    //p.set_from_fen(&"5r1k/1p1b1p1p/p2ppb2/5P1B/1q6/1Pr3R1/2PQ2PP/5R1K w - - 0 1".to_string());
    println!("{}", p.get_chess_board_string());
    let m = search::start_nega_max(p.clone(), 8);
    let us =  p.whose_move;
    let enemy = position::player::switch_player(p.whose_move);
    p.make_move(&m, us, enemy);
    println!("{}", p.get_chess_board_string());
}
