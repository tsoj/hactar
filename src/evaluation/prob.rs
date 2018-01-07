use evaluation::score::{Score, SCORE_MATE};
use alpha_beta_search::MAX_DEPTH;

pub type WinProb = i32;
pub const WIN: Prob = SCORE_MATE - MAX_DEPTH;
pub const LOSS: Prob = 0;


pub fn convert_to_winning_probability(mut score: Score) -> Prob
{
    score = (score / 2) + WIN / 2;
    if score > 950
    {
        score = 950;
    }
    else if score < 50
    {
        score = 50;
    }
    score
}
