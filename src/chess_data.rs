pub const HORIZONTAL_LINE_UNICODE: &'static str = "\u{2501}"; //http://www.fileformat.info/info/unicode/category/So/list.htm
pub const VERTICAL_LINE_UNICODE: &'static str = "\u{2503}";
pub const ONE_UNICODE: &'static str = "\u{23FA}";
pub const ZERO_UNICODE: &'static str = " ";
pub const BLACK_PAWN_UNICODE: &'static str = "\u{2659}";
pub const BLACK_KNIGHT_UNICODE: &'static str = "\u{2658}";
pub const BLACK_BISHOP_UNICODE: &'static str = "\u{2657}";
pub const BLACK_ROOK_UNICODE: &'static str = "\u{2656}";
pub const BLACK_QUEEN_UNICODE: &'static str = "\u{2655}";
pub const BLACK_KING_UNICODE: &'static str = "\u{2654}";
pub const WHITE_PAWN_UNICODE: &'static str = "\u{265F}";
pub const WHITE_KNIGHT_UNICODE: &'static str = "\u{265E}";
pub const WHITE_BISHOP_UNICODE: &'static str = "\u{265D}";
pub const WHITE_ROOK_UNICODE: &'static str = "\u{265C}";
pub const WHITE_QUEEN_UNICODE: &'static str = "\u{265B}";
pub const WHITE_KING_UNICODE: &'static str = "\u{265A}";


pub const NORTH: isize = 8;
pub const SOUTH: isize = -8;
pub const EAST: isize = 1;
pub const WEST: isize = -1;
pub const NORTH_EAST: isize = 9;
pub const NORTH_WEST: isize = 7;
pub const SOUTH_WEST: isize = -9;
pub const SOUTH_EAST: isize = -7;

pub const BIT_AT_INDEX: [u64; 64] = include!("./chess_data_in/bit_at_index.in");
pub const RANKS: [u64; 8] = include!("./chess_data_in/ranks.in");
pub const FILES: [u64; 8] = include!("./chess_data_in/files.in");
pub const RANKS_64: [u64; 64] = include!("./chess_data_in/ranks_64.in");
pub const FILES_64: [u64; 64] = include!("./chess_data_in/files_64.in");
pub const DIAGONALS_64: [u64; 64] = include!("./chess_data_in/diagonals_64.in");
pub const ANTI_DIAGONALS_64: [u64; 64] = include!("./chess_data_in/anti_diagonals_64.in");

pub const RANK_ATTACK_TABLE: [[u64; 64]; 64] = include!("./chess_data_in/rank_attack_table.in");
pub const FILE_ATTACK_TABLE: [[u64; 64]; 64] = include!("./chess_data_in/file_attack_table.in");
pub const DIAGONAL_ATTACK_TABLE: [[u64; 64]; 64] = include!("./chess_data_in/diagonal_attack_table.in");
pub const ANTI_DIAGONAL_ATTACK_TABLE: [[u64; 64]; 64] = include!("./chess_data_in/anti_diagonal_attack_table.in");
pub const KNIGHT_ATTACK_TABLE: [u64; 64] = include!("./chess_data_in/knight_attack_table.in");
pub const KING_ATTACK_TABLE: [u64; 64] = include!("./chess_data_in/king_attack_table.in");
