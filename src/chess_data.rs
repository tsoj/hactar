#![allow(dead_code)]

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
const MAIN_DIAGONAL: u64 = 0b1000000001000000001000000001000000001000000001000000001000000001; //A1 to H8
const ANTI_DIAGONAL: u64 = 0b0000000100000010000001000000100000010000001000000100000010000000; //H1 to A8

pub const CASTLING_QUEENSIDE_ROOK_FROM_INDEX: [usize; 2] = [0, 56];
pub const CASTLING_QUEENSIDE_ROOK_FROM: [u64; 2] = [BIT_AT_INDEX[CASTLING_QUEENSIDE_ROOK_FROM_INDEX[0]], BIT_AT_INDEX[CASTLING_QUEENSIDE_ROOK_FROM_INDEX[1]]];
pub const CASTLING_KINGSIDE_ROOK_FROM_INDEX: [usize; 2] = [7, 63];
pub const CASTLING_KINGSIDE_ROOK_FROM: [u64; 2] = [BIT_AT_INDEX[CASTLING_KINGSIDE_ROOK_FROM_INDEX[0]], BIT_AT_INDEX[CASTLING_KINGSIDE_ROOK_FROM_INDEX[1]]];
pub const CASTLING_QUEENSIDE_ROOK_TO_INDEX: [usize; 2] = [3, 59];
pub const CASTLING_QUEENSIDE_ROOK_TO: [u64; 2] = [BIT_AT_INDEX[CASTLING_QUEENSIDE_ROOK_TO_INDEX[0]], BIT_AT_INDEX[CASTLING_QUEENSIDE_ROOK_TO_INDEX[1]]];
pub const CASTLING_KINGSIDE_ROOK_TO_INDEX: [usize; 2] = [5, 61];
pub const CASTLING_KINGSIDE_ROOK_TO: [u64; 2] = [BIT_AT_INDEX[CASTLING_KINGSIDE_ROOK_TO_INDEX[0]], BIT_AT_INDEX[CASTLING_KINGSIDE_ROOK_TO_INDEX[1]]];
pub const CASTLING_KING_FROM_INDEX: [usize; 2] = [4, 60];
pub const CASTLING_KING_FROM: [u64; 2] = [BIT_AT_INDEX[CASTLING_KING_FROM_INDEX[0]], BIT_AT_INDEX[CASTLING_KING_FROM_INDEX[1]]];
pub const CASTLING_QUEENSIDE_KING_TO_INDEX: [usize; 2] = [2, 58];
pub const CASTLING_QUEENSIDE_KING_TO: [u64; 2] = [BIT_AT_INDEX[CASTLING_QUEENSIDE_KING_TO_INDEX[0]], BIT_AT_INDEX[CASTLING_QUEENSIDE_KING_TO_INDEX[1]]];
pub const CASTLING_KINGSIDE_KING_TO_INDEX: [usize; 2] = [6, 62];
pub const CASTLING_KINGSIDE_KING_TO: [u64; 2] = [BIT_AT_INDEX[CASTLING_KINGSIDE_KING_TO_INDEX[0]], BIT_AT_INDEX[CASTLING_KINGSIDE_KING_TO_INDEX[1]]];
pub const CASTLING_QUEENSIDE_CHECK_RELEVANT_FIELDS: [[usize; 2]; 2] = [[3,4], [59,60]];
pub const CASTLING_KINGSIDE_CHECK_RELEVANT_FIELDS: [[usize; 2]; 2] = [[4,5], [60,61]];
pub const CASTLING_QUEENSIDE_BLOCK_RELEVANT_AREA: [u64; 2] =
    [ BIT_AT_INDEX[1] | BIT_AT_INDEX[2] | BIT_AT_INDEX[3], BIT_AT_INDEX[57] | BIT_AT_INDEX[58] | BIT_AT_INDEX[59] ];
pub const CASTLING_KINGSIDE_BLOCK_RELEVANT_AREA: [u64; 2] = [ BIT_AT_INDEX[5] | BIT_AT_INDEX[6], BIT_AT_INDEX[61] | BIT_AT_INDEX[62] ];

const RANK_ATTACK_TABLE: [[u64; 64]; 64] = include!("./chess_data_in/rank_attack_table.in");
const FILE_ATTACK_TABLE: [[u64; 64]; 64] = include!("./chess_data_in/file_attack_table.in");
const DIAGONAL_ATTACK_TABLE: [[u64; 64]; 64] = include!("./chess_data_in/diagonal_attack_table.in");
const ANTI_DIAGONAL_ATTACK_TABLE: [[u64; 64]; 64] = include!("./chess_data_in/anti_diagonal_attack_table.in");
const KNIGHT_ATTACK_TABLE: [u64; 64] = include!("./chess_data_in/knight_attack_table.in");
const KING_ATTACK_TABLE: [u64; 64] = include!("./chess_data_in/king_attack_table.in");
pub const PAWN_QUIET_ATTACK_TABLE: [[u64; 64]; 2] = include!("./chess_data_in/pawn_quiet_attack_table.in");
pub const PAWN_CAPTURE_ATTACK_TABLE: [[u64; 64]; 2] = include!("./chess_data_in/pawn_capture_attack_table.in");
pub const HOME_RANK: [u64; 2] = [RANKS[0], RANKS[7]];
pub const PAWN_HOME_RANK: [u64; 2] = [RANKS[1], RANKS[6]];

pub const ZOBRIST_RANDOM_BITMASKS_PIECES: [[u64; 64]; 6] = include!("./chess_data_in/zobrist_random_bitmasks_pieces.in");
pub const ZOBRIST_RANDOM_BITMASKS_PLAYERS: [[u64; 64]; 2] = include!("./chess_data_in/zobrist_random_bitmasks_players.in");

#[inline(always)]
fn get_hashkey_rank(index: usize, occupancy: u64) -> usize
{
    (((occupancy >> ((index / 8)*8)) >> 1) & 0b111111) as usize
}
#[inline(always)]
fn get_hashkey_file(index: usize , occupancy: u64) -> usize
{
    ((((((occupancy >> (index % 8)) & FILES[0] ).wrapping_mul(MAIN_DIAGONAL)) >> 56) >> 1) & 0b111111) as usize
}
#[inline(always)]
fn get_hashkey_diagonal(index: usize, occupancy: u64) -> usize
{
    (((((occupancy & DIAGONALS_64[index as usize]).wrapping_mul(FILES[0])) >> 56) >> 1) & 0b111111) as usize
}
#[inline(always)]
fn get_hashkey_anti_diagonal(index: usize, occupancy: u64) -> usize
{
    (((((occupancy & ANTI_DIAGONALS_64[index]).wrapping_mul(FILES[0])) >> 56) >> 1) & 0b111111) as usize
}
#[inline(always)]
pub fn get_attack_mask_knight(index: usize, _occupancy: u64) -> u64
{
    KNIGHT_ATTACK_TABLE[index]
}
#[inline(always)]
pub fn get_attack_mask_bishop(index: usize, occupancy: u64) -> u64
{
    ANTI_DIAGONAL_ATTACK_TABLE[index][get_hashkey_anti_diagonal(index, occupancy)] |
    DIAGONAL_ATTACK_TABLE[index][get_hashkey_diagonal(index, occupancy)]
}
#[inline(always)]
pub fn get_attack_mask_rook(index: usize, occupancy: u64) -> u64
{
    RANK_ATTACK_TABLE[index][get_hashkey_rank(index, occupancy)] |
    FILE_ATTACK_TABLE[index][get_hashkey_file(index, occupancy)]
}
#[inline(always)]
pub fn get_attack_mask_queen(index: usize, occupancy: u64) -> u64
{
    ANTI_DIAGONAL_ATTACK_TABLE[index][get_hashkey_anti_diagonal(index, occupancy)] |
    DIAGONAL_ATTACK_TABLE[index][get_hashkey_diagonal(index, occupancy)] |
    RANK_ATTACK_TABLE[index][get_hashkey_rank(index, occupancy)] |
    FILE_ATTACK_TABLE[index][get_hashkey_file(index, occupancy)]
}
#[inline(always)]
pub fn get_attack_mask_king(index: usize, _occupancy: u64) -> u64
{
    KING_ATTACK_TABLE[index]
}
#[inline(always)]
pub fn find_and_clear_trailing_one(mask: &mut u64) -> usize
{
    let ret = mask.trailing_zeros() as usize;
    *mask &= (*mask) -1;
    ret
}

pub fn get_field_notation(i: usize) -> &'static str
{
    match i
    {
         0 => return "a1",
         1 => return "b1",
         2 => return "c1",
         3 => return "d1",
         4 => return "e1",
         5 => return "f1",
         6 => return "g1",
         7 => return "h1",
         8 => return "a2",
         9 => return "b2",
         10 => return "c2",
         11 => return "d2",
         12 => return "e2",
         13 => return "f2",
         14 => return "g2",
         15 => return "h2",
         16 => return "a3",
         17 => return "b3",
         18 => return "c3",
         19 => return "d3",
         20 => return "e3",
         21 => return "f3",
         22 => return "g3",
         23 => return "h3",
         24 => return "a4",
         25 => return "b4",
         26 => return "c4",
         27 => return "d4",
         28 => return "e4",
         29 => return "f4",
         30 => return "g4",
         31 => return "h4",
         32 => return "a5",
         33 => return "b5",
         34 => return "c5",
         35 => return "d5",
         36 => return "e5",
         37 => return "f5",
         38 => return "g5",
         39 => return "h5",
         40 => return "a6",
         41 => return "b6",
         42 => return "c6",
         43 => return "d6",
         44 => return "e6",
         45 => return "f6",
         46 => return "g6",
         47 => return "h6",
         48 => return "a7",
         49 => return "b7",
         50 => return "c7",
         51 => return "d7",
         52 => return "e7",
         53 => return "f7",
         54 => return "g7",
         55 => return "h7",
         56 => return "a8",
         57 => return "b8",
         58 => return "c8",
         59 => return "d8",
         60 => return "e8",
         61 => return "f8",
         62 => return "g8",
         63 => return "h8",
        _x =>
        {
            println!("Index not between 0 and 63.");
            return "?";
        },
    }
}


pub fn get_field_index(s: &str) -> usize
{
    match s
    {
        "a1" | "A1"=> return 0,
        "b1" | "B1"=> return 1,
        "c1" | "C1"=> return 2,
        "d1" | "D1"=> return 3,
        "e1" | "E1"=> return 4,
        "f1" | "F1"=> return 5,
        "g1" | "G1"=> return 6,
        "h1" | "H1"=> return 7,
        "a2" | "A2"=> return 8,
        "b2" | "B2"=> return 9,
        "c2" | "C2"=> return 10,
        "d2" | "D2"=> return 11,
        "e2" | "E2"=> return 12,
        "f2" | "F2"=> return 13,
        "g2" | "G2"=> return 14,
        "h2" | "H2"=> return 15,
        "a3" | "A3"=> return 16,
        "b3" | "B3"=> return 17,
        "c3" | "C3"=> return 18,
        "d3" | "D3"=> return 19,
        "e3" | "E3"=> return 20,
        "f3" | "F3"=> return 21,
        "g3" | "G3"=> return 22,
        "h3" | "H3"=> return 23,
        "a4" | "A4"=> return 24,
        "b4" | "B4"=> return 25,
        "c4" | "C4"=> return 26,
        "d4" | "D4"=> return 27,
        "e4" | "E4"=> return 28,
        "f4" | "F4"=> return 29,
        "g4" | "G4"=> return 30,
        "h4" | "H4"=> return 31,
        "a5" | "A5"=> return 32,
        "b5" | "B5"=> return 33,
        "c5" | "C5"=> return 34,
        "d5" | "D5"=> return 35,
        "e5" | "E5"=> return 36,
        "f5" | "F5"=> return 37,
        "g5" | "G5"=> return 38,
        "h5" | "H5"=> return 39,
        "a6" | "A6"=> return 40,
        "b6" | "B6"=> return 41,
        "c6" | "C6"=> return 42,
        "d6" | "D6"=> return 43,
        "e6" | "E6"=> return 44,
        "f6" | "F6"=> return 45,
        "g6" | "G6"=> return 46,
        "h6" | "H6"=> return 47,
        "a7" | "A7"=> return 48,
        "b7" | "B7"=> return 49,
        "c7" | "C7"=> return 50,
        "d7" | "D7"=> return 51,
        "e7" | "E7"=> return 52,
        "f7" | "F7"=> return 53,
        "g7" | "G7"=> return 54,
        "h7" | "H7"=> return 55,
        "a8" | "A8"=> return 56,
        "b8" | "B8"=> return 57,
        "c8" | "C8"=> return 58,
        "d8" | "D8"=> return 59,
        "e8" | "E8"=> return 60,
        "f8" | "F8"=> return 61,
        "g8" | "G8"=> return 62,
        "h8" | "H8"=> return 63,
        _x =>
        {
            println!("Notation not properly formatted.");
            return 64;
        },
    }
}
