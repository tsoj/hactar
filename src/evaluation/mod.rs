#![allow(unused_variables)]
#![allow(unused_mut)]

pub mod score;
use chess_data::*;
use position::Position;
use position::piece::{Piece, PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING};
use position::player::{Player, BLACK, WHITE};
use evaluation::score::{Score, SCORE};

/*
TODO:
- passed pawns ---DONE
- double pawns / pawnstucture ---DONE
- king safety
- moveability
- rooks together, castling
- important squares
*/
const BONUS_PASSED_PAWN: Score = 10;
const PENALTY_HAS_NO_NEIGHBOR_PAWN: Score = 10;
const BONUS_ROOKS_ARE_CONNECTED: Score = 20;
const BONUS_KNIGHT_NOT_ON_EDGE: Score = 10;
const PENALTY_BISHOP_ON_HOMERANK: Score = 10;
const PENALTY_KING_UNSAFE: Score = 40;

#[inline(always)]
fn evaluate_pawn(position: &Position, index: usize, us: Player, enemy: Player) -> Score
{
    let mut ret = 0;
    //passed pawn
    if IS_PASSED[us][index] & position.pieces[PAWN] & position.players[enemy] == 0
    {
        let file = (index/8) as i32;
        let relative_file = (file - 4 - 4*MOVE_DIRECTION[us])*MOVE_DIRECTION[us];
        ret += BONUS_PASSED_PAWN+relative_file*20;
    }
    //pawn structure
    if index%8 != 0 && position.pieces[PAWN] & position.players[us] & FILES_64[index - 1]== 0
    {
        ret -= PENALTY_HAS_NO_NEIGHBOR_PAWN;
    }
    if index%8 != 7 && position.pieces[PAWN] & position.players[us]  & FILES_64[index + 1]== 0
    {
        ret -= PENALTY_HAS_NO_NEIGHBOR_PAWN;
    }
    ret
}
#[inline(always)]
fn evaluate_knight(position: &Position, index: usize, us: Player, enemy: Player) -> Score
{
    let mut ret = 0;
    //Knights on the edge are bad
    if BIT_AT_INDEX[index] & CENTER_7X7 != 0
    {
        ret += BONUS_KNIGHT_NOT_ON_EDGE;
    }
    ret
}
#[inline(always)]
fn evaluate_bishop(position: &Position, index: usize, us: Player, enemy: Player) -> Score
{
    let mut ret = 0;
    if BIT_AT_INDEX[index] & HOME_RANK[us] != 0
    {
        ret -= PENALTY_BISHOP_ON_HOMERANK;
    }
    ret
}
#[inline(always)]
fn evaluate_rook(position: &Position, index: usize, us: Player, enemy: Player) -> Score
{
    let mut ret = 0;
    //connected rooks are better
    let attack_mask = get_attack_mask_rook(index, position.players[WHITE] | position.players[BLACK]);
    if attack_mask & position.pieces[ROOK] & position.players[us] & !BIT_AT_INDEX[index] != 0
    {
        ret += BONUS_ROOKS_ARE_CONNECTED;
    }
    ret
}
#[inline(always)]
fn evaluate_queen(position: &Position, index: usize, us: Player, enemy: Player) -> Score
{
    let mut ret = 0;
    ret
}
#[inline(always)]
fn evaluate_king(position: &Position, index: usize, us: Player, enemy: Player) -> Score
{
    let mut ret = 0;
    let mut king_attacked_area = BIT_AT_INDEX[index] | (get_attack_mask_king(index, 0) & !position.players[us]);
    loop
    {
        let attacked_square_index = find_and_clear_trailing_one(&mut king_attacked_area);
        if position.is_check(us, enemy, attacked_square_index)
        {
            ret -= PENALTY_KING_UNSAFE;
        }
        if king_attacked_area == 0
        {
            break;
        }
    }
    ret
}
#[warn(unused_variables)]
#[warn(unused_mut)]

impl Position
{
    #[inline(always)]
    fn evaluate_all<F>(&self, piece: Piece, evaluate_piece: F) -> Score
    where F: Fn(&Position, usize, Player, Player) -> Score
    {
        let us = self.us;
        let enemy = self.enemy;
        let mut ret = 0;
        if self.pieces[piece] != 0
        {
            let mut temp_occupancy = self.pieces[piece];
            loop
            {
                let index = find_and_clear_trailing_one(&mut temp_occupancy);
                if BIT_AT_INDEX[index] & self.players[us] != 0
                {
                    ret += evaluate_piece(&self, index, us, enemy);
                    ret += SCORE[piece];
                }
                else
                {
                    ret -= evaluate_piece(&self, index, enemy, us);
                    ret -= SCORE[piece];
                }
                if temp_occupancy == 0
                {
                    break;
                }
            }
        }
        ret
    }

    pub fn evaluate(&self) -> Score
    {
        if self.halfmove_clock >= 100
        {
            return 0;
        }
        let mut ret = 0;
        ret += self.evaluate_all(PAWN, evaluate_pawn);
        ret += self.evaluate_all(KNIGHT, evaluate_knight);
        ret += self.evaluate_all(BISHOP, evaluate_bishop);
        ret += self.evaluate_all(ROOK, evaluate_rook);
        ret += self.evaluate_all(QUEEN, evaluate_queen);
        ret += self.evaluate_all(KING, evaluate_king);
        ret
    }
}
