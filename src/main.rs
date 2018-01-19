
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
use position::player::{WHITE};

use std::io;
use std::io::prelude::*;
use std::thread;
use std::time::SystemTime;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

const CHESS_ENGINE_NAME: &'static str = "hactar";
const CHESS_ENGINE_AUTHOR: &'static str = "Tsoj Tsoj";
pub const STARTPOS_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Clone, Copy)]
pub struct Options
{
    pub transposition_table_size_mb: usize
}

fn uci()
{
    println!("id name {}", CHESS_ENGINE_NAME);
    println!("id author {}", CHESS_ENGINE_AUTHOR);
    println!("");
    println!("option name Hash type spin default 128 min 1 max 1048576");
    println!("");
    println!("uciok");
}
fn set_position(position: &mut Position, mut params: std::str::SplitWhitespace,history: &mut Vec<Position>)
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
    history.clear();
    history.push(position.clone());
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
                history.push(position.clone());
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
fn go(position: &Position, params: std::str::SplitWhitespace, options: &Options, should_stop: &mut Arc<AtomicBool>, history: &Vec<Position>)
{
    #![allow(unused_variables)]
    #![allow(unused_assignments)]
    let mut depth = search::MAX_DEPTH;
    let mut wtime: Option<i64> = None;
	let mut btime: Option<i64> = None;
	let mut winc: Option<i64> = None;
	let mut binc: Option<i64> = None;
	let mut movestogo: Option<i64> = None;
    let mut movetime: Option<i64> = None;
    let mut nodes: Option<i64> = None;
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
                    "wtime" => wtime = Some(parameter.parse::<i64>().unwrap()),
                    "btime" => btime = Some(parameter.parse::<i64>().unwrap()),
                    "winc" => winc = Some(parameter.parse::<i64>().unwrap()),
                    "binc" => binc = Some(parameter.parse::<i64>().unwrap()),
                    "movestogo" => movestogo = Some(parameter.parse::<i64>().unwrap()),
                    "movetime" => movetime = Some(parameter.parse::<i64>().unwrap()),
                    "nodes" => nodes = Some(parameter.parse::<i64>().unwrap()),
                    _x => println!("Unknown parameter: {}", _x)
                };
                parameter_type = None;
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
    match movetime
    {
        Some(x) =>
        {
            let temp_should_stop = Arc::clone(&should_stop);
            let child = thread::Builder::new().name("timer".to_string()).spawn(move || { stop_in(x, temp_should_stop) });
        },
        None => {}
    }
    if movestogo == None
    {
        let averange_game_length = 80;
        movestogo = Some(averange_game_length - position.fullmoves_played as i64);
        if movestogo.unwrap() < 10
        {
            movestogo = Some(10)
        }
    }
    let time_per_move;
    if position.us == WHITE
    {
        if wtime == None && winc == None
        {
            time_per_move = -1;
        }
        else
        {
            if winc == None
            {
                winc = Some(0);
            }
            if wtime == None
            {
                wtime = Some(0);
            }
            time_per_move = wtime.unwrap()/movestogo.unwrap() + winc.unwrap();
        }
    }
    else
    {
        if btime == None && binc == None
        {
            time_per_move = -1;
        }
        else
        {
            if binc == None
            {
                binc = Some(0);
            }
            if btime == None
            {
                btime = Some(0);
            }
            time_per_move = btime.unwrap()/movestogo.unwrap() + binc.unwrap();
        }
    }

    let temp_position = position.clone();
    let temp_should_stop = Arc::clone(&should_stop);
    let temp_options =  options.clone();
    let temp_history = history.clone();
    let child = thread::Builder::new().name("search".to_string()).spawn(move || { Searcher::go(temp_position, depth, temp_options, temp_should_stop, time_per_move, temp_history) });
}
fn stop_in(miliseconds: i64, should_stop: Arc<AtomicBool>)
{
    let now = SystemTime::now();
    let mut time = 0;
    while time <= miliseconds && should_stop.load(Ordering::Relaxed) == false
    {
        match now.elapsed()
        {
            Ok(elapsed) =>
            {
                time = ((format!("{}.{}", elapsed.as_secs(), elapsed.subsec_nanos())).parse::<f64>().unwrap()*1000.0) as i64;
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
fn print_fen(position: &Position)
{
    println!("{}",position.get_fen_string());
}
fn print_debug(position: &Position)
{
    println!("{}",position.get_data_string());
}
fn set_option(options: &mut Options, mut params: std::str::SplitWhitespace)
{
    match params.next()
    {
        Some(x) =>
        {
            if x != "name"
            {
                println!("Unknown parameter: {}", x);
                return;
            }
        },
        None => return
    }
    match params.next()
    {
        Some("Hash") =>
        {
            match params.next()
            {
                Some("value") =>
                {
                    match params.next()
                    {
                        Some(x) =>
                        {
                            options.transposition_table_size_mb = x.parse::<usize>().unwrap();
                        },
                        None => return
                    }
                },
                Some(x) =>
                {
                    println!("Unknown parameter: {}", x);
                    return;
                },
                None => return
            }
        },
        Some(x) =>
        {
            println!("Unknown parameter: {}", x);
            return;
        },
        None => return
    }
}

fn main()
{
    perft::test_perft();

    let stdin = io::stdin();
    let mut position = Position::empty_position();
    let mut history: Vec<Position> = Vec::new();
    let mut should_stop = Arc::new(AtomicBool::new(false));
    let mut options = Options{transposition_table_size_mb: 128};
    position.set_from_fen(&STARTPOS_FEN.to_string());
    for line in stdin.lock().lines()
    {
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
            "position" => set_position(&mut position, params, &mut history),
            "go" => go(&position, params, &options, &mut should_stop, &history),
            "stop" => stop(&mut should_stop),
            "quit" => { stop(&mut should_stop); return },
            "print" => print(&position),
            "printfen" => print_fen(&position),
            "printdebug" => print_debug(&position),
            "setoption" => set_option(&mut options, params),
            "ucinewgame" => {},
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
