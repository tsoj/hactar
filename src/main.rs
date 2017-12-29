mod chess_data;
mod position;
mod search;
mod evaluation;
mod score;

fn main() {
    let mut p = position::Position::empty_position();
    p.set_from_fen(&"r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0".to_string());
    //p.set_from_fen(&"1r2k3/8/8/2pP4/8/8/8/4K3 w - c6 0 1".to_string());
    //p.set_from_fen(&"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());
    //println!("{}", p.get_data_string());
    //println!("{}", search::start_perft(p.clone(), 5));
    search::start_nega_max_search(p.clone(),6);
    //println!("{}", p.get_all_pseudo_legal_moves_string());

}
