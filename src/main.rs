mod chess_data;
mod position;

fn main() {
    //println!("{}", chess_data::BLACK_PAWN_UNICODE);
    //println!("{}", position::get_bitboard_string(12345));
    let mut p = position::Position::empty_position();
    p.set_from_fen(&"rnbqkbnr/pp2pppp/2p5/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3".to_string());
    print!("{}", p.get_chess_board_string());
    print!("{}", p.get_data_string());
}
