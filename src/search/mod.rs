#![allow(dead_code)]
use position;
use evaluation;
use std;
pub mod perft;

fn nega_max(
    nodes: &mut u64,
    us: position::player::Player,
    enemy: position::player::Player,
    orig_position: &position::Position,
    depth: u32, mut alpha: evaluation::score::Score,
    beta: evaluation::score::Score) -> evaluation::score::Score
{
    *nodes += 1;
    if depth==0
    {
        return evaluation::evaluate(&orig_position, us, enemy);
    }
    let mut current_score: evaluation::score::Score;
    let mut number_legal_moves = 0;
    let move_list = orig_position.generate_move_list(us, enemy);
    for i in 0..move_list.len
    {
        let mut n_position = orig_position.clone();
        n_position.make_move(&move_list[i], us, enemy);
        if !n_position.is_check_unkown_kings_index(us, enemy)
        {
            number_legal_moves += 1;
            current_score = -nega_max(nodes, enemy, us, &n_position, depth - 1, -beta, -alpha);

            if alpha < current_score
            {
                alpha = current_score;
                if beta <= current_score
                {
                    break;
                }
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
pub fn start_nega_max(orig_position: position::Position, depth: u32) -> position::Move
{
    let now = std::time::SystemTime::now();
    let mut nodes = 1;
    let mut orig_position = orig_position.clone();
    let enemy = position::player::switch_player(orig_position.whose_move);
    let us = orig_position.whose_move;
    let mut number_legal_moves = 0;
    let mut alpha = -evaluation::score::SCORE_INFINITY;
    let beta = evaluation::score::SCORE_INFINITY;
    let mut current_score;
    let mut best_board_index = 0;
    let move_list = orig_position.generate_move_list(us, enemy);
    for i in 0..move_list.len
    {
        let mut n_position = orig_position.clone();
        n_position.make_move(&move_list[i], us, enemy);
        if !n_position.is_check_unkown_kings_index(us, enemy)
        {
            number_legal_moves += 1;
            current_score = -nega_max(&mut nodes, enemy, us, &n_position, depth - 1, -beta, -alpha);

            if alpha < current_score
            {
                alpha = current_score;
                best_board_index = i;
            }
        }
    }
    //check for MATE or STALEMATE
    if number_legal_moves == 0
    {
        if orig_position.is_check_unkown_kings_index(us, enemy)
        {
            //MATE
        }
        else
        {
            //STALEMATE
        }
    }
    orig_position.make_move(&move_list[best_board_index], us, enemy);
    println!("{}", orig_position.get_chess_board_string());
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
    println!("depth: {}", depth);
    println!("time needed: {}", time);
    println!("nodes: {}", nodes);
    println!("nodes per seconds: {}", nodes as f32 / time);
    println!("best move:\n{}",move_list[best_board_index].get_data_string());

    move_list[best_board_index].clone()
}
