pub mod transposition_table;
pub mod perft;
pub mod node;

use position::mov::{Move};
use position::Position;
use evaluation::score::{Score, SCORE_MATE, SCORE_INFINITY, VALUE_PAWN};
use std::time::SystemTime;
use search::transposition_table::TranspositionTable;
use search::node::{Node, NORMAL_NODE, PV_NODE, ROOT_NODE};

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
        self.nodes_count += 1;
        match self.transposition_table.get_score(orig_position.zobrist_key, depth)
        {
            Some(x) => return x,
            None => {}
        }
        if depth==0
        {
            return self.quiesce(orig_position, alpha, beta, pv, 0);
        }
        let mut current_score: Score;
        let mut number_legal_moves = 0;

        let pv_move = match self.pv.pop()
        {
            Some(x) => x,
            None => Move::empty_move()
        };

        if !orig_position.is_check_unkown_kings_index(orig_position.us, orig_position.enemy)
        {
            let current_score = orig_position.evaluate();
            if current_score > alpha && depth <= 2
            {
                alpha = current_score;
            }
            if current_score >= beta && depth <= /*6 or */5/* or 4 or 3 or 2*/
            {
                return current_score;
            }
        }

        let mut move_list = orig_position.generate_move_list();
        move_list.sort_moves(&self.transposition_table, &pv_move);
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
            current_score = -self.nega_max(
                if i == 0{ PV_NODE }else{ NORMAL_NODE },
                &new_position,
                depth -1,
                -beta,
                -alpha,
                &mut candidate_pv
            );
            self.transposition_table.add(move_list[i].zobrist_key, -current_score, depth -1);
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
        if stand_pat > alpha && (!orig_position.is_check_unkown_kings_index(orig_position.us, orig_position.enemy) || number_checks > MAX_NUM_CHECKS_IN_QUIESCE)
        {
            alpha = stand_pat;
            if stand_pat >= beta
            {
                return beta;
            }
        }
        else
        {
            number_checks += 1;
        }
        let mut current_score: Score;
        let mut move_list = orig_position.generate_capture_move_list();
        move_list.sort_moves_quiesce();
        for i in 0..move_list.len
        {
            let mut new_position = orig_position.clone();
            new_position.make_move(&move_list[i]);
            if new_position.is_check_unkown_kings_index(orig_position.us, orig_position.enemy)
            {
                continue;
            }
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
        alpha
    }
    pub fn go(orig_position: &Position, depth: Depth) -> Move
    {
        let mut searcher = Searcher
        {
            transposition_table: TranspositionTable::empty_transposition_table(100_000_000),
            nodes_count: 0,
            pv: Vec::new(),
            best_move: Move::empty_move()
        };

        for i in 1..(depth+1)
        {
            let now = SystemTime::now();
            searcher.nodes_count = 0;
            let mut pv = Vec::new();
            let score = searcher.nega_max(ROOT_NODE, &orig_position, i, -SCORE_INFINITY, SCORE_INFINITY, &mut pv);
            searcher.pv = pv;
            let time;
            match now.elapsed()
            {
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
            print!("info ");
            print!("depth {} ", i);
            print!("time {} ", time*1000.0);
            print!("nodes {} ", searcher.nodes_count);
            print!("nps {} ", searcher.nodes_count as f32 / time);
            if score >= SCORE_MATE
            {
                print!("score mate {}", (-score + SCORE_MATE + i as Score + 1)/2);
            }
            else if score <= -SCORE_MATE
            {
                print!("score mate {}", -(score + SCORE_MATE + i as Score + 1)/2);
            }
            else
            {
                print!("score cp {}", score);
            }
            print!(" pv ");
            for i in 0..searcher.pv.len()
            {
                print!("{} ", searcher.pv[searcher.pv.len()-1 - i].get_move_notation());
            }
            println!();
        }
        println!("bestmove {}", searcher.best_move.get_move_notation());
        searcher.best_move
    }
}
