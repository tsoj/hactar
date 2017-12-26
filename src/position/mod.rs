#![allow(dead_code)]

use chess_data;
use std::ops::{Index,IndexMut};
pub mod piecetype;
pub mod player;

fn format_for_chess_board(field_content: &Vec<String>)->String
{
    let mut s = "".to_string();
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
    s.push_str("  A   B   C   D   E   F   G   H\n");
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

fn find_and_clear_trailing_one(mask: &mut u64) -> usize
{
    let ret = mask.trailing_zeros() as usize;
    *mask &= (*mask) -1;
    ret
}

pub struct Move
{
    pub from: usize,
    pub to: usize,
    pub moved: piecetype::Piecetype,
    pub captured: piecetype::Piecetype,
    pub promoted: piecetype::Piecetype,
    pub en_passant_castling: u64,
    pub zobrist_key: u64,
}
impl Move
{
    pub fn clone_from(&mut self, p: &Move)
    {
        self.from = p.from;
        self.to = p.to;
        self.moved = p.moved;
        self.captured = p.captured;
        self.promoted = p.promoted;
        self.en_passant_castling = p.en_passant_castling;
        self.zobrist_key = p.zobrist_key;
    }
    pub fn empty_move() -> Move
    {
        Move
        {
            from: 0,
            to: 0,
            moved: piecetype::NO_PIECE,
            captured: piecetype::NO_PIECE,
            promoted: piecetype::NO_PIECE,
            en_passant_castling: 0,
            zobrist_key: 0,
        }
    }
    pub fn get_data_string(&self) -> String
    {
        let mut ret = "".to_string();
        ret += "\nMove:\n";
        ret += "--------------------------------------------------\n";
        ret += "\n\tFROM: ";
        ret += &self.from.to_string()[..];
        ret += "\n\tTO: ";
        ret += &self.to.to_string()[..];
        ret += "\n\tMOVED: ";
        ret += &self.moved.to_string()[..];
        ret += "\n\tCAPTURED: ";
        ret += &self.captured.to_string()[..];
        ret += "\n\tPROMOTED: ";
        ret += &self.promoted.to_string()[..];
        ret += "\n\tZOBRIST KEY: ";
        ret += &format!("{:x}",self.zobrist_key)[..];
        ret += "\n\tMOVE: CASTLING / EN PASSANT:\n";
        ret += &get_bitboard_string(self.en_passant_castling)[..];
        ret += "--------------------------------------------------\n";
        ret
    }
}

const MOVE_LIST_MAXIMUM_LENGTH: usize = 128;
pub struct MoveList
{
    pub len: usize,
    a: [Move; MOVE_LIST_MAXIMUM_LENGTH]
}
impl Index<usize> for MoveList {
    type Output = Move;
    fn index<'a>(&'a self, index: usize) -> &'a  Move {
        &self.a[index]
    }
}
impl IndexMut<usize> for MoveList {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Move {
        &mut self.a[index]
    }
}
impl MoveList
{
    pub fn get_empty_move_list() -> MoveList
    {
        MoveList{len: 0, a: unsafe{::std::mem::uninitialized()}}
    }

    pub fn generate_pawn_moves(&mut self, orig_position: &Position, us: player::Player, enemy: player::Player)
    {

    }
    pub fn generate_castling_moves(&mut self, orig_position: &Position, us: player::Player, enemy: player::Player)
    {

    }
    pub fn generate_piece_moves<F>(&mut self, orig_position: &Position, us: player::Player, enemy: player::Player, piece: piecetype::Piecetype, mut get_attack_mask: F)
    where F: FnMut(usize, u64) -> u64
    {
        let mut piece_occupancy = orig_position.pieces[piece] & orig_position.players[us];
        if piece_occupancy != 0
        {
            loop
            {
                let from = find_and_clear_trailing_one(&mut piece_occupancy);

                let occupancy = orig_position.players[player::WHITE] | orig_position.players[player::BLACK];
                let mut quiet_attack_mask = get_attack_mask(from, occupancy);
                let mut capture_attack_mask = quiet_attack_mask & orig_position.players[enemy];
                quiet_attack_mask &= !capture_attack_mask;
                quiet_attack_mask &= !orig_position.players[us];
                if quiet_attack_mask != 0
                {
                    loop
                    {
                        let to = find_and_clear_trailing_one(&mut quiet_attack_mask);

                        let move_list_length = self.len;
                        self[move_list_length] = Move
                        {
                            from: from,
                            to: to,
                            moved: piece,
                            captured: piecetype::NO_PIECE,
                            promoted: piecetype::NO_PIECE,
                            en_passant_castling: orig_position.en_passant_castling,
                            zobrist_key: 0,
                        };
                        self.len+=1;

                        if quiet_attack_mask == 0
                        {
                            break;
                        }
                    }
                }
                if capture_attack_mask != 0
                {
                    loop
                    {
                        let to = find_and_clear_trailing_one(&mut capture_attack_mask);

                        let move_list_length = self.len;
                        self[move_list_length] = Move
                        {
                            from: from,
                            to: to,
                            moved: piece,
                            captured: piecetype::NO_PIECE,
                            promoted: piecetype::NO_PIECE,
                            en_passant_castling: orig_position.en_passant_castling,
                            zobrist_key: 0,
                        };
                        for i in 0..piecetype::NO_PIECE
                        {
                            if (orig_position.pieces[i] & chess_data::BIT_AT_INDEX[to]) != 0
                            {
                                self[move_list_length].captured = i as piecetype::Piecetype;
                                break;
                            }
                        }
                        self.len+=1;

                        if capture_attack_mask == 0
                        {
                            break;
                        }
                    }
                }
                if piece_occupancy == 0
                {
                    break;
                }

            }
        }

    }
}

pub struct Position
{
    pub pieces: [u64; 6], //[Pawns, Knights, Bishops, Rooks, Queens, Kings]
    pub players: [u64; 2], //[White pieces, Black pieces]
    pub en_passant_castling: u64,
    pub zobrist_key: u64,
    pub whose_move: player::Player,
    pub last_move: Move,
    pub fullmoves_played: u32,
    pub halfmove_clock: u32
}
impl Position
{
    pub fn empty_position() -> Position
    {

        Position
        {
            pieces: [0,0,0,0,0,0],
            players: [0,0],
            en_passant_castling: 0,
            zobrist_key: 0,
            whose_move: player::NO_PLAYER,
            last_move:
            Move::empty_move(),
            fullmoves_played: 0,
            halfmove_clock: 0
        }
    }
    pub fn clone_from(&mut self, p: &Position)
    {
        self.pieces.clone_from(&p.pieces);
        self.players.clone_from(&p.players);
        self.en_passant_castling = p.en_passant_castling;
        self.zobrist_key = p.zobrist_key;
        self.whose_move = p.whose_move;
        self.last_move.clone_from(&p.last_move);
        self.fullmoves_played = p.fullmoves_played;
        self.halfmove_clock = p.halfmove_clock;
    }
    pub fn add_piece(&mut self, player: player::Player, piece: piecetype::Piecetype , field: usize)
    {
        self.pieces[piece] |=  chess_data::BIT_AT_INDEX[field];
        self.players[player] |=  chess_data::BIT_AT_INDEX[field];
    }
    pub fn remove_piece(&mut self, player: player::Player, piece: piecetype::Piecetype , field: usize)
    {
        self.pieces[piece] &=  !chess_data::BIT_AT_INDEX[field];
        self.players[player] &=  !chess_data::BIT_AT_INDEX[field];
    }
    pub fn move_piece(&mut self, player: player::Player, piece: piecetype::Piecetype , from: usize,  to: usize)
    {
        self.remove_piece(player, piece, from);
        self.add_piece(player, piece, to);
    }
    pub fn get_chess_board_string(&self) -> String
    {
        let mut temp: Vec<String> = vec![String::new(); 64];
        for  i in 0..chess_data::BIT_AT_INDEX.len()
        {
            temp[i] = " ".to_string();
            if (self.players[player::BLACK] & chess_data::BIT_AT_INDEX[i]) != 0
            {
                if (self.pieces[piecetype::PAWN] & chess_data::BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = chess_data::BLACK_PAWN_UNICODE.to_string();
                }
                else if (self.pieces[piecetype::KNIGTH] & chess_data::BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = chess_data::BLACK_KNIGHT_UNICODE.to_string();
                }
                else if (self.pieces[piecetype::BISHOP] & chess_data::BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = chess_data::BLACK_BISHOP_UNICODE.to_string();
                }
                else if (self.pieces[piecetype::ROOK] & chess_data::BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = chess_data::BLACK_ROOK_UNICODE.to_string();
                }
                else if (self.pieces[piecetype::QUEEN] & chess_data::BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = chess_data::BLACK_QUEEN_UNICODE.to_string();
                }
                else if (self.pieces[piecetype::KING] & chess_data::BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = chess_data::BLACK_KING_UNICODE.to_string();
                }
            }
            else if (self.players[player::WHITE] & chess_data::BIT_AT_INDEX[i]) != 0
            {
                if (self.pieces[piecetype::PAWN] & chess_data::BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = chess_data::WHITE_PAWN_UNICODE.to_string();
                }
                else if (self.pieces[piecetype::KNIGTH] & chess_data::BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = chess_data::WHITE_KNIGHT_UNICODE.to_string();
                }
                else if (self.pieces[piecetype::BISHOP] & chess_data::BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = chess_data::WHITE_BISHOP_UNICODE.to_string();
                }
                else if (self.pieces[piecetype::ROOK] & chess_data::BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = chess_data::WHITE_ROOK_UNICODE.to_string();
                }
                else if (self.pieces[piecetype::QUEEN] & chess_data::BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = chess_data::WHITE_QUEEN_UNICODE.to_string();
                }
                else if (self.pieces[piecetype::KING] & chess_data::BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = chess_data::WHITE_KING_UNICODE.to_string();
                }
            }
        }
        let mut s = format_for_chess_board(&temp);
        s.push_str("\n");
        s.push_str(&(self.fullmoves_played).to_string());
        s.push_str(" moves played.\n");
        if self.whose_move == player::WHITE
        {
            s.push_str("White to move.\n");
        }
        else
        {
            s.push_str("Black to move.\n");
        }
        s
    }
    pub fn get_data_string(&self) -> String
    {
        let mut ret = "".to_string();
        ret += "\nWHOSE MOVE: ";
        ret += &self.whose_move.to_string()[..];
        ret += "\nFULLMOVES PLAYED: ";
        ret += &self.fullmoves_played.to_string()[..];
        ret += "\nHALFMOVE CLOCK: ";
        ret += &self.halfmove_clock.to_string()[..];
        ret += "\nZOBRIST KEY: ";
        ret += &format!("{:x}", self.zobrist_key)[..];
        ret += &self.last_move.get_data_string()[..];
        ret += "CASTLING / EN PASSANT\n";
        ret += &get_bitboard_string(self.en_passant_castling)[..];
        ret += "WHITE:\n";
        ret += &get_bitboard_string(self.players[player::WHITE])[..];
        ret += "BLACK:\n";
        ret += &get_bitboard_string(self.players[player::BLACK])[..];
        ret += "PAWNS:\n";
        ret += &get_bitboard_string(self.pieces[piecetype::PAWN])[..];
        ret += "KNIGHTS:\n";
        ret += &get_bitboard_string(self.pieces[piecetype::KNIGTH])[..];
        ret += "BISHOPS:\n";
        ret += &get_bitboard_string(self.pieces[piecetype::BISHOP])[..];
        ret += "ROOKS:\n";
        ret += &get_bitboard_string(self.pieces[piecetype::ROOK])[..];
        ret += "QUEENS:\n";
        ret += &get_bitboard_string(self.pieces[piecetype::QUEEN])[..];
        ret += "KINGS:\n";
        ret += &get_bitboard_string(self.pieces[piecetype::KING])[..];
        ret
    }
    pub fn set_from_fen(&mut self, fen: &String) -> bool
    {
        let mut p = Position::empty_position();
        let mut iter = (*fen).split_whitespace();
        let piece_placement = iter.next().unwrap().to_string();
        let active_color = iter.next().unwrap().to_string();
        let castling_availability = iter.next().unwrap().to_string();
        let en_passant_target_square = iter.next().unwrap().to_string();
        let halfmove_clock = iter.next().unwrap().to_string();//fifty-move rule
        let fullmove_number = iter.next().unwrap().to_string();

        let mut field_counter: usize = 56;
        for i in piece_placement.chars()
        {
            match i
            {
                '/' => field_counter-=16,
                '8' => field_counter+=8,
                '7' => field_counter+=7,
                '6' => field_counter+=6,
                '5' => field_counter+=5,
                '4' => field_counter+=4,
                '3' => field_counter+=3,
                '2' => field_counter+=2,
                '1' => field_counter+=1,
                '0' => field_counter+=0,
                'P' =>
                {
                    p.add_piece(player::WHITE, piecetype::PAWN, field_counter);
                    field_counter+=1;
                },
                'N' =>
                {
                    p.add_piece(player::WHITE, piecetype::KNIGTH, field_counter);
                    field_counter+=1;
                },
                'B' =>
                {
                    p.add_piece(player::WHITE, piecetype::BISHOP, field_counter);
                    field_counter+=1;
                },
                'R' =>
                {
                    p.add_piece(player::WHITE, piecetype::ROOK, field_counter);
                    field_counter+=1;
                },
                'Q' =>
                {
                    p.add_piece(player::WHITE, piecetype::QUEEN, field_counter);
                    field_counter+=1;
                },
                'K' =>
                {
                    p.add_piece(player::WHITE, piecetype::KING, field_counter);
                    field_counter+=1;
                },
                'p' =>
                {
                    p.add_piece(player::BLACK, piecetype::PAWN, field_counter);
                    field_counter+=1;
                },
                'n' =>
                {
                    p.add_piece(player::BLACK, piecetype::KNIGTH, field_counter);
                    field_counter+=1;
                },
                'b' =>
                {
                    p.add_piece(player::BLACK, piecetype::BISHOP, field_counter);
                    field_counter+=1;
                },
                'r' =>
                {
                    p.add_piece(player::BLACK, piecetype::ROOK, field_counter);
                    field_counter+=1;
                },
                'q' =>
                {
                    p.add_piece(player::BLACK, piecetype::QUEEN, field_counter);
                    field_counter+=1;
                },
                'k' =>
                {
                    p.add_piece(player::BLACK, piecetype::KING, field_counter);
                    field_counter+=1;
                },
                _x =>
                {
                    println!("FEN-string not formatted properly.");
                    return false;
                }
            }
        }
        if active_color == "w" || active_color == "W"
        {
            p.whose_move = player::WHITE;
        }
        else if active_color == "b" || active_color == "B"
        {
            p.whose_move = player::BLACK;
        }
        else
        {
            println!("FEN-string not formatted properly.");
            return false;
        }
        for i in castling_availability.chars()
        {
            match i
            {
                '-' => {},
                'K' => p.en_passant_castling |= chess_data::CASTLING_KINGSIDE_ROOK_FROM[player::WHITE] | chess_data::CASTLING_KING_FROM[player::WHITE],
                'k' => p.en_passant_castling |= chess_data::CASTLING_KINGSIDE_ROOK_FROM[player::BLACK] | chess_data::CASTLING_KING_FROM[player::BLACK],
                'Q' => p.en_passant_castling |= chess_data::CASTLING_QUEENSIDE_ROOK_FROM[player::WHITE] | chess_data::CASTLING_KING_FROM[player::WHITE],
                'q' => p.en_passant_castling |= chess_data::CASTLING_QUEENSIDE_ROOK_FROM[player::BLACK] | chess_data::CASTLING_KING_FROM[player::BLACK],
                _x =>
                {
                    println!("FEN-string not formatted properly.");
                    return false;
                }
            }
        }
        if en_passant_target_square!="-"
        {
            let en_passant_target_field_index = chess_data::get_field_index(&en_passant_target_square[..]);
            if en_passant_target_field_index == 64
            {
                println!("FEN-string not formatted properly.");
                return false;
            }
            p.en_passant_castling |= chess_data::BIT_AT_INDEX[en_passant_target_field_index];
        }
        p.halfmove_clock = halfmove_clock.parse::<u32>().unwrap();
        p.fullmoves_played = fullmove_number.parse::<u32>().unwrap();
        p.zobrist_key = p.calculate_zobristkey();
        self.clone_from(&p);
        true
    }
    pub fn calculate_zobristkey(&self) -> u64
    {
        let mut ret: u64 = 0;
        for i in 0..piecetype::NO_PIECE
        {
            if self.pieces[i] != 0
            {
                let mut temp_occupancy = self.pieces[i];
                loop
                {
                    let field_index = temp_occupancy.trailing_zeros() as usize;
                    if (chess_data::BIT_AT_INDEX[field_index] & self.players[player::WHITE])!=0
                    {
                        ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PLAYERS[player::WHITE][field_index];
                    }
                    else
                    {
                        ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PLAYERS[player::BLACK][field_index];
                    }
                    ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PIECES[i][field_index];
                    temp_occupancy &= !chess_data::BIT_AT_INDEX[field_index];
                    if temp_occupancy == 0
                    {
                        break;
                    }
                }
            }
        }
        ret ^= self.en_passant_castling;
        ret ^= self.whose_move as u64;
        ret
    }
    pub fn update_zobristkey(&self, m: &Move) -> u64
    {
        let mut ret: u64 = self.zobrist_key;

        ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PIECES[m.moved][m.from];
        if m.promoted != piecetype::NO_PIECE
        {
            ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PIECES[m.promoted][m.to];
        }
        else
        {
            ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PIECES[m.moved][m.to];
        }

        ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PLAYERS[self.whose_move][m.from];
        ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PLAYERS[self.whose_move][m.to];
        if m.captured != piecetype::NO_PIECE
        {
            ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PIECES[player::switch_player(self.whose_move)][m.to];
            ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PLAYERS[player::switch_player(self.whose_move)][m.to];
        }

        ret ^= self.en_passant_castling;
        ret ^= self.whose_move as u64;
        ret ^= m.en_passant_castling;
        ret ^= player::switch_player(self.whose_move) as u64;

        if (m.en_passant_castling & (chess_data::FILES[2] | chess_data::FILES[5])) != (self.en_passant_castling & (chess_data::FILES[2] | chess_data::FILES[5]))
        {
            //TODO: en passant...
        }

        if (m.en_passant_castling & (chess_data::FILES[0] | chess_data::FILES[7])) != (self.en_passant_castling & (chess_data::FILES[0] | chess_data::FILES[7]))
        {
            //TODO: castling...
        }

        ret
    }
    pub fn generate_move_list(&self, us: player::Player, enemy: player::Player) -> MoveList
    {
        let mut move_list = MoveList::get_empty_move_list();
        move_list.generate_pawn_moves(&self, us, enemy);
        move_list.generate_castling_moves(&self, us, enemy);
        move_list.generate_piece_moves(&self, us, enemy, piecetype::KNIGTH, chess_data::get_attack_mask_knight);
        move_list.generate_piece_moves(&self, us, enemy, piecetype::BISHOP, chess_data::get_attack_mask_bishop);
        move_list.generate_piece_moves(&self, us, enemy, piecetype::ROOK, chess_data::get_attack_mask_rook);
        move_list.generate_piece_moves(&self, us, enemy, piecetype::QUEEN, chess_data::get_attack_mask_queen);
        move_list.generate_piece_moves(&self, us, enemy, piecetype::KING, chess_data::get_attack_mask_king);
        move_list
    }
}
