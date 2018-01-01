#![allow(dead_code)]
use position;
use evaluation;
use std;
pub mod transposition_table;
pub mod perft;

pub type Depth = i32;

fn nega_max(
    nodes: &mut u64,
    us: position::player::Player,
    enemy: position::player::Player,
    orig_position: &position::Position,
    depth: Depth,
    mut alpha: evaluation::score::Score,
    beta: evaluation::score::Score,
    transposition_table: &mut Vec<transposition_table::TranspositionTableElement>
) -> evaluation::score::Score
{
    *nodes += 1;
    if depth==0
    {
        return evaluation::evaluate(&orig_position, us, enemy);
    }
    let mut current_score: evaluation::score::Score;
    let mut number_legal_moves = 0;
    let mut move_list = orig_position.generate_move_list(us, enemy);
    move_list.sort_moves_best_first(transposition_table);
    for i in 0..move_list.len
    {
        let mut n_position = orig_position.clone();
        n_position.make_move(&move_list[i], us, enemy);
        if n_position.is_check_unkown_kings_index(us, enemy)
        {
            continue;
        }
        number_legal_moves += 1;

        let t_index = (move_list[i].zobrist_key%(transposition_table.len() as u64)) as usize;
        if  transposition_table[t_index].zobrist_key == move_list[i].zobrist_key && transposition_table[t_index].depth >= depth
        {
            current_score = transposition_table[t_index].score;
        }
        else
        {
            current_score = -nega_max(nodes, enemy, us, &n_position, depth - 1, -beta, -alpha, transposition_table);
            transposition_table[t_index].depth = depth;
            transposition_table[t_index].score = current_score;
            transposition_table[t_index].zobrist_key = move_list[i].zobrist_key;
            transposition_table[t_index].failed_high = false;

        }
        if alpha < current_score
        {
            alpha = current_score;
            if beta <= current_score
            {
                transposition_table[t_index].failed_high = true;
                break;
            }
        }
    }
    //check for MATE or STALEMATE
    if number_legal_moves == 0
    {
        if orig_position.is_check_unkown_kings_index(us, enemy)
        {
            alpha = -evaluation::score::SCORE_MATE - depth as evaluation::score::Score;
        }
        else
        {
            alpha = 0;
        }
    }
    alpha
}
pub fn start_nega_max(orig_position: position::Position, depth: Depth) -> position::mov::Move
{
    let mut transposition_table = transposition_table::get_empty_transposition_table(10_000);
    transposition_table.shrink_to_fit();
    println!("hi");
    let now = std::time::SystemTime::now();
    let mut nodes = 1;
    let enemy = position::player::switch_player(orig_position.whose_move);
    let us = orig_position.whose_move;
    let mut number_legal_moves = 0;
    let mut alpha = -evaluation::score::SCORE_INFINITY;
    let beta = evaluation::score::SCORE_INFINITY;
    let mut current_score;
    let mut best_board_index = 0;
    let mut move_list = orig_position.generate_move_list(us, enemy);
    move_list.sort_moves_best_first(&transposition_table);
    for i in 0..move_list.len
    {
        let mut n_position = orig_position.clone();
        n_position.make_move(&move_list[i], us, enemy);
        if n_position.is_check_unkown_kings_index(us, enemy)
        {
            continue;
        }
        number_legal_moves += 1;
        current_score = -nega_max(&mut nodes, enemy, us, &n_position, depth - 1, -beta, -alpha, &mut transposition_table);
        if alpha < current_score
        {
            alpha = current_score;
            best_board_index = i;
        }
    }
    //check for MATE or STALEMATE
    if number_legal_moves == 0
    {
        if orig_position.is_check_unkown_kings_index(us, enemy)
        {
            println!("NO LEGAL MOVES: CHECKMATE.");
            return position::mov::Move::empty_move();
        }
        else
        {
            println!("NO LEGAL MOVES: STALEMATE.");
            return position::mov::Move::empty_move();
        }
    }
    let time;
    match now.elapsed() {
        Ok(elapsed) =>
        {
            time = format!("{}.{}", elapsed.as_secs(), elapsed.subsec_nanos()).parse::<f32>().unwrap();
        }
        Err(e) =>
        {
            println!("Error: {:?}", e);
            panic!();
        }
    }
    println!("info depth {}", depth);
    println!("info time {}", time*1000.0);
    println!("info nodes {}", nodes);
    println!("info nps {}", nodes as f32 / time);
    if alpha >= evaluation::score::SCORE_MATE
    {
        println!("info score mate {}", (depth as evaluation::score::Score + 1 - alpha + evaluation::score::SCORE_MATE) / 2);
    }
    else if alpha <= -evaluation::score::SCORE_MATE
    {
        println!("info score mate {}", -(depth as evaluation::score::Score + 1 + alpha + evaluation::score::SCORE_MATE) / 2);
    }
    else
    {
        println!("info score cp {}", alpha as f32 / evaluation::score::VALUE_PAWN as f32);
    }
    println!("bestmove {}",move_list[best_board_index].get_move_notation());

    move_list[best_board_index].clone()
}
