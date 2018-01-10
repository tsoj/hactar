pub type Score = i64;
pub const SCORE_INFINITY: Score = 9_223_372_036_854_775_807;
pub const VALUE_PAWN: Score = 100;
pub const VALUE_KNIGHT: Score = 320;
pub const VALUE_BISHOP: Score = 330;
pub const VALUE_ROOK: Score = 520;
pub const VALUE_QUEEN: Score = 920;
pub const VALUE_KING: Score = (8*VALUE_PAWN + 2*VALUE_KNIGHT + 2*VALUE_BISHOP + 2*VALUE_ROOK + VALUE_QUEEN);
pub const VALUE_NO_PIECE: Score = 0;
pub const SCORE_MATE: Score = VALUE_KING;

pub const SCORE: [Score; 7] = [VALUE_PAWN, VALUE_KNIGHT, VALUE_BISHOP, VALUE_ROOK, VALUE_QUEEN, VALUE_KING, VALUE_NO_PIECE];
