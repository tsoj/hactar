
use position::mov::{Move};
use position::Position;
use evaluation::score::{Score, SCORE_MATE, SCORE_INFINITY};
use search::node_type::{NodeType, NORMAL_NODE, ROOT_NODE};
use search::Depth;

use std::time::SystemTime;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub type PV = Vec<Move>;

const MAX_NUM_CHECKS_EXTENSIONS_IN_QUIESCE: u8 = 2;
const MAX_NUM_CHECKS_EXTENSIONS: u8 = 4;

pub struct Searcher
{
    pub best_move: Move,
    pub nodes_count: u64,
    pub pv: PV,
    pub should_stop: Arc<AtomicBool>
}
impl Searcher
{
    pub fn nega_max(
        &mut self,
        node_type: NodeType,
        orig_position: &Position,
        mut depth: Depth,
        mut alpha: Score,
        beta: Score,
        pv: &mut PV,
        mut checks_extensions: u8
    ) -> Score
    {
        if self.should_stop.load(Ordering::Relaxed)
        {
            return 0;
        }
        self.nodes_count += 1;
        if depth==0
        {
            return self.quiesce(orig_position, alpha, beta, pv, 0);
        }

        let in_check = orig_position.is_check_unkown_kings_index(orig_position.us, orig_position.enemy);
        if in_check && checks_extensions < MAX_NUM_CHECKS_EXTENSIONS
        {
            depth += 1;
            checks_extensions += 1;
        }

        let mut current_score;
        let mut move_list = orig_position.generate_move_list();
        move_list.sort_moves();
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
            current_score = -self.nega_max(
                NORMAL_NODE,
                &new_position,
                depth -1,
                -beta,
                -alpha,
                &mut candidate_pv,
                checks_extensions
            );
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
        mut check_extensions: u8
    ) -> Score
    {
        self.nodes_count += 1;
        let in_check = orig_position.is_check_unkown_kings_index(orig_position.us, orig_position.enemy);
        let stand_pat = orig_position.evaluate();
        if stand_pat > alpha && (!in_check || check_extensions > MAX_NUM_CHECKS_EXTENSIONS_IN_QUIESCE)
        {
            alpha = stand_pat;
        }
        if stand_pat >= beta
        {
            return beta;
        }
        if in_check
        {
            check_extensions += 1;
        }
        let mut current_score: Score;
        let mut move_list = orig_position.generate_capture_move_list();
        move_list.sort_moves();
        for i in 0..move_list.len
        {
            let mut new_position = orig_position.clone();
            new_position.make_move(&move_list[i]);
            if new_position.is_check_unkown_kings_index(orig_position.us, orig_position.enemy)
            {
                continue;
            }
            let mut candidate_pv = Vec::new();
            current_score = -self.quiesce(&new_position, -beta, -alpha, &mut candidate_pv, check_extensions);

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
    pub fn go(orig_position: Position, depth: Depth, should_stop: Arc<AtomicBool>) -> Move
    {
        let mut searcher = Searcher
        {
            nodes_count: 0,
            pv: Vec::new(),
            best_move: Move::empty_move(),
            should_stop: should_stop
        };
        let now = SystemTime::now();
        let mut pv = Vec::new();
        let score = searcher.nega_max(ROOT_NODE, &orig_position, depth, -SCORE_INFINITY, SCORE_INFINITY, &mut pv, 0);
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
        print!("depth {} ", depth);
        print!("time {} ", time*1000.0);
        print!("nodes {} ", searcher.nodes_count);
        print!("nps {} ", (searcher.nodes_count as f32 / time) as u64);
        if score >= SCORE_MATE
        {
            print!("score mate {}", (-score + SCORE_MATE + depth as Score + 2)/2);
        }
        else if score <= -SCORE_MATE
        {
            print!("score mate {}", -(score + SCORE_MATE + depth as Score + 2)/2);
        }
        else
        {
            print!("score cp {} ", score);
            //print!("score prob {} ", convert_to_winning_probability(score));
        }
        print!("pv ");
        for i in 0..searcher.pv.len()
        {
            print!("{} ", searcher.pv[searcher.pv.len()-1 - i].get_move_notation());
        }
        println!();
        println!("bestmove {}", searcher.best_move.get_move_notation());
        searcher.best_move
    }
}


pub fn quiesce(
    orig_position: &Position,
    mut alpha: Score,
    beta: Score,
    mut check_extensions: u8
) -> Score
{
    let stand_pat = orig_position.evaluate();
    if stand_pat > alpha
    {
        alpha = stand_pat;
    }
    let in_check = orig_position.is_check_unkown_kings_index(orig_position.us, orig_position.enemy) && check_extensions < MAX_NUM_CHECKS_EXTENSIONS_IN_QUIESCE;
    if stand_pat >= beta && !in_check
    {
        return beta + 1;
    }
    let mut move_list;
    if in_check
    {
        move_list = orig_position.generate_move_list();
        check_extensions += 1;
    }
    else
    {
        move_list = orig_position.generate_capture_move_list();
    }
    move_list.sort_moves();
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
        let current_score = -quiesce(&new_position, -beta, -alpha, check_extensions);

        if current_score > alpha
        {
            alpha = current_score;
            if current_score >= beta
            {
                return beta + 1;
            }
        }
    }
    //check for MATE or STALEMATE
    if number_legal_moves == 0
    {
        if in_check
        {
            alpha = -SCORE_MATE;
        }
        alpha = stand_pat;
    }
    alpha
}
