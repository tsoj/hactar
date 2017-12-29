pub mod score;
use chess_data;
use position;

pub fn evaluate(position: &position::Position, us: position::player::Player, _enemy: position::player::Player) -> score::Score
{
    let mut ret = 0;
    if position.pieces[position::piecetype::PAWN] != 0
    {
        let mut temp_occupancy = position.pieces[position::piecetype::PAWN];
        loop
        {
            let index = chess_data::find_and_clear_trailing_one(&mut temp_occupancy);
            if chess_data::BIT_AT_INDEX[index] & position.players[us] != 0
            {
                ret += score::VALUE_PAWN;
            }
            else
            {
                ret -= score::VALUE_PAWN;
            }
            if temp_occupancy == 0
            {
                break;
            }
        }
    }
    if position.pieces[position::piecetype::KNIGHT] != 0
    {
        let mut temp_occupancy = position.pieces[position::piecetype::KNIGHT];
        loop
        {
            let index = chess_data::find_and_clear_trailing_one(&mut temp_occupancy);
            if chess_data::BIT_AT_INDEX[index] & position.players[us] != 0
            {
                ret += score::VALUE_KNIGHT;
            }
            else
            {
                ret -= score::VALUE_KNIGHT;
            }
            if temp_occupancy == 0
            {
                break;
            }
        }
    }
    if position.pieces[position::piecetype::BISHOP] != 0
    {
        let mut temp_occupancy = position.pieces[position::piecetype::BISHOP];
        loop
        {
            let index = chess_data::find_and_clear_trailing_one(&mut temp_occupancy);
            if chess_data::BIT_AT_INDEX[index] & position.players[us] != 0
            {
                ret += score::VALUE_BISHOP;
            }
            else
            {
                ret -= score::VALUE_BISHOP;
            }
            if temp_occupancy == 0
            {
                break;
            }
        }
    }
    if position.pieces[position::piecetype::ROOK] != 0
    {
        let mut temp_occupancy = position.pieces[position::piecetype::ROOK];
        loop
        {
            let index = chess_data::find_and_clear_trailing_one(&mut temp_occupancy);
            if chess_data::BIT_AT_INDEX[index] & position.players[us] != 0
            {
                ret += score::VALUE_ROOK;
            }
            else
            {
                ret -= score::VALUE_ROOK;
            }
            if temp_occupancy == 0
            {
                break;
            }
        }
    }
    if position.pieces[position::piecetype::QUEEN] != 0
    {
        let mut temp_occupancy = position.pieces[position::piecetype::QUEEN];
        loop
        {
            let index = chess_data::find_and_clear_trailing_one(&mut temp_occupancy);
            if chess_data::BIT_AT_INDEX[index] & position.players[us] != 0
            {
                ret += score::VALUE_QUEEN;
            }
            else
            {
                ret -= score::VALUE_QUEEN;
            }
            if temp_occupancy == 0
            {
                break;
            }
        }
    }
    if position.pieces[position::piecetype::KING] != 0
    {
        let mut temp_occupancy = position.pieces[position::piecetype::KING];
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
