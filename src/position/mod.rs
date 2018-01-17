#![allow(dead_code)]
use chess_data::*;
pub mod piece;
pub mod player;
pub mod mov;

use position::mov::*;
use position::piece::{Piece, PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING, NO_PIECE};
use position::player::{Player, WHITE, BLACK, NO_PLAYER};

fn format_for_chess_board(field_content: &Vec<String>)->String
{
    let mut s = "".to_string();
    for _ in 0..33
    {
        s.push_str(HORIZONTAL_LINE_UNICODE);
    }
    s.push_str("\n");
    for h in 0..8
    {
        let i = 7 - h;
        for j in 0..8
        {
            s.push_str(VERTICAL_LINE_UNICODE);
            s.push_str(" ");
            s.push_str(&field_content[8*i + j]);
            s.push_str(" ");
        }
        s.push_str(VERTICAL_LINE_UNICODE);
        s.push_str(" ");
        s.push_str(&((i+1) as u32).to_string());
        s.push_str("\n");
        for _ in 0..33
        {
            s.push_str(HORIZONTAL_LINE_UNICODE);
        }
        s.push_str("\n");
    }
    s.push_str("  A   B   C   D   E   F   G   H\n");
    s
}
fn format_for_fen(field_content: &Vec<String>)->String
{
    let mut s = "".to_string();
    for h in 0..8
    {
        let i = 7 - h;
        for j in 0..8
        {
            s.push_str(&field_content[8*i + j]);
        }
        if i > 0
        {
            s.push_str("/");
        }
    }
    s = str::replace(&s, "11111111", "8");
    s = str::replace(&s, "1111111", "7");
    s = str::replace(&s, "111111", "6");
    s = str::replace(&s, "11111", "5");
    s = str::replace(&s, "1111", "4");
    s = str::replace(&s, "111", "3");
    s = str::replace(&s, "11", "2");
    s
}
pub fn get_bitboard_string(bitboard: u64) -> String
{
  let mut temp: Vec<String> = vec![String::new(); 64];
  for  i in 0..BIT_AT_INDEX.len()
  {

    temp[i] = ZERO_UNICODE.to_string();
    if (bitboard & BIT_AT_INDEX[i]) != 0
    {
        temp[i] = ONE_UNICODE.to_string();
    }
  }
  format_for_chess_board(&temp)
}

#[derive(Copy, Clone)]
pub struct Position
{
    pub pieces: [u64; 6], //[Pawns, Knights, Bishops, Rooks, Queens, Kings]
    pub players: [u64; 2], //[White pieces, Black pieces]
    pub en_passant_castling: u64,
    pub zobrist_key: u64,
    pub us: Player,
    pub enemy: Player,
    pub fullmoves_played: u16,
    pub halfmove_clock: u16
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
            us: NO_PLAYER,
            enemy: NO_PLAYER,
            fullmoves_played: 0,
            halfmove_clock: 0
        }
    }
    pub fn add_piece(&mut self, player: Player, piece: Piece , field: u64)
    {
        self.pieces[piece] |=  field;
        self.players[player] |=  field;
    }
    pub fn remove_piece(&mut self, player: Player, piece: Piece , field: u64)
    {
        self.pieces[piece] &=  !field;
        self.players[player] &=  !field;
    }
    pub fn move_piece(&mut self, player: Player, piece: Piece , from: u64,  to: u64)
    {
        self.remove_piece(player, piece, from);
        self.add_piece(player, piece, to);
    }
    pub fn get_chess_board_string(&self) -> String
    {
        let mut temp: Vec<String> = vec![String::new(); 64];
        for  i in 0..BIT_AT_INDEX.len()
        {
            temp[i] = " ".to_string();
            if (self.players[BLACK] & BIT_AT_INDEX[i]) != 0
            {
                if (self.pieces[PAWN] & BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = BLACK_PAWN_UNICODE.to_string();
                }
                else if (self.pieces[KNIGHT] & BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = BLACK_KNIGHT_UNICODE.to_string();
                }
                else if (self.pieces[BISHOP] & BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = BLACK_BISHOP_UNICODE.to_string();
                }
                else if (self.pieces[ROOK] & BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = BLACK_ROOK_UNICODE.to_string();
                }
                else if (self.pieces[QUEEN] & BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = BLACK_QUEEN_UNICODE.to_string();
                }
                else if (self.pieces[KING] & BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = BLACK_KING_UNICODE.to_string();
                }
            }
            else if (self.players[WHITE] & BIT_AT_INDEX[i]) != 0
            {
                if (self.pieces[PAWN] & BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = WHITE_PAWN_UNICODE.to_string();
                }
                else if (self.pieces[KNIGHT] & BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = WHITE_KNIGHT_UNICODE.to_string();
                }
                else if (self.pieces[BISHOP] & BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = WHITE_BISHOP_UNICODE.to_string();
                }
                else if (self.pieces[ROOK] & BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = WHITE_ROOK_UNICODE.to_string();
                }
                else if (self.pieces[QUEEN] & BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = WHITE_QUEEN_UNICODE.to_string();
                }
                else if (self.pieces[KING] & BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = WHITE_KING_UNICODE.to_string();
                }
            }
        }
        let mut s = format_for_chess_board(&temp);
        s.push_str(&(self.fullmoves_played).to_string());
        s.push_str(" moves played.\n");
        if self.us == WHITE
        {
            s.push_str("White to move.");
        }
        else
        {
            s.push_str("Black to move.");
        }
        s
    }
    pub fn get_fen_string(&self) -> String
    {
        let mut temp: Vec<String> = vec![String::new(); 64];
        for  i in 0..BIT_AT_INDEX.len()
        {
            temp[i] = "1".to_string();
            if (self.players[BLACK] & BIT_AT_INDEX[i]) != 0
            {
                if (self.pieces[PAWN] & BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = "p".to_string();
                }
                else if (self.pieces[KNIGHT] & BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = "n".to_string();
                }
                else if (self.pieces[BISHOP] & BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = "b".to_string();
                }
                else if (self.pieces[ROOK] & BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = "r".to_string();
                }
                else if (self.pieces[QUEEN] & BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = "q".to_string();
                }
                else if (self.pieces[KING] & BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = "k".to_string();
                }
            }
            else if (self.players[WHITE] & BIT_AT_INDEX[i]) != 0
            {
                if (self.pieces[PAWN] & BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = "P".to_string();
                }
                else if (self.pieces[KNIGHT] & BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = "N".to_string();
                }
                else if (self.pieces[BISHOP] & BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = "B".to_string();
                }
                else if (self.pieces[ROOK] & BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = "R".to_string();
                }
                else if (self.pieces[QUEEN] & BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = "Q".to_string();
                }
                else if (self.pieces[KING] & BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = "K".to_string();
                }
            }
        }
        let mut fen = format_for_fen(&temp);
        if self.us == WHITE
        {
            fen.push_str(" w ");
        }
        else
        {
            fen.push_str(" b ");
        }
        let castling = self.en_passant_castling & (RANKS[0] | RANKS[7]);
        if castling != 0
        {
            if castling & CASTLING_KING_FROM[WHITE] != 0 &&
               castling & CASTLING_KINGSIDE_ROOK_FROM[WHITE] != 0
            {
                fen.push_str("K");
            }
            if castling & CASTLING_KING_FROM[WHITE] != 0 &&
               castling & CASTLING_QUEENSIDE_ROOK_FROM[WHITE] != 0
            {
                fen.push_str("Q");
            }
            if castling & CASTLING_KING_FROM[BLACK] != 0 &&
               castling & CASTLING_KINGSIDE_ROOK_FROM[BLACK] != 0
            {
                fen.push_str("k");
            }
            if castling & CASTLING_KING_FROM[BLACK] != 0 &&
               castling & CASTLING_QUEENSIDE_ROOK_FROM[BLACK] != 0
            {
                fen.push_str("q");
            }
            fen.push_str(" ");
        }
        else
        {
            fen.push_str("- ");
        }
        let en_passant = self.en_passant_castling & !castling;
        if en_passant != 0
        {
            fen.push_str(get_field_notation(find_and_clear_trailing_one(&mut en_passant.clone())));
        }
        else
        {
            fen.push_str("-");
        }
        fen.push_str(" ");
        fen.push_str(&(self.halfmove_clock).to_string());
        fen.push_str(" ");
        fen.push_str(&(self.fullmoves_played).to_string());
        fen
    }
    pub fn get_data_string(&self) -> String
    {
        let mut ret = "".to_string();
        ret += "\nWHOSE MOVE: ";
        ret += &self.us.to_string()[..];
        ret += "\nFULLMOVES PLAYED: ";
        ret += &self.fullmoves_played.to_string()[..];
        ret += "\nHALFMOVE CLOCK: ";
        ret += &self.halfmove_clock.to_string()[..];
        ret += "\nZOBRIST KEY: ";
        ret += &format!("{:x}", self.zobrist_key)[..];
        ret += "\nCASTLING / EN PASSANT\n";
        ret += &get_bitboard_string(self.en_passant_castling)[..];
        ret += "WHITE:\n";
        ret += &get_bitboard_string(self.players[WHITE])[..];
        ret += "BLACK:\n";
        ret += &get_bitboard_string(self.players[BLACK])[..];
        ret += "PAWNS:\n";
        ret += &get_bitboard_string(self.pieces[PAWN])[..];
        ret += "KNIGHTS:\n";
        ret += &get_bitboard_string(self.pieces[KNIGHT])[..];
        ret += "BISHOPS:\n";
        ret += &get_bitboard_string(self.pieces[BISHOP])[..];
        ret += "ROOKS:\n";
        ret += &get_bitboard_string(self.pieces[ROOK])[..];
        ret += "QUEENS:\n";
        ret += &get_bitboard_string(self.pieces[QUEEN])[..];
        ret += "KINGS:\n";
        ret += &get_bitboard_string(self.pieces[KING])[..];
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
                    p.add_piece(WHITE, PAWN, BIT_AT_INDEX[field_counter]);
                    field_counter+=1;
                },
                'N' =>
                {
                    p.add_piece(WHITE, KNIGHT, BIT_AT_INDEX[field_counter]);
                    field_counter+=1;
                },
                'B' =>
                {
                    p.add_piece(WHITE, BISHOP, BIT_AT_INDEX[field_counter]);
                    field_counter+=1;
                },
                'R' =>
                {
                    p.add_piece(WHITE, ROOK, BIT_AT_INDEX[field_counter]);
                    field_counter+=1;
                },
                'Q' =>
                {
                    p.add_piece(WHITE, QUEEN, BIT_AT_INDEX[field_counter]);
                    field_counter+=1;
                },
                'K' =>
                {
                    p.add_piece(WHITE, KING, BIT_AT_INDEX[field_counter]);
                    field_counter+=1;
                },
                'p' =>
                {
                    p.add_piece(BLACK, PAWN, BIT_AT_INDEX[field_counter]);
                    field_counter+=1;
                },
                'n' =>
                {
                    p.add_piece(BLACK, KNIGHT, BIT_AT_INDEX[field_counter]);
                    field_counter+=1;
                },
                'b' =>
                {
                    p.add_piece(BLACK, BISHOP, BIT_AT_INDEX[field_counter]);
                    field_counter+=1;
                },
                'r' =>
                {
                    p.add_piece(BLACK, ROOK, BIT_AT_INDEX[field_counter]);
                    field_counter+=1;
                },
                'q' =>
                {
                    p.add_piece(BLACK, QUEEN, BIT_AT_INDEX[field_counter]);
                    field_counter+=1;
                },
                'k' =>
                {
                    p.add_piece(BLACK, KING, BIT_AT_INDEX[field_counter]);
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
            p.us = WHITE;
            p.enemy = BLACK;
        }
        else if active_color == "b" || active_color == "B"
        {
            p.us = BLACK;
            p.enemy = WHITE;
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
                'K' => p.en_passant_castling |= CASTLING_KINGSIDE_ROOK_FROM[WHITE] | CASTLING_KING_FROM[WHITE],
                'k' => p.en_passant_castling |= CASTLING_KINGSIDE_ROOK_FROM[BLACK] | CASTLING_KING_FROM[BLACK],
                'Q' => p.en_passant_castling |= CASTLING_QUEENSIDE_ROOK_FROM[WHITE] | CASTLING_KING_FROM[WHITE],
                'q' => p.en_passant_castling |= CASTLING_QUEENSIDE_ROOK_FROM[BLACK] | CASTLING_KING_FROM[BLACK],
                _x =>
                {
                    println!("FEN-string not formatted properly.");
                    return false;
                }
            }
        }
        if en_passant_target_square!="-"
        {
            let en_passant_target_field_index = get_field_index(&en_passant_target_square[..]);
            if en_passant_target_field_index == 64
            {
                println!("FEN-string not formatted properly.");
                return false;
            }
            p.en_passant_castling |= BIT_AT_INDEX[en_passant_target_field_index];
        }
        p.halfmove_clock = halfmove_clock.parse::<u16>().unwrap();
        p.fullmoves_played = fullmove_number.parse::<u16>().unwrap();
        p.zobrist_key = p.calculate_zobristkey();
        self.clone_from(&p);
        true
    }
    pub fn calculate_zobristkey(&self) -> u64
    {
        let mut ret: u64 = 0;
        for i in 0..NO_PIECE
        {
            if self.pieces[i] != 0
            {
                let mut temp_occupancy = self.pieces[i];
                loop
                {
                    let field_index = temp_occupancy.trailing_zeros() as usize;
                    if (BIT_AT_INDEX[field_index] & self.players[WHITE])!=0
                    {
                        ret ^= ZOBRIST_RANDOM_BITMASKS_PLAYERS[WHITE][field_index];
                    }
                    else
                    {
                        ret ^= ZOBRIST_RANDOM_BITMASKS_PLAYERS[BLACK][field_index];
                    }
                    ret ^= ZOBRIST_RANDOM_BITMASKS_PIECES[i][field_index];
                    temp_occupancy &= !BIT_AT_INDEX[field_index];
                    if temp_occupancy == 0
                    {
                        break;
                    }
                }
            }
        }
        ret ^= self.en_passant_castling;
        ret ^= self.us as u64;
        ret
    }
    pub fn get_updated_zobristkey(&self, m: &Move) -> u64
    {
        let mut ret: u64 = self.zobrist_key;

        ret ^= ZOBRIST_RANDOM_BITMASKS_PIECES[m.moved][m.from];
        if m.promoted != NO_PIECE
        {
            ret ^= ZOBRIST_RANDOM_BITMASKS_PIECES[m.promoted][m.to];
        }
        else
        {
            ret ^= ZOBRIST_RANDOM_BITMASKS_PIECES[m.moved][m.to];
        }

        ret ^= ZOBRIST_RANDOM_BITMASKS_PLAYERS[self.us][m.from];
        ret ^= ZOBRIST_RANDOM_BITMASKS_PLAYERS[self.us][m.to];

        if m.captured != NO_PIECE && !m.captured_en_passant
        {
            ret ^= ZOBRIST_RANDOM_BITMASKS_PIECES[m.captured][m.to];
            ret ^= ZOBRIST_RANDOM_BITMASKS_PLAYERS[self.enemy][m.to];
        }

        ret ^= self.en_passant_castling;
        ret ^= m.en_passant_castling;
        ret ^= WHITE as u64;
        ret ^= BLACK as u64;

        if m.captured_en_passant
        {
            let captured_index = PAWN_QUIET_ATTACK_TABLE[self.enemy][m.to].trailing_zeros() as usize;
            ret ^= ZOBRIST_RANDOM_BITMASKS_PIECES[PAWN][captured_index];
            ret ^= ZOBRIST_RANDOM_BITMASKS_PLAYERS[self.enemy][captured_index];
        }

        if m.castled
        {
            //IF QUEENSIDE
            if m.to == CASTLING_QUEENSIDE_KING_TO_INDEX[self.us]
            {
                ret ^= ZOBRIST_RANDOM_BITMASKS_PIECES[ROOK][CASTLING_QUEENSIDE_ROOK_FROM_INDEX[self.us]];
                ret ^= ZOBRIST_RANDOM_BITMASKS_PIECES[ROOK][CASTLING_QUEENSIDE_ROOK_TO_INDEX[self.us]];
                ret ^= ZOBRIST_RANDOM_BITMASKS_PLAYERS[self.us][CASTLING_QUEENSIDE_ROOK_FROM_INDEX[self.us]];
                ret ^= ZOBRIST_RANDOM_BITMASKS_PLAYERS[self.us][CASTLING_QUEENSIDE_ROOK_TO_INDEX[self.us]];
            }
            //IF KINGSIDE
            else
            {
                ret ^= ZOBRIST_RANDOM_BITMASKS_PIECES[ROOK][CASTLING_KINGSIDE_ROOK_FROM_INDEX[self.us]];
                ret ^= ZOBRIST_RANDOM_BITMASKS_PIECES[ROOK][CASTLING_KINGSIDE_ROOK_TO_INDEX[self.us]];
                ret ^= ZOBRIST_RANDOM_BITMASKS_PLAYERS[self.us][CASTLING_KINGSIDE_ROOK_FROM_INDEX[self.us]];
                ret ^= ZOBRIST_RANDOM_BITMASKS_PLAYERS[self.us][CASTLING_KINGSIDE_ROOK_TO_INDEX[self.us]];
            }
        }
        ret
    }
    pub fn generate_move_list(&self) -> MoveList
    {
        let mut move_list = MoveList::empty_move_list();
        let new_en_passant_castling = self.en_passant_castling & (RANKS[0] | RANKS[7]);
        move_list.generate_pawn_moves(&self, new_en_passant_castling, false);
        move_list.generate_castling_moves(&self, new_en_passant_castling);
        move_list.generate_piece_moves(&self, KNIGHT, get_attack_mask_knight, new_en_passant_castling, false);
        move_list.generate_piece_moves(&self, BISHOP, get_attack_mask_bishop, new_en_passant_castling, false);
        move_list.generate_piece_moves(&self, ROOK, get_attack_mask_rook, new_en_passant_castling, false);
        move_list.generate_piece_moves(&self, QUEEN, get_attack_mask_queen, new_en_passant_castling, false);
        move_list.generate_piece_moves(&self, KING, get_attack_mask_king, new_en_passant_castling, false);
        move_list
    }
    pub fn generate_capture_move_list(&self) -> MoveList
    {
        let mut move_list = MoveList::empty_move_list();
        let new_en_passant_castling = self.en_passant_castling & (RANKS[0] | RANKS[7]);
        move_list.generate_pawn_moves(&self, new_en_passant_castling, true);
        move_list.generate_piece_moves(&self, KNIGHT, get_attack_mask_knight, new_en_passant_castling, true);
        move_list.generate_piece_moves(&self, BISHOP, get_attack_mask_bishop, new_en_passant_castling, true);
        move_list.generate_piece_moves(&self, ROOK, get_attack_mask_rook, new_en_passant_castling, true);
        move_list.generate_piece_moves(&self, QUEEN, get_attack_mask_queen, new_en_passant_castling, true);
        move_list.generate_piece_moves(&self, KING, get_attack_mask_king, new_en_passant_castling, true);
        move_list
    }
    pub fn is_check(&self, us: Player, enemy: Player, kings_index: usize) -> bool
    {
        let occupancy = self.players[WHITE] | self.players[BLACK];
        //QUEEN
        if get_attack_mask_queen(kings_index, occupancy) & self.pieces[QUEEN] & self.players[enemy] != 0
        {
            return true;
        }
        //KNIGHT
        if get_attack_mask_knight(kings_index, occupancy) & self.pieces[KNIGHT] & self.players[enemy] != 0
        {
            return true;
        }
        //BISHOP
        if get_attack_mask_bishop(kings_index, occupancy) & self.pieces[BISHOP] & self.players[enemy] != 0
        {
            return true;
        }
        //ROOK
        if get_attack_mask_rook(kings_index, occupancy) & self.pieces[ROOK] & self.players[enemy] != 0
        {
            return true;
        }
        //KING
        if get_attack_mask_king(kings_index, occupancy) & self.pieces[KING] & self.players[enemy] != 0
        {
            return true;
        }
        //PAWN
        if PAWN_CAPTURE_ATTACK_TABLE[us][kings_index] & self.pieces[PAWN] & self.players[enemy] != 0
        {
            return true;
        }
        false
    }
    pub fn is_check_unkown_kings_index(&self, us: Player, enemy: Player) -> bool
    {
        let kings_index = (self.pieces[KING] & self.players[us]).trailing_zeros() as usize;
        if kings_index == 64
        {
            return true;
        }
        self.is_check(us, enemy, kings_index)
    }
    pub fn make_move(&mut self, m: &Move)
    {
        let enemy = self.enemy;
        let us = self.us;
        self.en_passant_castling = m.en_passant_castling;
        //en passant
        if m.captured_en_passant
        {
            self.remove_piece(enemy, PAWN, PAWN_QUIET_ATTACK_TABLE[enemy][m.to]);
            self.move_piece(us, PAWN, BIT_AT_INDEX[m.from], BIT_AT_INDEX[m.to]);
        }
        //castling
        else if m.castled
        {
            //IF QUEENSIDE
            if m.to == CASTLING_QUEENSIDE_KING_TO_INDEX[us]
            {
                self.move_piece(us, KING, CASTLING_KING_FROM[us], CASTLING_QUEENSIDE_KING_TO[us]);
                self.move_piece(us, ROOK, CASTLING_QUEENSIDE_ROOK_FROM[us], CASTLING_QUEENSIDE_ROOK_TO[us]);
            }
            //IF KINGSIDE
            else
            {
                self.move_piece(us, KING, CASTLING_KING_FROM[us], CASTLING_KINGSIDE_KING_TO[us]);
                self.move_piece(us, ROOK, CASTLING_KINGSIDE_ROOK_FROM[us], CASTLING_KINGSIDE_ROOK_TO[us]);
            }
        }
        else
        {
            if m.captured != NO_PIECE
            {
                self.remove_piece(enemy, m.captured, BIT_AT_INDEX[m.to]);
            }
            if m.promoted == NO_PIECE
            {
                self.move_piece(us, m.moved, BIT_AT_INDEX[m.from], BIT_AT_INDEX[m.to]);
            }
            else
            {
                self.remove_piece(us, m.moved, BIT_AT_INDEX[m.from]);
                self.add_piece(us, m.promoted, BIT_AT_INDEX[m.to]);
            }
        }
        if self.us == BLACK
        {
            self.fullmoves_played += 1;
        }
        let temp = self.us;
        self.us = self.enemy;
        self.enemy = temp;
        self.zobrist_key = m.zobrist_key;
    }
    pub fn get_all_pseudo_legal_mov_string(&mut self) -> String
    {
        let mut ret = "".to_string();
        let ml = self.generate_move_list();
        for i in 0..ml.len
        {
            let mut next_p = self.clone();
            next_p.make_move(&ml[i]);
            ret += "------------------------------------------------\n";
            ret += &next_p.get_chess_board_string()[..];
            ret += "\n";
            ret += &next_p.get_fen_string()[..];
            ret += "\n";
        }
        ret+= &ml.len.to_string()[..];
        ret += " pseudo-legal mov.\n";
        ret
    }
}
