mod chess_data;
mod position;

fn main() {
    let mut p = position::Position::empty_position();
    //p.set_from_fen(&"rnbqkbnr/ppp1pppp/8/8/8/8/PPPpPPPP/RNBQKBNR w KQkq - 0 0".to_string());
    p.set_from_fen(&"r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0".to_string());
    println!("{:x}", p.zobrist_key);
    let ml = p.generate_move_list(position::player::WHITE, position::player::BLACK);
    for i in 0..ml.len
    {
        println!("----------------------------------------");
        let mut next_p = p.clone();
        let b_e_p_c = next_p.make_move(&ml[i], position::player::WHITE, position::player::BLACK);
        println!("{}", next_p.get_chess_board_string());
        println!("{:x}", next_p.zobrist_key);
        next_p.undo_move(&ml[i], b_e_p_c, position::player::WHITE, position::player::BLACK);
        println!("{}", next_p.get_chess_board_string());
        println!("{:x}", next_p.zobrist_key);
    }
}
