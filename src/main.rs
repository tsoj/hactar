mod chess_data;
mod position;

fn main() {
    //println!("{}", chess_data::BLACK_PAWN_UNICODE);
    //println!("{}", position::get_bitboard_string(12345));
    let mut p = position::Position::empty_position();
    //p.set_from_fen(&"rnbqkbnr/pp2pppp/2p5/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3".to_string());
    p.set_from_fen(&"rnbqkbnr/pppppppp/8/8/8/3P4/PPP1PPPP/RNBQKBNR b KQkq - 0 0".to_string());
    print!("{}", p.get_chess_board_string());
    print!("{}", p.get_data_string());
    p.set_from_fen(&"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0".to_string());
    println!("{:x}", p.update_zobristkey(&position::Move{from: 11, to: 19, moved: position::piecetype::PAWN, captured: position::piecetype::NO_PIECE, promoted: position::piecetype::NO_PIECE, en_passant_castling: p.en_passant_castling, zobrist_key: 0}));
}
