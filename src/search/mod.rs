
use position;
use evaluation;
use std;
pub mod transposition_table;
pub mod perft;
pub mod node;

pub type Depth = usize;
pub const MAX_DEPTH: Depth = 64;
pub type PV = Vec<position::mov::Move>;

const MAX_NUM_CHECKS_IN_QUIESCE: u8 = 2;

pub struct Searcher
{
    transposition_table: transposition_table::TranspositionTable,
    best_move: position::mov::Move,
    nodes_count: u64,
    pv: PV
}
impl Searcher
{
    fn nega_max(
        &mut self,
        node_type: node::Node,
        orig_position: &position::Position,
        depth: Depth,
        mut alpha: evaluation::score::Score,
        beta: evaluation::score::Score,
        pv: &mut PV
    ) -> evaluation::score::Score
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
        if !orig_position.is_check_unkown_kings_index(orig_position.us, orig_position.enemy) && node_type!= node::PV_NODE && depth >= 2 && beta != evaluation::score::SCORE_INFINITY
        {
            let current_score = evaluation::evaluate(&orig_position);
            if current_score >= beta
            {
                return current_score;
            }
        }
        let mut current_score: evaluation::score::Score;
        let mut number_legal_moves = 0;

        let pv_move;
        match self.pv.pop()
        {
            Some(x) => pv_move = x,
            None => pv_move = position::mov::Move::empty_move()
        }
        let mut move_list = orig_position.generate_move_list();
        move_list.sort_moves(&self.transposition_table, &pv_move);
        for i in 0..move_list.len
        {
            let mut n_position = orig_position.clone();
            n_position.make_move(&move_list[i]);
            if n_position.is_check_unkown_kings_index(orig_position.us, orig_position.enemy)
            {
                continue;
            }
            number_legal_moves += 1;
            let mut candidate_pv = Vec::new();
            current_score = -self.nega_max(
                if i == 0{ node::PV_NODE }else{ node::NORMAL_NODE },
                &n_position,
                depth -1,
                -beta,
                -alpha,
                &mut candidate_pv
            );
            self.transposition_table.add(move_list[i].zobrist_key, -current_score, depth -1);
            if current_score > alpha
            {
                alpha = current_score;
                if node_type == node::ROOT_NODE
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
                alpha = -evaluation::score::SCORE_MATE - depth as evaluation::score::Score;
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
        orig_position: &position::Position,
        mut alpha: evaluation::score::Score,
        beta: evaluation::score::Score,
        pv: &mut PV,
        mut number_checks: u8
    ) -> evaluation::score::Score
    {
        self.nodes_count += 1;
        let stand_pat = evaluation::evaluate(&orig_position);
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
        let mut current_score: evaluation::score::Score;
        let mut move_list = orig_position.generate_capture_move_list();
        move_list.sort_moves_quiesce();
        for i in 0..move_list.len
        {
            let mut n_position = orig_position.clone();
            n_position.make_move(&move_list[i]);
            if n_position.is_check_unkown_kings_index(orig_position.us, orig_position.enemy)
            {
                continue;
            }
            let mut candidate_pv = Vec::new();
            current_score = -self.quiesce(&n_position, -beta, -alpha, &mut candidate_pv, number_checks);

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
    pub fn go(orig_position: &position::Position, depth: Depth) -> position::mov::Move
    {
        let mut searcher =
        Searcher
            {
                transposition_table: transposition_table::TranspositionTable::get_empty_transposition_table(100_000_000),
                best_move: position::mov::Move::empty_move(),
                nodes_count: 0,
                pv: Vec::new()
            };

        for i in 1..(depth+1)
        {
            let now = std::time::SystemTime::now();
            searcher.nodes_count = 0;
            let mut pv = Vec::new();
            let score = searcher.nega_max(node::ROOT_NODE, &orig_position, i, -evaluation::score::SCORE_INFINITY, evaluation::score::SCORE_INFINITY, &mut pv);
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
            if score >= evaluation::score::SCORE_MATE
            {
                print!("score mate {}", (depth as evaluation::score::Score + 1 - score + evaluation::score::SCORE_MATE) / 2);
            }
            else if score <= -evaluation::score::SCORE_MATE
            {
                print!("score mate {}", -(depth as evaluation::score::Score + 1 + score + evaluation::score::SCORE_MATE) / 2);
            }
            else
            {
                print!("score cp {}", score as f32 / evaluation::score::VALUE_PAWN as f32);
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
