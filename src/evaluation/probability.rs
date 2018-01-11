use evaluation::score::{Score, SCORE_MATE, VALUE_QUEEN};

pub type Probability = f64;
pub fn score_to_probability(mut score: Score) -> Probability
{
    let multiplier = 3;
    if score > 2*VALUE_QUEEN
    {
        let temp = score - 2*VALUE_QUEEN;
        score = 2*VALUE_QUEEN*multiplier + temp;
    }
    else if score > 0
    {
        score = score*multiplier;
    }
    else if score < 2*-VALUE_QUEEN
    {
        let temp = score - 2*-VALUE_QUEEN;
        score = 2*-VALUE_QUEEN*multiplier + temp;
    }
    else if score < 0
    {
        score = score*multiplier;
    }
    let mut ret = (score + SCORE_MATE) as f64 / (2*SCORE_MATE) as f64;
    if ret > 1.0
    {
        ret = 0.99;
    }
    else if ret < 0.0
    {
        ret = 0.01;
    }
    ret
}
