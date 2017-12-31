use chess_data;

pub type Piece = usize;

pub const PAWN: Piece = 0;
pub const KNIGHT: Piece = 1;
pub const BISHOP: Piece = 2;
pub const ROOK: Piece = 3;
pub const QUEEN: Piece = 4;
pub const KING: Piece = 5;
pub const NO_PIECE: Piece = 6;

pub fn get_unicode(piece: Piece) -> &'static str
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
