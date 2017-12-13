mod chess_data;
mod position;

fn main() {
    println!("{}", chess_data::BLACK_PAWN_UNICODE);
    println!("{}", position::get_bitboard_string(12345));
}
