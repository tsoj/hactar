#![allow(dead_code)]
use position;
use evaluation;
use score;
use std;

static mut NODES: u32 = 0;

fn perft(depth: u32, position: &mut position::Position, us: position::player::Player, enemy: position::player::Player) -> u32
{
    if depth == 0
    {
        return 1;
    }

    let mut nodes = 0;
    let move_list = position.generate_move_list(us, enemy);
    for i in 0..move_list.len
    {
        let mut position_n = position.clone();
        let backup_en_passant_castling = position.make_move(&move_list[i], us, enemy);
        if !position.is_check_unkown_kings_index(us, enemy)
        {
            nodes += perft(depth - 1, position, enemy, us);
        }
        position.undo_move(&move_list[i], backup_en_passant_castling, us, enemy);

        let mut t = false;

        for i in 0..6
        {
            if position_n.pieces[i] != position.pieces[i]
            {
                t = true;
                println!("1 {}", i);
            }
        }
        for i in 0..2
        {
            if position_n.players[i] != position.players[i]
            {
                t = true;
                println!("2 {}", i);
            }
        }
        if position_n.en_passant_castling != position.en_passant_castling
        {
            t = true;
            println!("en_p_c");
        }

        if t
        {
            println!("{}", position_n.get_chess_board_string());
            println!("{}", position_n.get_data_string());
            println!("{}", position.get_chess_board_string());
            position_n.make_move(&move_list[i], us, enemy);
            println!("{}", position_n.get_chess_board_string());
            println!("{}", move_list[i].get_data_string());
            panic!();
        }
    }
    nodes
}
pub fn start_perft(mut position: position::Position, depth: u32) -> u32
{
    let enemy = position::player::switch_player(position.whose_move);
    let us = position.whose_move;
    perft(depth, &mut position, us, enemy)
}

fn nega_max(
    us: position::player::Player,
    enemy: position::player::Player,
    position: &mut position::Position,
    depth: u32, mut alpha: score::Score,
    beta: score::Score) -> score::Score
{
  if depth==0
  {
    return evaluation::evaluate(position, us, enemy);
  }
  unsafe{NODES += 1;}
  let mut current_score: score::Score;
  let mut number_legal_moves = 0;
  let move_list = position.generate_move_list(us, enemy);
  for i in 0..move_list.len
  {
      let mut n_position = position.clone();
      n_position.make_move(&move_list[i], us, enemy);
      //let backup_en_passant_castling = position.make_move(&move_list[i], us, enemy);
      if !n_position.is_check_unkown_kings_index(us, enemy)
      {
          number_legal_moves += 1;
          current_score = -nega_max(enemy, us, &mut n_position, depth - 1, -beta, -alpha);

          if alpha < current_score
          {
              alpha = current_score;
              if beta <= current_score
              {
                  //position.undo_move(&move_list[i], backup_en_passant_castling, us, enemy);
                  break;
              }
          }
      }
      //position.undo_move(&move_list[i], backup_en_passant_castling, us, enemy);


  }
  //check for MATE or STALEMATE
  if number_legal_moves == 0
  {
    if position.is_check_unkown_kings_index(us, enemy)
    {
      alpha = -score::SCORE_MATE - depth as score::Score;
    }
    else
    {
      alpha = 0;
    }
  }
  alpha
}

pub fn start_nega_max_search(mut position: position::Position, depth: u32) -> position::Move
{
    let now = std::time::SystemTime::now();
    unsafe{NODES = 0;}
    let mut orig_position = position.clone();
    let enemy = position::player::switch_player(position.whose_move);
    let us = position.whose_move;
    let mut number_legal_moves = 0;
    let mut alpha = -score::SCORE_INFINITY;
    let beta = score::SCORE_INFINITY;
    let mut current_score;
    let mut best_board_index = 0;
    let move_list = position.generate_move_list(us, enemy);
    for i in 0..move_list.len
    {
        let backup_en_passant_castling = position.make_move(&move_list[i], us, enemy);
        if !position.is_check_unkown_kings_index(us, enemy)
        {
            number_legal_moves += 1;
            current_score = -nega_max(enemy, us, &mut position, depth - 1, -beta, -alpha);

            if alpha < current_score
            {
                alpha = current_score;
                best_board_index = i;
            }
        }
        position.undo_move(&move_list[i], backup_en_passant_castling, us, enemy);
    }
    //check for MATE or STALEMATE
    if number_legal_moves == 0
    {
      if position.is_check_unkown_kings_index(us, enemy)
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
    println!("time needed: {}", time);
    unsafe{println!("nodes: {}", NODES);
    println!("nodes per seconds: {}", NODES as f32 / time);}

    move_list[best_board_index].clone()


}
