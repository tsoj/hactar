use evaluation::score::{Score, VALUE_NO_PIECE};
use search::{Depth, MAX_DEPTH};

use std::ops::{Index,IndexMut};


#[derive(Copy, Clone)]
pub struct TranspositionTableEntry
{
    zobrist_key: u64,
    score: Score,
    depth: Depth,
    failed_high: bool
}

pub struct TranspositionTable
{
    a: Vec<TranspositionTableEntry>
}
impl Index<usize> for TranspositionTable
{
    type Output = TranspositionTableEntry;
    #[inline(always)]
    fn index<'a>(&'a self, index: usize) -> &'a  TranspositionTableEntry {
        &self.a[index]
    }
}
impl IndexMut<usize> for TranspositionTable
{
    #[inline(always)]
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut TranspositionTableEntry {
        &mut self.a[index]
    }
}
impl TranspositionTable
{
    #[inline(always)]
    pub fn add(&mut self, zobrist_key: u64, current_score: Score, depth: Depth)
    {
        let t_index = (zobrist_key % (self.a.len() as u64)) as usize;
        self[t_index].depth = depth;
        self[t_index].score = current_score;
        self[t_index].zobrist_key = zobrist_key;
        self[t_index].failed_high = false;
    }
    #[inline(always)]
    pub fn get_score(&self, zobrist_key: u64, min_depth: Depth) -> Option<Score>
    {
        let t_index = (zobrist_key % (self.a.len() as u64)) as usize;
        if self[t_index].zobrist_key == zobrist_key && self[t_index].depth >= min_depth
        {
            Some(self[t_index].score)
        }
        else
        {
            None
        }
    }
    #[inline(always)]
    pub fn failed_high(&self, zobrist_key: u64) -> bool
    {
        let t_index = (zobrist_key % (self.a.len() as u64)) as usize;
        self[t_index].zobrist_key ==zobrist_key && self[t_index].failed_high
    }
    #[inline(always)]
    pub fn set_failed_high(&mut self, zobrist_key: u64)
    {
        let t_index = (zobrist_key % (self.a.len() as u64)) as usize;
        self[t_index].failed_high = true;
    }
    pub fn empty_transposition_table(size: usize) -> TranspositionTable
    {
        TranspositionTable{a: vec![TranspositionTableEntry{zobrist_key: 0, score: VALUE_NO_PIECE, depth: MAX_DEPTH + 1, failed_high: false}; size]}
    }
}
