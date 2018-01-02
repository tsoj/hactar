use evaluation::score;
use search::Depth;
use std::ops::{Index,IndexMut};

#[derive(Copy, Clone)]
pub struct TranspositionTableEntry
{
    pub zobrist_key: u64,
    pub score: score::Score,
    pub depth: Depth
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
    pub fn add(&mut self, zobrist_key: u64, current_score: score::Score, depth: Depth)
    {
        let t_index = (zobrist_key % (self.a.len() as u64)) as usize;
        self[t_index].depth = depth;
        self[t_index].score = current_score;
        self[t_index].zobrist_key = zobrist_key;
    }
    pub fn get_score(&self, zobrist_key: u64, min_depth: Depth) -> Option<score::Score>
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
    pub fn get_empty_transposition_table(size: usize) -> TranspositionTable
    {
        TranspositionTable{a: vec![TranspositionTableEntry{zobrist_key: 0, score: score::VALUE_NO_PIECE, depth: -1}; size]}
    }
}
