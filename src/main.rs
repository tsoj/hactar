
mod chess_data;
mod position;
mod search;
mod evaluation;

use search::Searcher;
use search::perft;
use search::Depth;
use position::{Position, piece};
use position::mov::Move;
use position::piece::{PAWN, NO_PIECE};

use std::io;
use std::io::prelude::*;
use std::thread;
use std::time::SystemTime;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

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
    let mut new_move = Move::empty_move();
    new_move.from = chess_data::get_field_index(&m[0..2]);
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
        chess_data::BIT_AT_INDEX[new_move.to] & (chess_data::PAWN_QUIET_ATTACK_TABLE[us][new_move.from] | chess_data::PAWN_CAPTURE_ATTACK_TABLE[us][new_move.from]) == 0
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
    new_move.en_passant_castling &= !(chess_data::BIT_AT_INDEX[new_move.from] & (chess_data::RANKS[0] | chess_data::RANKS[7]));
    new_move.en_passant_castling &= !(chess_data::BIT_AT_INDEX[new_move.to] & (chess_data::RANKS[0] | chess_data::RANKS[7]));
    new_move.zobrist_key = position.get_updated_zobristkey(&new_move);
    new_move
}
fn go(position: &Position, params: std::str::SplitWhitespace, should_stop: &mut Arc<AtomicBool>)
{
    #![allow(unused_variables)]
    #![allow(unused_assignments)]
    let mut depth = search::MAX_DEPTH;
    let mut wtime: Option<usize> = None;
	let mut btime: Option<usize> = None;
	let mut winc: Option<usize> = None;
	let mut binc: Option<usize> = None;
	let mut movestogo: Option<usize> = None;
    let mut movetime: Option<usize> = None;
    let mut nodes: Option<usize> = None;
    let mut parameter_type: Option<&str> = None;
    for parameter in params
    {
        match parameter_type
        {
            Some(x) =>
            {
                match x
                {
                    "depth" => depth = parameter.parse::<Depth>().unwrap(),
                    "wtime" => wtime = Some(parameter.parse::<usize>().unwrap()),
                    "btime" => btime = Some(parameter.parse::<usize>().unwrap()),
                    "winc" => winc = Some(parameter.parse::<usize>().unwrap()),
                    "binc" => binc = Some(parameter.parse::<usize>().unwrap()),
                    "movestogo" => movestogo = Some(parameter.parse::<usize>().unwrap()),
                    "movetime" => movetime = Some(parameter.parse::<usize>().unwrap()),
                    "nodes" => nodes = Some(parameter.parse::<usize>().unwrap()),
                    _x => println!("Unknown parameter: {}", _x)
                }
            },
            None =>
            {
                parameter_type = Some(parameter);
            }
        }
    }
    #[warn(unused_variables)]
    #[warn(unused_assignments)]

    should_stop.store(false, Ordering::Relaxed);
    let mut time_limit: Option<usize> = None;
    match movetime
    {
        Some(x) => time_limit = Some(x),
        None => {}
    }
    match time_limit
    {
        Some(x) =>
        {
            let temp_should_stop = Arc::clone(&should_stop);
            thread::spawn(move || { stop_in(x, temp_should_stop) });
        },
        None => {}
    }
    let temp_position = position.clone();
    let temp_should_stop = Arc::clone(&should_stop);
    thread::spawn(move || { Searcher::go(temp_position, depth, temp_should_stop) });
}
fn stop_in(miliseconds: usize, should_stop: Arc<AtomicBool>)
{
    let now = SystemTime::now();
    let mut time = 0;
    while time <= miliseconds && should_stop.load(Ordering::Relaxed) == false
    {
        match now.elapsed()
        {
            Ok(elapsed) =>
            {
                time = ((format!("{}.{}", elapsed.as_secs(), elapsed.subsec_nanos())).parse::<f32>().unwrap()*1000.0) as usize;
            }
            Err(e) =>
            {
                println!("Error: {:?}", e);
                panic!();
            }
        }
    }
    should_stop.store(true, Ordering::Relaxed);
}
fn stop(should_stop: &mut Arc<AtomicBool>)
{
    should_stop.store(true, Ordering::Relaxed);
}
fn print(position: &Position)
{
    println!("{}",position.get_chess_board_string());
}
fn print_debug(position: &Position)
{
    println!("{}",position.get_data_string());
}
fn eval(position: &Position)
{
    println!("{}",position.evaluate());
}

fn main()
{

    println!("STARTED!");
    perft::test_perft();

    let mut should_stop = Arc::new(AtomicBool::new(true));
    let mut position = Position::empty_position();
    let stdin = io::stdin();
    for line in stdin.lock().lines()
    {
        //TODO: dont crash without command and enter...
        let line = line.unwrap_or("".into());
        let mut params = line.split_whitespace();
        let command;
        match params.next()
        {
            Some(x) => command = x,
            None => continue
        }
        match command
        {
            "uci" => uci(),
            "isready" => println!("readyok"),
            "position" => set_position(&mut position, params),
            "go" => go(&position, params, &mut should_stop),
            "eval" => eval(&position),
            "stop" => stop(&mut should_stop),
            "quit" => { stop(&mut should_stop); return },
            "print" => print(&position),
            "printdebug" => print_debug(&position),
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
