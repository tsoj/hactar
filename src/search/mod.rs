pub mod transposition_table;
pub mod perft;
pub mod node;

use position::mov::{Move};
use position::Position;
use position::piece::NO_PIECE;
use evaluation::score::{Score, SCORE_MATE, SCORE_INFINITY};//, VALUE_PAWN};
use search::transposition_table::TranspositionTable;
use search::node::{Node, NORMAL_NODE, ROOT_NODE};

use std::time::SystemTime;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub type Depth = usize;
pub const MAX_DEPTH: Depth = 64;
pub type PV = Vec<Move>;

const MAX_NUM_CHECKS_IN_QUIESCE: u8 = 2;

pub struct Searcher
{
    pub transposition_table: TranspositionTable,
    pub best_move: Move,
    pub nodes_count: u64,
    pub pv: PV,
    pub should_stop: Arc<AtomicBool>
}
impl Searcher
{
    fn nega_max(
        &mut self,
        node_type: Node,
        orig_position: &Position,
        depth: Depth,
        mut alpha: Score,
        beta: Score,
        pv: &mut PV
    ) -> Score
    {
        if self.should_stop.load(Ordering::Relaxed)
        {
            return 0;
        }

        match self.transposition_table.get_score(orig_position.zobrist_key, depth)
        {
            Some(x) => return x,
            None =>{}
        }

        self.nodes_count += 1;
        if depth==0
        {
            return self.quiesce(orig_position, alpha, beta, pv, 0);
        }

        let pv_move = match self.pv.pop()
        {
            Some(x) => x,
            None => Move::empty_move()
        };

        let mut current_score: Score;
        let in_check = orig_position.is_check_unkown_kings_index(orig_position.us, orig_position.enemy);
        if !in_check && node_type != ROOT_NODE && depth <= 3
        {
            let current_score = orig_position.evaluate();
            if current_score >= beta
            {
                return current_score;
            }
        }

        let mut move_list = orig_position.generate_move_list();
        move_list.sort_moves(&self.transposition_table, &pv_move);
        let mut number_legal_moves = 0;
        for i in 0..move_list.len
        {
            let mut new_position = orig_position.clone();
            new_position.make_move(&move_list[i]);
            if new_position.is_check_unkown_kings_index(orig_position.us, orig_position.enemy)
            {
                continue;
            }
            number_legal_moves += 1;

            let mut candidate_pv = Vec::new();
            if
                i >= 4 &&
                !in_check &&
                move_list[i].captured == NO_PIECE &&
                depth >= 2 &&
                -self.nega_max(NORMAL_NODE, &new_position, depth - 2, -beta, -alpha, &mut candidate_pv) <= alpha
            {
                continue;
            }
            current_score = -self.nega_max(NORMAL_NODE, &new_position, depth - 1, -beta, -alpha, &mut candidate_pv);
            if current_score > alpha
            {
                alpha = current_score;
                if node_type == ROOT_NODE
                {
                    self.best_move = move_list[i].clone();
                }
                *pv = candidate_pv;
                pv.push(move_list[i]);
                if current_score >= beta
                {
                    self.transposition_table.set_failed_high(move_list[i].zobrist_key);
                    break;
                }
            }
        }
        //check for MATE or STALEMATE
        if number_legal_moves == 0
        {
            if orig_position.is_check_unkown_kings_index(orig_position.us, orig_position.enemy)
            {
                alpha = -(SCORE_MATE + depth as Score);
            }
            else
            {
                alpha = 0;
            }
        }
        self.transposition_table.add(orig_position.zobrist_key, alpha, depth);
        alpha
    }
    fn quiesce(
        &mut self,
        orig_position: &Position,
        mut alpha: Score,
        beta: Score,
        pv: &mut PV,
        mut number_checks: u8
    ) -> Score
    {
        self.nodes_count += 1;
        let stand_pat = orig_position.evaluate();
        let in_check = orig_position.is_check_unkown_kings_index(orig_position.us, orig_position.enemy) && number_checks > MAX_NUM_CHECKS_IN_QUIESCE;
        if stand_pat > alpha && !in_check
        {
            alpha = stand_pat;
        }
        if stand_pat >= beta
        {
            return beta;
        }
        let mut move_list;
        if in_check
        {
            move_list = orig_position.generate_move_list();
            number_checks += 1;
        }
        else
        {
            move_list = orig_position.generate_capture_move_list();
        }
        let mut current_score;
        move_list.sort_moves_quiesce();
        let mut number_legal_moves = 0;
        for i in 0..move_list.len
        {
            let mut new_position = orig_position.clone();
            new_position.make_move(&move_list[i]);
            if new_position.is_check_unkown_kings_index(orig_position.us, orig_position.enemy)
            {
                continue;
            }
            number_legal_moves += 1;
            let mut candidate_pv = Vec::new();
            current_score = -self.quiesce(&new_position, -beta, -alpha, &mut candidate_pv, number_checks);

            if current_score > alpha
            {
                *pv = candidate_pv;
                pv.push(move_list[i]);
                alpha = current_score;
                if current_score >= beta
                {
                    break;
                }
            }
        }
        if number_legal_moves == 0
        {
            if in_check
            {
                alpha = -SCORE_MATE;
            }
            else
            {
                alpha = stand_pat;
            }
        }
        alpha
    }
    pub fn go(orig_position: Position, depth: Depth, should_stop: Arc<AtomicBool>, time_per_move_ms: i64) -> Move
    {
        let mut searcher = Searcher
        {
            transposition_table: TranspositionTable::empty_transposition_table(100_000_000),
            nodes_count: 0,
            pv: Vec::new(),
            best_move: Move::empty_move(),
            should_stop: should_stop
        };
        let mut best_move = Move::empty_move();
        for i in 1..(depth+1)
        {
            let now = SystemTime::now();
            searcher.nodes_count = 0;
            let mut pv = Vec::new();
            let score = searcher.nega_max(ROOT_NODE, &orig_position, i, -SCORE_INFINITY, SCORE_INFINITY, &mut pv);
            if searcher.should_stop.load(Ordering::Relaxed) == false
            {
                best_move = searcher.best_move;
                searcher.pv = pv;
                let time;
                match now.elapsed()
                {
                    Ok(elapsed) =>
                    {
                        time = format!("{}.{}", elapsed.as_secs(), elapsed.subsec_nanos()).parse::<f64>().unwrap();
                    }
                    Err(e) =>
                    {
                        println!("Error: {:?}", e);
                        panic!();
                    }
                }
                print!("info ");
                print!("depth {0: <2} ", i);
                print!("time {0: <8} ", (time*1000.0) as u64);
                print!("nodes {0: <11} ", searcher.nodes_count);
                print!("nps {0: <9} ", (searcher.nodes_count as f64 / time) as u64);
                if score >= SCORE_MATE
                {
                    print!("score mate {0: <8} ", (-score + SCORE_MATE + i as Score + 1)/2);
                }
                else if score <= -SCORE_MATE
                {
                    print!("score mate {0: <8} ", -(score + SCORE_MATE + i as Score + 1)/2);
                }
                else
                {
                    print!("score cp {0: <8} ", score);
                }
                print!("pv ");
                for i in 0..searcher.pv.len()
                {
                    print!("{} ", searcher.pv[searcher.pv.len()-1 - i].get_move_notation());
                }
                println!();
                if score >= SCORE_MATE || score <= -SCORE_MATE
                {
                    break;
                }
                if time_per_move_ms != -1
                {
                    let time_last_iteration = (time*1000.0)as i64;
                    let estimated_time_next_iteration = (time_last_iteration - 400)*10;
                    if estimated_time_next_iteration > time_per_move_ms && i >= 6
                    {
                        break;
                    }
                }
            }
        }
        println!("bestmove {}", best_move.get_move_notation());
        best_move
    }
}
