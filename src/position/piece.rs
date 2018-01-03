use chess_data::*;

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
            return WHITE_PAWN_UNICODE;
        },
        KNIGHT =>
        {
            return WHITE_KNIGHT_UNICODE;
        },
        BISHOP =>
        {
            return WHITE_BISHOP_UNICODE;
        },
        ROOK =>
        {
            return WHITE_ROOK_UNICODE;
        },
        QUEEN =>
        {
            return WHITE_QUEEN_UNICODE;
        },
        KING =>
        {
            return WHITE_KING_UNICODE;
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
pub fn get_notation(piece: Piece) -> &'static str
{
    match piece
    {
        PAWN =>
        {
            return "p";
        },
        KNIGHT =>
        {
            return "n";
        },
        BISHOP =>
        {
            return "b";
        },
        ROOK =>
        {
            return "r";
        },
        QUEEN =>
        {
            return "q";
        },
        KING =>
        {
            return "k";
        },
        NO_PIECE =>
        {
            return "";
        },

        _x =>
        {
            return "?";
        }
    }
}
pub fn get_piece(s: &str) -> Piece
{
    match s
    {
        "p" | "P" =>
        {
            return PAWN;
        },
        "n" | "N" =>
        {
            return KNIGHT;
        },
        "b" | "B" =>
        {
            return BISHOP;
        },
        "r" | "R" =>
        {
            return ROOK;
        },
        "q" | "Q" =>
        {
            return QUEEN;
        },
        "k" | "K" =>
        {
            return KING;
        },

        _x =>
        {
            return NO_PIECE;
        }
    }
}
