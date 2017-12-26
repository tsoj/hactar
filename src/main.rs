mod chess_data;
mod position;

fn main() {
    let mut p = position::Position::empty_position();
    p.set_from_fen(&"rnbqkbnr/ppp1pppp/8/8/8/8/PPPpPPPP/RNBQKBNR w KQkq - 0 0".to_string());
    let ml = p.generate_move_list(position::player::WHITE, position::player::BLACK);
    for i in 0..ml.len
    {
        println!("{}\n", ml[i].get_data_string());
    }
}
