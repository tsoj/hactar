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

const RANK_ATTACK_TABLE: [[u64; 64]; 64] = include!("./chess_data_in/rank_attack_table.in");
const FILE_ATTACK_TABLE: [[u64; 64]; 64] = include!("./chess_data_in/file_attack_table.in");
const DIAGONAL_ATTACK_TABLE: [[u64; 64]; 64] = include!("./chess_data_in/diagonal_attack_table.in");
const ANTI_DIAGONAL_ATTACK_TABLE: [[u64; 64]; 64] = include!("./chess_data_in/anti_diagonal_attack_table.in");
const KNIGHT_ATTACK_TABLE: [u64; 64] = include!("./chess_data_in/knight_attack_table.in");
const KING_ATTACK_TABLE: [u64; 64] = include!("./chess_data_in/king_attack_table.in");

const ZOBRIST_RANDOM_BITMASKS_PIECES: [[u64; 64]; 6] = include!("./chess_data_in/zobrist_random_bitmasks_pieces.in");
const ZOBRIST_RANDOM_BITMASKS_PLAYERS: [[u64; 64]; 2] = include!("./chess_data_in/zobrist_random_bitmasks_players.in");

fn get_hashkey_rank(index: usize, occupancy: u64) -> usize
{
    (((occupancy >> ((index / 8)*8)) >> 1) & 0b111111) as usize
}
fn get_hashkey_file(index: usize , occupancy: u64) -> usize
{
    ((((((occupancy >> (index % 8)) & FILES[0] ).wrapping_mul(MAIN_DIAGONAL)) >> 56) >> 1) & 0b111111) as usize
}
fn get_hashkey_diagonal(index: usize, occupancy: u64) -> usize
{
    (((((occupancy & DIAGONALS_64[index as usize]).wrapping_mul(FILES[0])) >> 56) >> 1) & 0b111111) as usize
}
fn get_hashkey_anti_diagonal(index: usize, occupancy: u64) -> usize
{
    (((((occupancy & ANTI_DIAGONALS_64[index]).wrapping_mul(FILES[0])) >> 56) >> 1) & 0b111111) as usize
}
fn get_attack_mask_knight(index: usize) -> u64
{
    KNIGHT_ATTACK_TABLE[index]
}
fn get_attack_mask_bishop(index: usize, occupancy: u64) -> u64
{
    ANTI_DIAGONAL_ATTACK_TABLE[index][get_hashkey_anti_diagonal(index, occupancy)] |
    DIAGONAL_ATTACK_TABLE[index][get_hashkey_diagonal(index, occupancy)]
}
fn get_attack_mask_rook(index: usize, occupancy: u64) -> u64
{
    RANK_ATTACK_TABLE[index][get_hashkey_rank(index, occupancy)] |
    FILE_ATTACK_TABLE[index][get_hashkey_file(index, occupancy)]
}
fn get_attack_mask_queen(index: usize, occupancy: u64) -> u64
{
    ANTI_DIAGONAL_ATTACK_TABLE[index][get_hashkey_anti_diagonal(index, occupancy)] |
    DIAGONAL_ATTACK_TABLE[index][get_hashkey_diagonal(index, occupancy)] |
    RANK_ATTACK_TABLE[index][get_hashkey_rank(index, occupancy)] |
    FILE_ATTACK_TABLE[index][get_hashkey_file(index, occupancy)]
}
fn get_attack_mask_king(index: usize) -> u64
{
    KING_ATTACK_TABLE[index]
}
