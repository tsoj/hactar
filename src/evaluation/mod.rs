pub mod score;
use chess_data;
use position::Position;
use position::piece::{PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING};
use evaluation::score::{VALUE_PAWN, VALUE_KNIGHT, VALUE_BISHOP, VALUE_ROOK, VALUE_QUEEN};

impl Position
{
    pub fn evaluate(&self) -> score::Score
    {
        let us = self.us;
        let mut ret = 0;
        if self.pieces[PAWN] != 0
        {
            let mut temp_occupancy = self.pieces[PAWN];
            loop
            {
                let index = chess_data::find_and_clear_trailing_one(&mut temp_occupancy);
                if chess_data::BIT_AT_INDEX[index] & self.players[us] != 0
                {
                    ret += VALUE_PAWN;
                }
                else
                {
                    ret -= VALUE_PAWN;
                }
                if temp_occupancy == 0
                {
                    break;
                }
            }
        }
        if self.pieces[KNIGHT] != 0
        {
            let mut temp_occupancy = self.pieces[KNIGHT];
            loop
            {
                let index = chess_data::find_and_clear_trailing_one(&mut temp_occupancy);
                if chess_data::BIT_AT_INDEX[index] & self.players[us] != 0
                {
                    ret += VALUE_KNIGHT;
                }
                else
                {
                    ret -= VALUE_KNIGHT;
                }
                if temp_occupancy == 0
                {
                    break;
                }
            }
        }
        if self.pieces[BISHOP] != 0
        {
            let mut temp_occupancy = self.pieces[BISHOP];
            loop
            {
                let index = chess_data::find_and_clear_trailing_one(&mut temp_occupancy);
                if chess_data::BIT_AT_INDEX[index] & self.players[us] != 0
                {
                    ret += VALUE_BISHOP;
                }
                else
                {
                    ret -= VALUE_BISHOP;
                }
                if temp_occupancy == 0
                {
                    break;
                }
            }
        }
        if self.pieces[ROOK] != 0
        {
            let mut temp_occupancy = self.pieces[ROOK];
            loop
            {
                let index = chess_data::find_and_clear_trailing_one(&mut temp_occupancy);
                if chess_data::BIT_AT_INDEX[index] & self.players[us] != 0
                {
                    ret += VALUE_ROOK;
                }
                else
                {
                    ret -= VALUE_ROOK;
                }
                if temp_occupancy == 0
                {
                    break;
                }
            }
        }
        if self.pieces[QUEEN] != 0
        {
            let mut temp_occupancy = self.pieces[QUEEN];
            loop
            {
                let index = chess_data::find_and_clear_trailing_one(&mut temp_occupancy);
                if chess_data::BIT_AT_INDEX[index] & self.players[us] != 0
                {
                    ret += VALUE_QUEEN;
                }
                else
                {
                    ret -= VALUE_QUEEN;
                }
                if temp_occupancy == 0
                {
                    break;
                }
            }
        }
        if self.pieces[KING] != 0
        {
            let mut temp_occupancy = self.pieces[KING];
            loop
            {
                let _index = chess_data::find_and_clear_trailing_one(&mut temp_occupancy);
                if temp_occupancy == 0
                {
                    break;
                }
            }
        }
        ret
    }
}
