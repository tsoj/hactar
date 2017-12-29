use chess_data;

pub type Piecetype = usize;

pub const PAWN: Piecetype = 0;
pub const KNIGHT: Piecetype = 1;
pub const BISHOP: Piecetype = 2;
pub const ROOK: Piecetype = 3;
pub const QUEEN: Piecetype = 4;
pub const KING: Piecetype = 5;
pub const NO_PIECE: Piecetype = 6;

pub fn get_unicode(piece: Piecetype) -> &'static str
{
    match piece
    {
        PAWN =>
        {
            return chess_data::WHITE_PAWN_UNICODE;
        },
        KNIGHT =>
        {
            return chess_data::WHITE_KNIGHT_UNICODE;
        },
        BISHOP =>
        {
            return chess_data::WHITE_BISHOP_UNICODE;
        },
        ROOK =>
        {
            return chess_data::WHITE_ROOK_UNICODE;
        },
        QUEEN =>
        {
            return chess_data::WHITE_QUEEN_UNICODE;
        },
        KING =>
        {
            return chess_data::WHITE_KING_UNICODE;
        },
        NO_PIECE =>
        {
            return "-";
        },

        _x =>
        {
            return "?";
        }
    }
}
