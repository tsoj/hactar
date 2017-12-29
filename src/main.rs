#![allow(unused_must_use)]
mod chess_data;
mod position;
mod search;
mod evaluation;
use std::io;
use std::io::Write;

fn test_perft() -> bool
{
    let mut p = position::Position::empty_position();
    p.set_from_fen(&"r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0".to_string());
    if search::perft::start_perft(p, 4) != 4085603
    {
        println!("\nFailed Perft-Test");
        return false;
    }
    true
}

fn main() {
    println!("STARTED!");
    print!("TESTING..."); io::stdout().flush();
    if !test_perft()
    {
        panic!();
    }
    println!("DONE!");
    let mut p = position::Position::empty_position();
    p.set_from_fen(&"r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0".to_string());
    let us =  p.whose_move;
    let enemy = position::player::switch_player(p.whose_move);
    println!("{:x}", p.calculate_zobristkey());
    println!("{}", p.get_chess_board_string());
    let m = search::start_nega_max(p.clone(),6);
    p.make_move(&m, us, enemy);
    println!("{:x}", p.calculate_zobristkey());
    println!("{}", p.get_chess_board_string());

}
