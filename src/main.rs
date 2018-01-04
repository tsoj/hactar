
mod chess_data;
mod position;
mod search;
mod evaluation;

use search::Searcher;
use search::perft;
use position::{Position, piece};
use position::mov::Move;
use position::piece::{PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING, NO_PIECE};
use search::transposition_table::TranspositionTable;

use std::io;
use std::io::prelude::*;

const CHESS_ENGINE_NAME: &'static str = "hactar";
const CHESS_ENGINE_AUTHOR: &'static str = "Tsoj Tsoj";
pub const STARTPOS_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn uci()
{
    println!("id name {}", CHESS_ENGINE_NAME);
    println!("id author {}", CHESS_ENGINE_AUTHOR);
    println!("uciok");
}
fn set_position(position: &mut Position, mut params: std::str::SplitWhitespace)
{
    let parameter = params.next().unwrap();
    match parameter
    {
        "startpos" =>
        {
            position.set_from_fen(&STARTPOS_FEN.to_string());
        },
        "fen" =>
        {
            let mut fen_string = "".to_string();
            fen_string += params.next().unwrap();
            fen_string += " ";
            fen_string += params.next().unwrap();
            fen_string += " ";
            fen_string += params.next().unwrap();
            fen_string += " ";
            fen_string += params.next().unwrap();
            fen_string += " ";
            fen_string += params.next().unwrap();
            fen_string += " ";
            fen_string += params.next().unwrap();
            fen_string += " ";
            position.set_from_fen(&fen_string);
        },
        _x => println!("Unknown parameter: {}", parameter)
    }
    let parameter;
    match params.next()
    {
        Some(x) => parameter = x,
        None => return
    }
    match parameter
    {
        "moves" =>
        {
            for m in params
            {
                let new_move = &get_move(&m.to_string(), &position);
                position.make_move(&new_move);
            }
        },
        _x => println!("Unknown parameter: {}", parameter)
    }

}
fn get_move(m: &String, position: &Position) -> Move
{
    let us = position.us;
    let enemy = position.enemy;
    let mut new_move = Move::empty_move();
    new_move.from = chess_data::get_field_index(&m[0..2]);
    new_move.to = chess_data::get_field_index(&m[2..4]);
    new_move.to = chess_data::get_field_index(&m[2..4]);
    if m.len()==5
    {
        new_move.promoted = piece::get_piece(&m[4..5]);
    }
    for i in 0..NO_PIECE
    {
        if position.pieces[i] & chess_data::BIT_AT_INDEX[new_move.from] != 0
        {
            new_move.moved = i;
        }
        if position.pieces[i] & chess_data::BIT_AT_INDEX[new_move.to] != 0
        {
            new_move.captured = i;
        }
    }
    new_move.en_passant_castling = position.en_passant_castling & (chess_data::RANKS[0] | chess_data::RANKS[7]);
    //captured en passant
    if
        new_move.moved == PAWN &&
        chess_data::BIT_AT_INDEX[new_move.to] & (position.en_passant_castling & (chess_data::RANKS[2] | chess_data::RANKS[5])) != 0
    {
        new_move.captured_en_passant = true;
    }
    //pawn double push
    if
        new_move.moved == PAWN &&
        chess_data::BIT_AT_INDEX[new_move.from] & chess_data::PAWN_HOME_RANK[us] != 0 &&
        chess_data::BIT_AT_INDEX[new_move.to] != (chess_data::PAWN_QUIET_ATTACK_TABLE[us][new_move.from] | chess_data::PAWN_CAPTURE_ATTACK_TABLE[us][new_move.from])
    {
        new_move.en_passant_castling |= chess_data::PAWN_QUIET_ATTACK_TABLE[us][new_move.from];
    }
    //castling
    //queenside
    if
        new_move.from == chess_data::CASTLING_KING_FROM_INDEX[us] &&
        new_move.to == chess_data::CASTLING_QUEENSIDE_KING_TO_INDEX[us] &&
        position.en_passant_castling & chess_data::CASTLING_QUEENSIDE_ROOK_FROM[us] != 0 &&
        position.en_passant_castling & chess_data::CASTLING_KING_FROM[us] != 0
    {
        new_move.en_passant_castling &= !(chess_data::CASTLING_QUEENSIDE_ROOK_FROM[us] | chess_data::CASTLING_KING_FROM[us]);
        new_move.castled = true;
    }
    //kingside
    if
        new_move.from == chess_data::CASTLING_KING_FROM_INDEX[us] &&
        new_move.to == chess_data::CASTLING_KINGSIDE_KING_TO_INDEX[us] &&
        position.en_passant_castling & chess_data::CASTLING_KINGSIDE_ROOK_FROM[us] != 0 &&
        position.en_passant_castling & chess_data::CASTLING_KING_FROM[us] != 0
    {
        new_move.en_passant_castling &= !(chess_data::CASTLING_KINGSIDE_ROOK_FROM[us] | chess_data::CASTLING_KING_FROM[us]);
        new_move.castled = true;
    }
    new_move.zobrist_key = position.get_updated_zobristkey(&new_move);
    new_move
}
fn go( position: &Position, params: std::str::SplitWhitespace)
{
    Searcher::go(&position, 13);
}
fn stop()
{
}
fn print(position: &Position)
{
    println!("{}",position.get_chess_board_string());
}

fn main()
{
    /*
    println!("STARTED!");
    perft::test_perft();
*/
    let mut position = Position::empty_position();
    let stdin = io::stdin();
    for line in stdin.lock().lines()
    {
        let line = line.unwrap_or("".into());
        let mut params = line.split_whitespace();
        let command = params.next().unwrap();
        match command
        {
            "uci" => uci(),
            "isready" => println!("readyok"),
            "position" => set_position(&mut position, params),
            "go" => go(&position, params),
            "stop" => stop(),
            "quit" => { stop(); return },
            "print" => print(&position),
            _x => println!("Unknown command: {}", command)
        }
    }

    /*let mut p = Position::empty_position();
    p.set_from_fen(&"r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0".to_string());
    //p.set_from_fen(&"2r1r3/pp1k2pp/8/3b4/4P3/4K3/PP4PP/R5R1 w - - 0 26".to_string());
    //p.set_from_fen(&"5r1k/1p1b1p1p/p2ppb2/5P1B/1q6/1Pr3R1/2PQ2PP/5R1K w - - 0 1".to_string());//CHECKMATE in 4
    //p.set_from_fen(&"5r1k/1p1b1p1p/p2ppb1Q/5P1B/1q6/1Pr3R1/2P3PP/5R1K b - - 1 1".to_string());//CHECKMATED in 3.5
    //p.set_from_fen(&"2r2rk1/pp2q2b/2p5/2Pp1pp1/1P1N3P/P1Q1PP2/4RK1P/7R b - - 0 30".to_string());
    //p.set_from_fen(&position::START_POS_FEN.to_string());
    println!("{}", p.get_chess_board_string());
    let m = Searcher::go(&p, search::MAX_DEPTH);
    p.make_move(&m);
    println!("{}", p.get_chess_board_string());*/
}
