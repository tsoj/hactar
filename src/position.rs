#![allow(dead_code)]

use chess_data;

pub enum Piecetype
{
    Pawn = 0,
    Knigth = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
    NoPiece = 6
}
pub enum Player
{
    White = 0,
    Black = 1
}

pub struct Move
{
    pub from: usize,
    pub to: usize,
    pub moved: Piecetype,
    pub captured: Piecetype,
    pub promoted: Piecetype,
    pub en_passant_castling: u64,
    pub zobrist_key: u64,
}

pub struct Position
{
    pub pieces: [u64; 6], //[Pawns, Knights, Bishops, Rooks, Queens, Kings]
    pub player: [u64; 2], //[White pieces, Black pieces]
    pub whose_move: Player,
    pub last_move: Move
}

fn format_for_chess_board(field_content: &Vec<String>)->String
{
    let mut s = "".to_string();
    s.push_str("\n");
    for _ in 0..33
    {
        s.push_str(chess_data::HORIZONTAL_LINE_UNICODE);
    }
    s.push_str("\n");
    for h in 0..8
    {
        let i = 7 - h;
        for j in 0..8
        {
            s.push_str(chess_data::VERTICAL_LINE_UNICODE);
            s.push_str(" ");
            s.push_str(&field_content[8*i + j]);
            s.push_str(" ");
        }
        s.push_str(chess_data::VERTICAL_LINE_UNICODE);
        s.push_str(" ");
        s.push_str(&((i+1) as u32).to_string());
        s.push_str("\n");
        for _ in 0..33
        {
            s.push_str(chess_data::HORIZONTAL_LINE_UNICODE);
        }
        s.push_str("\n");
    }
    s.push_str("  A   B   C   D   E   F   G   H");
    s
}


pub fn get_bitboard_string(bitboard: u64) -> String
{
  let mut temp: Vec<String> = vec![String::new(); 64];
  for  i in 0..chess_data::BIT_AT_INDEX.len()
  {

    temp[i] = chess_data::ZERO_UNICODE.to_string();
    if (bitboard & chess_data::BIT_AT_INDEX[i]) != 0
    {
        temp[i] = chess_data::ONE_UNICODE.to_string();
    }
  }
  format_for_chess_board(&temp)
}
