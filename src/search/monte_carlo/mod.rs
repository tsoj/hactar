#![allow(dead_code)]

use position::Position;
use evaluation::score::{SCORE_INFINITY};
use search::alpha_beta::quiesce;
use position::mov::Move;
use evaluation::probability::Probability;
use evaluation::probability::score_to_probability;
use search::Depth;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Clone)]
struct MctNode
{
    pub childs: Vec<MctNode>,
    pub win_prob: Probability,
    pub simulations: usize,
    pub mov: Move,
    pub position: Position,
    pub finished: bool
}
impl MctNode
{
    pub fn get_search_tree_string(&self, depth: usize) -> String
    {
        let mut ret = "".to_string();
        for _ in 1..depth
        {
            ret += " ";
        }
        if depth>0
        {
            ret += &self.mov.get_move_notation()[..];
            ret += " depth: ";
            ret += &depth.to_string()[..];
            ret += "\n";
        }
        for i in 0..self.childs.len()
        {
            ret += &self.childs[i].get_search_tree_string(depth+1)[..];
        }
        ret
    }
    pub fn get_number_nodes(&self) -> u64
    {
        if self.childs.len() == 0
        {
            return 1;
        }
        let mut ret = 0;
        for i in 0..self.childs.len()
        {
            ret += self.childs[i].get_number_nodes();
        }
        ret
    }
    pub fn get_max_min_depth(&self, depth: Depth) -> (Depth, Depth)
    {
        if self.childs.len() == 0
        {
            return (depth, depth);
        }
        let (mut max, mut min) = self.childs[0].get_max_min_depth(depth + 1);
        for i in 1..self.childs.len()
        {
            let (current_max, current_min) = self.childs[i].get_max_min_depth(depth + 1);
            if current_max > max
            {
                max = current_max;
            }
            if current_min < min
            {
                min = current_min;
            }
        }
        (max, min)
    }

    fn generate_child_nodes(&mut self)
    {
        let move_list = self.position.generate_move_list();
        for i in 0..move_list.len
        {
            let mut new_position = self.position.clone();
            new_position.make_move(&move_list[i]);
            if new_position.is_check_unkown_kings_index(self.position.us, self.position.enemy)
            {
                continue;
            }
            self.childs.push(
                MctNode
                {
                    childs: Vec::new(),
                    win_prob: 0.0,
                    simulations: 0,
                    mov: move_list[i].clone(),
                    position: new_position,
                    finished: false
                }
            );
        }
    }
    fn chose_most_promising_child(&mut self) -> Option<&mut MctNode>
    {
        let mut best = 0.0;
        let mut best_index = None;
        for i in 0..self.childs.len()
        {
            if self.childs[i].finished
            {
                continue;
            }
            let current_score = node_score(self.childs[i].win_prob, self.childs[i].simulations, self.simulations);
            if current_score > best
            {
                best = current_score;
                best_index = Some(i);
            }
        }
        match best_index
        {
            Some(x) => return Some(&mut self.childs[x]),
            None => return None
        };
    }
    fn expand(&mut self)
    {
        if self.childs.len() == 0
        {
            self.generate_child_nodes();
            for i in 0..self.childs.len()
            {
                self.childs[i].win_prob = self.childs[i].get_win_prob();
                self.childs[i].simulations = 1;
            }
            self.simulations += self.childs.len();
            if self.childs.len() == 0
            {
                if self.position.is_check_unkown_kings_index(self.position.us, self.position.enemy)
                {
                    self.win_prob = 0.0;
                }
                else
                {
                    self.win_prob = 0.5;
                }
                self.finished = true;
                return;
            }
        }
        else
        {
            match self.chose_most_promising_child()
            {
                Some(x) =>
                {
                    x.expand();
                },
                None => {}
            };
            self.simulations += 1;
        }
        
        self.win_prob = 0.0;
        for i in 0..self.childs.len()
        {
            if 1.0 - self.childs[i].win_prob > self.win_prob
            {
                self.win_prob = 1.0 - self.childs[i].win_prob;
            }
        }
    }
    fn get_win_prob(&self) -> Probability
    {
        score_to_probability(quiesce(&self.position, -SCORE_INFINITY, SCORE_INFINITY))
        //score_to_probability(self.position.evaluate())
    }
}

fn node_score(wins: Probability, simulations: usize, simulations_parent_node: usize) -> Probability
{
    let c = 1.414;
    wins + c*((simulations_parent_node as f64).ln() / simulations as f64).sqrt()
}
pub fn go_monte_carlo(position: Position, should_stop: Arc<AtomicBool>)
{
    let mut root_node = MctNode
    {
        childs: Vec::new(),
        win_prob: 0.0,
        simulations: 0,
        mov: Move::empty_move(),
        position: position,
        finished: false
    };
    while should_stop.load(Ordering::Relaxed) == false
    {
        root_node.expand();
    }
    let mut best = 0.0;
    let mut best_index = 0;
    for i in 0..root_node.childs.len()
    {
        let current = 1.0 - root_node.childs[i].win_prob;
        if current > best
        {
            best = current;
            best_index = i;
        }
    }
    println!("{}", root_node.childs[best_index].position.get_chess_board_string());
    println!("winning probability: {}", best);
    let (max_depth, min_depth) = root_node.get_max_min_depth(0);
    println!("max depth: {}", max_depth);
    println!("min depth: {}", min_depth);
    println!("nodes: {}", root_node.get_number_nodes());
    println!("simulations: {}", root_node.simulations);
    println!("bestmove {}", root_node.childs[best_index].mov.get_move_notation());
}
