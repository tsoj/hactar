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
    //p.set_from_fen(&"2r1r3/pp1k2pp/8/3b4/4P3/4K3/PP4PP/R5R1 w - - 0 26".to_string());
    println!("{}", p.get_chess_board_string());
    let m = search::Searcher::go(&p, 8);
    p.make_move(&m);
    println!("{}", p.get_chess_board_string());
}
