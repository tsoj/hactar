use evaluation::score::{Score, VALUE_NO_PIECE};
use search::{Depth, MAX_DEPTH};
use position::mov::Move;

use std::ops::{Index,IndexMut};


#[derive(Copy, Clone)]
pub struct TranspositionTableEntry
{
    zobrist_key: u64,
    score: Score,
    depth: Depth,
    failed_high: bool,
    move_from: usize,
    move_to: usize
}

pub struct TranspositionTable
{
    a: Vec<TranspositionTableEntry>
}
impl Index<usize> for TranspositionTable
{
    type Output = TranspositionTableEntry;
    fn index<'a>(&'a self, index: usize) -> &'a  TranspositionTableEntry {
        &self.a[index]
    }
}
impl IndexMut<usize> for TranspositionTable
{
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut TranspositionTableEntry {
        &mut self.a[index]
    }
}
impl TranspositionTable
{
    pub fn add(&mut self, zobrist_key: u64, current_score: Score, depth: Depth, m: &Move)
    {
        let t_index = (zobrist_key % (self.a.len() as u64)) as usize;
        self[t_index].depth = depth;
        self[t_index].score = current_score;
        self[t_index].zobrist_key = zobrist_key;
        self[t_index].failed_high = false;
        self[t_index].move_to = m.to;
        self[t_index].move_from = m.from;
    }
    pub fn get_score(&self, zobrist_key: u64, min_depth: Depth, m: &Move) -> Option<Score>
    {
        let t_index = (zobrist_key % (self.a.len() as u64)) as usize;
        if self[t_index].zobrist_key == zobrist_key && self[t_index].depth >= min_depth && self[t_index].move_from == m.from && self[t_index].move_to == m.to
        {
            Some(self[t_index].score)
        }
        else
        {
            None
        }
    }
    pub fn failed_high(&self, zobrist_key: u64) -> bool
    {
        let t_index = (zobrist_key % (self.a.len() as u64)) as usize;
        self[t_index].zobrist_key ==zobrist_key && self[t_index].failed_high
    }
    pub fn set_failed_high(&mut self, zobrist_key: u64)
    {
        let t_index = (zobrist_key % (self.a.len() as u64)) as usize;
        self[t_index].failed_high = true;
    }
    pub fn empty_transposition_table(size: usize) -> TranspositionTable
    {
        TranspositionTable{a: vec![TranspositionTableEntry{zobrist_key: 0, score: VALUE_NO_PIECE, depth: MAX_DEPTH + 1, failed_high: false, move_from: 0, move_to: 0}; size]}
    }
}
