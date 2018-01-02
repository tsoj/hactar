
use position;
use evaluation;
use std;
pub mod transposition_table;
pub mod perft;
pub mod node;

pub type Depth = i32;

pub struct Searcher
{
    transposition_table: transposition_table::TranspositionTable,
    best_move: position::mov::Move,
    nodes_count: u64
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
    ) -> evaluation::score::Score
    {
        self.nodes_count += 1;
        if depth==0
        {
            return self.quiesce(orig_position, depth, alpha, beta);//evaluation::evaluate(&orig_position);
        }
        let mut current_score: evaluation::score::Score;
        let mut number_legal_moves = 0;
        let mut move_list = orig_position.generate_move_list();
        move_list.sort_moves_best_first(&self.transposition_table);
        for i in 0..move_list.len
        {
            let mut n_position = orig_position.clone();
            n_position.make_move(&move_list[i]);
            if n_position.is_check_unkown_kings_index(orig_position.us, orig_position.enemy)
            {
                continue;
            }
            number_legal_moves += 1;

            match self.transposition_table.get_score(move_list[i].zobrist_key, depth)
            {
                Some(x) => current_score = x,
                None =>
                {
                    current_score = -self.nega_max(node::NORMAL_NODE, &n_position, depth - 1, -beta, -alpha);
                    self.transposition_table.add(move_list[i].zobrist_key, current_score, depth);
                }
            }
            if alpha < current_score
            {
                alpha = current_score;
                if node_type == node::ROOT_NODE
                {
                    self.best_move = move_list[i].clone();
                }
                if beta <= current_score
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
    pub fn go(orig_position: &position::Position, depth: Depth) -> position::mov::Move
    {
        let mut searcher =
        Searcher
            {
                transposition_table: transposition_table::TranspositionTable::get_empty_transposition_table(100_000_000),
                best_move: position::mov::Move::empty_move(),
                nodes_count: 0
            };

        for i in 1..(depth+1)
        {
            let now = std::time::SystemTime::now();
            searcher.nodes_count = 0;
            let score = searcher.nega_max(node::ROOT_NODE, &orig_position, i, -evaluation::score::SCORE_INFINITY, evaluation::score::SCORE_INFINITY);
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
                println!("score mate {}", (depth as evaluation::score::Score + 1 - score + evaluation::score::SCORE_MATE) / 2);
            }
            else if score <= -evaluation::score::SCORE_MATE
            {
                println!("score mate {}", -(depth as evaluation::score::Score + 1 + score + evaluation::score::SCORE_MATE) / 2);
            }
            else
            {
                println!("score cp {}", score as f32 / evaluation::score::VALUE_PAWN as f32);
            }

        }
        println!("bestmove {}", searcher.best_move.get_move_notation());
        searcher.best_move
    }
    fn quiesce(
        &mut self,
        orig_position: &position::Position,
        depth: Depth,
        mut alpha: evaluation::score::Score,
        beta: evaluation::score::Score,
    ) -> evaluation::score::Score
    {
        self.nodes_count += 1;
        let stand_pat = evaluation::evaluate(&orig_position);
        if stand_pat >= beta
        {
            return beta;
        }
        if alpha < stand_pat
        {
            alpha = stand_pat;
        }
        let mut current_score: evaluation::score::Score;
        let mut move_list = orig_position.generate_capture_move_list();
        move_list.sort_moves_best_first(&self.transposition_table);
        for i in 0..move_list.len
        {
            let mut n_position = orig_position.clone();
            n_position.make_move(&move_list[i]);
            current_score = -self.quiesce(&n_position, depth - 1, -beta, -alpha);

            if current_score >= beta
            {
                return beta;
            }
            if current_score > alpha
            {
               alpha = current_score;
            }
        }
        return alpha;
    }
}
