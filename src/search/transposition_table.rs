use evaluation::score;
use search::Depth;

#[derive(Copy, Clone)]
pub struct TranspositionTableElement
{
    pub zobrist_key: u64,
    pub score: score::Score,
    pub depth: Depth,
    pub failed_high: bool,
}

pub fn get_empty_transposition_table(size: usize) -> Vec<TranspositionTableElement>
{
    vec![TranspositionTableElement{zobrist_key: 0, score: score::VALUE_NO_PIECE, depth: -1, failed_high: false}; size]
}
