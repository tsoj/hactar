#![allow(dead_code)]
use chess_data;
pub mod piece;
pub mod player;
pub mod mov;

pub const START_POS_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

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

#[derive(Copy, Clone)]
pub struct Position
{
    pub pieces: [u64; 6], //[Pawns, Knights, Bishops, Rooks, Queens, Kings]
    pub players: [u64; 2], //[White pieces, Black pieces]
    pub en_passant_castling: u64,
    pub zobrist_key: u64,
    pub whose_move: player::Player,
    pub last_move: mov::Move,
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
            mov::Move::empty_move(),
            fullmoves_played: 0,
            halfmove_clock: 0
        }
    }
    pub fn add_piece(&mut self, player: player::Player, piece: piece::Piece , field: u64)
    {
        self.pieces[piece] |=  field;
        self.players[player] |=  field;
    }
    pub fn remove_piece(&mut self, player: player::Player, piece: piece::Piece , field: u64)
    {
        self.pieces[piece] &=  !field;
        self.players[player] &=  !field;
    }
    pub fn move_piece(&mut self, player: player::Player, piece: piece::Piece , from: u64,  to: u64)
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
                if (self.pieces[piece::PAWN] & chess_data::BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = chess_data::BLACK_PAWN_UNICODE.to_string();
                }
                else if (self.pieces[piece::KNIGHT] & chess_data::BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = chess_data::BLACK_KNIGHT_UNICODE.to_string();
                }
                else if (self.pieces[piece::BISHOP] & chess_data::BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = chess_data::BLACK_BISHOP_UNICODE.to_string();
                }
                else if (self.pieces[piece::ROOK] & chess_data::BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = chess_data::BLACK_ROOK_UNICODE.to_string();
                }
                else if (self.pieces[piece::QUEEN] & chess_data::BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = chess_data::BLACK_QUEEN_UNICODE.to_string();
                }
                else if (self.pieces[piece::KING] & chess_data::BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = chess_data::BLACK_KING_UNICODE.to_string();
                }
            }
            else if (self.players[player::WHITE] & chess_data::BIT_AT_INDEX[i]) != 0
            {
                if (self.pieces[piece::PAWN] & chess_data::BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = chess_data::WHITE_PAWN_UNICODE.to_string();
                }
                else if (self.pieces[piece::KNIGHT] & chess_data::BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = chess_data::WHITE_KNIGHT_UNICODE.to_string();
                }
                else if (self.pieces[piece::BISHOP] & chess_data::BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = chess_data::WHITE_BISHOP_UNICODE.to_string();
                }
                else if (self.pieces[piece::ROOK] & chess_data::BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = chess_data::WHITE_ROOK_UNICODE.to_string();
                }
                else if (self.pieces[piece::QUEEN] & chess_data::BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = chess_data::WHITE_QUEEN_UNICODE.to_string();
                }
                else if (self.pieces[piece::KING] & chess_data::BIT_AT_INDEX[i]) != 0
                {
                    temp[i] = chess_data::WHITE_KING_UNICODE.to_string();
                }
            }
        }
        let mut s = format_for_chess_board(&temp);
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
        ret += &get_bitboard_string(self.pieces[piece::PAWN])[..];
        ret += "KNIGHTS:\n";
        ret += &get_bitboard_string(self.pieces[piece::KNIGHT])[..];
        ret += "BISHOPS:\n";
        ret += &get_bitboard_string(self.pieces[piece::BISHOP])[..];
        ret += "ROOKS:\n";
        ret += &get_bitboard_string(self.pieces[piece::ROOK])[..];
        ret += "QUEENS:\n";
        ret += &get_bitboard_string(self.pieces[piece::QUEEN])[..];
        ret += "KINGS:\n";
        ret += &get_bitboard_string(self.pieces[piece::KING])[..];
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
                    p.add_piece(player::WHITE, piece::PAWN, chess_data::BIT_AT_INDEX[field_counter]);
                    field_counter+=1;
                },
                'N' =>
                {
                    p.add_piece(player::WHITE, piece::KNIGHT, chess_data::BIT_AT_INDEX[field_counter]);
                    field_counter+=1;
                },
                'B' =>
                {
                    p.add_piece(player::WHITE, piece::BISHOP, chess_data::BIT_AT_INDEX[field_counter]);
                    field_counter+=1;
                },
                'R' =>
                {
                    p.add_piece(player::WHITE, piece::ROOK, chess_data::BIT_AT_INDEX[field_counter]);
                    field_counter+=1;
                },
                'Q' =>
                {
                    p.add_piece(player::WHITE, piece::QUEEN, chess_data::BIT_AT_INDEX[field_counter]);
                    field_counter+=1;
                },
                'K' =>
                {
                    p.add_piece(player::WHITE, piece::KING, chess_data::BIT_AT_INDEX[field_counter]);
                    field_counter+=1;
                },
                'p' =>
                {
                    p.add_piece(player::BLACK, piece::PAWN, chess_data::BIT_AT_INDEX[field_counter]);
                    field_counter+=1;
                },
                'n' =>
                {
                    p.add_piece(player::BLACK, piece::KNIGHT, chess_data::BIT_AT_INDEX[field_counter]);
                    field_counter+=1;
                },
                'b' =>
                {
                    p.add_piece(player::BLACK, piece::BISHOP, chess_data::BIT_AT_INDEX[field_counter]);
                    field_counter+=1;
                },
                'r' =>
                {
                    p.add_piece(player::BLACK, piece::ROOK, chess_data::BIT_AT_INDEX[field_counter]);
                    field_counter+=1;
                },
                'q' =>
                {
                    p.add_piece(player::BLACK, piece::QUEEN, chess_data::BIT_AT_INDEX[field_counter]);
                    field_counter+=1;
                },
                'k' =>
                {
                    p.add_piece(player::BLACK, piece::KING, chess_data::BIT_AT_INDEX[field_counter]);
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
        for i in 0..piece::NO_PIECE
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
    pub fn get_updated_zobristkey(&self, m: &mov::Move, en_passant_castling: u64, us: player::Player, enemy: player::Player) -> u64
    {
        let mut ret: u64 = self.zobrist_key;

        ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PIECES[m.moved][m.from];
        if m.promoted != piece::NO_PIECE
        {
            ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PIECES[m.promoted][m.to];
        }
        else
        {
            ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PIECES[m.moved][m.to];
        }

        ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PLAYERS[us][m.from];
        ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PLAYERS[us][m.to];

        if m.captured != piece::NO_PIECE && !m.captured_en_passant
        {
            ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PIECES[m.captured][m.to];
            ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PLAYERS[enemy][m.to];
        }

        ret ^= en_passant_castling;
        ret ^= m.en_passant_castling;
        ret ^= us as u64;
        ret ^= enemy as u64;

        if m.captured_en_passant
        {
            let captured_index = chess_data::PAWN_QUIET_ATTACK_TABLE[enemy][m.to].trailing_zeros() as usize;
            ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PIECES[piece::PAWN][captured_index];
            ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PLAYERS[enemy][captured_index];
        }

        if m.castled
        {
            //IF QUEENSIDE
            if m.to == chess_data::CASTLING_QUEENSIDE_KING_TO_INDEX[us]
            {
                ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PIECES[piece::ROOK][chess_data::CASTLING_QUEENSIDE_ROOK_FROM_INDEX[us]];
                ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PIECES[piece::ROOK][chess_data::CASTLING_QUEENSIDE_ROOK_TO_INDEX[us]];
                ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PLAYERS[us][chess_data::CASTLING_QUEENSIDE_ROOK_FROM_INDEX[us]];
                ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PLAYERS[us][chess_data::CASTLING_QUEENSIDE_ROOK_TO_INDEX[us]];
            }
            //IF KINGSIDE
            else
            {
                ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PIECES[piece::ROOK][chess_data::CASTLING_KINGSIDE_ROOK_FROM_INDEX[us]];
                ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PIECES[piece::ROOK][chess_data::CASTLING_KINGSIDE_ROOK_TO_INDEX[us]];
                ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PLAYERS[us][chess_data::CASTLING_KINGSIDE_ROOK_FROM_INDEX[us]];
                ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PLAYERS[us][chess_data::CASTLING_KINGSIDE_ROOK_TO_INDEX[us]];
            }
        }
        ret
    }
    pub fn update_zobristkey(&mut self, m: &mov::Move, en_passant_castling: u64, us: player::Player, enemy: player::Player)
    {
        let mut ret: u64 = self.zobrist_key;

        ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PIECES[m.moved][m.from];
        if m.promoted != piece::NO_PIECE
        {
            ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PIECES[m.promoted][m.to];
        }
        else
        {
            ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PIECES[m.moved][m.to];
        }

        ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PLAYERS[us][m.from];
        ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PLAYERS[us][m.to];
        if m.captured != piece::NO_PIECE
        {
            ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PIECES[m.captured][m.to];
            ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PLAYERS[enemy][m.to];
        }

        ret ^= en_passant_castling;
        ret ^= m.en_passant_castling;
        ret ^= us as u64;
        ret ^= enemy as u64;

        if m.captured_en_passant
        {
            let captured_index = chess_data::PAWN_QUIET_ATTACK_TABLE[enemy][m.to].trailing_zeros() as usize;
            if captured_index != 64
            {
                ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PIECES[piece::PAWN][captured_index];
                ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PLAYERS[enemy][captured_index];
            }

        }

        if m.castled
        {
            //IF QUEENSIDE
            if m.en_passant_castling & chess_data::CASTLING_QUEENSIDE_ROOK_FROM[us] == 0
            {
                ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PIECES[piece::ROOK][chess_data::CASTLING_QUEENSIDE_ROOK_FROM_INDEX[us]];
                ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PIECES[piece::ROOK][chess_data::CASTLING_QUEENSIDE_ROOK_TO_INDEX[us]];
                ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PLAYERS[enemy][chess_data::CASTLING_QUEENSIDE_ROOK_FROM_INDEX[us]];
                ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PLAYERS[enemy][chess_data::CASTLING_QUEENSIDE_ROOK_TO_INDEX[us]];
            }
            //IF KINGSIDE
            else
            {
                ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PIECES[piece::ROOK][chess_data::CASTLING_KINGSIDE_ROOK_FROM_INDEX[us]];
                ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PIECES[piece::ROOK][chess_data::CASTLING_KINGSIDE_ROOK_TO_INDEX[us]];
                ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PLAYERS[enemy][chess_data::CASTLING_KINGSIDE_ROOK_FROM_INDEX[us]];
                ret ^= chess_data::ZOBRIST_RANDOM_BITMASKS_PLAYERS[enemy][chess_data::CASTLING_KINGSIDE_ROOK_TO_INDEX[us]];
            }
        }
        self.zobrist_key = ret;
    }
    pub fn generate_move_list(&self, us: player::Player, enemy: player::Player) -> mov::MoveList
    {
        let mut move_list = mov::MoveList::get_empty_move_list();
        let new_en_passant_castling = self.en_passant_castling & (chess_data::RANKS[0] | chess_data::RANKS[7]);
        move_list.generate_pawn_moves(&self, us, enemy, new_en_passant_castling);
        move_list.generate_castling_moves(&self, us, enemy, new_en_passant_castling);
        move_list.generate_piece_moves(&self, us, enemy, piece::KNIGHT, chess_data::get_attack_mask_knight, new_en_passant_castling);
        move_list.generate_piece_moves(&self, us, enemy, piece::BISHOP, chess_data::get_attack_mask_bishop, new_en_passant_castling);
        move_list.generate_piece_moves(&self, us, enemy, piece::ROOK, chess_data::get_attack_mask_rook, new_en_passant_castling);
        move_list.generate_piece_moves(&self, us, enemy, piece::QUEEN, chess_data::get_attack_mask_queen, new_en_passant_castling);
        move_list.generate_piece_moves(&self, us, enemy, piece::KING, chess_data::get_attack_mask_king, new_en_passant_castling);
        move_list
    }
    pub fn is_check(&self, us: player::Player, enemy: player::Player, kings_index: usize) -> bool
    {
        let occupancy = self.players[player::WHITE] | self.players[player::BLACK];
        //QUEEN
        if chess_data::get_attack_mask_queen(kings_index, occupancy) & self.pieces[piece::QUEEN] & self.players[enemy] != 0
        {
            return true;
        }
        //KNIGHT
        if chess_data::get_attack_mask_knight(kings_index, occupancy) & self.pieces[piece::KNIGHT] & self.players[enemy] != 0
        {
            return true;
        }
        //BISHOP
        if chess_data::get_attack_mask_bishop(kings_index, occupancy) & self.pieces[piece::BISHOP] & self.players[enemy] != 0
        {
            return true;
        }
        //ROOK
        if chess_data::get_attack_mask_rook(kings_index, occupancy) & self.pieces[piece::ROOK] & self.players[enemy] != 0
        {
            return true;
        }
        //KING
        if chess_data::get_attack_mask_king(kings_index, occupancy) & self.pieces[piece::KING] & self.players[enemy] != 0
        {
            return true;
        }
        //PAWN
        if chess_data::PAWN_CAPTURE_ATTACK_TABLE[us][kings_index] & self.pieces[piece::PAWN] & self.players[enemy] != 0
        {
            return true;
        }
        false
    }
    pub fn is_check_unkown_kings_index(&self, us: player::Player, enemy: player::Player) -> bool
    {
        let kings_index = (self.pieces[piece::KING] & self.players[us]).trailing_zeros() as usize;
        if kings_index == 64
        {
            return true;
        }
        self.is_check(us, enemy, kings_index)
    }
    pub fn make_move(&mut self, m: &mov::Move, us: player::Player, enemy: player::Player) -> u64
    {
        let backup_en_passant_castling = self.en_passant_castling;
        self.en_passant_castling = m.en_passant_castling;
        //en passant
        if m.captured_en_passant
        {
            self.remove_piece(enemy, piece::PAWN, chess_data::PAWN_QUIET_ATTACK_TABLE[enemy][m.to]);
            self.move_piece(us, piece::PAWN, chess_data::BIT_AT_INDEX[m.from], chess_data::BIT_AT_INDEX[m.to]);
        }
        //castling
        else if m.castled
        {
            //IF QUEENSIDE
            if m.to == chess_data::CASTLING_QUEENSIDE_KING_TO_INDEX[us]
            {
                self.move_piece(us, piece::KING, chess_data::CASTLING_KING_FROM[us], chess_data::CASTLING_QUEENSIDE_KING_TO[us]);
                self.move_piece(us, piece::ROOK, chess_data::CASTLING_QUEENSIDE_ROOK_FROM[us], chess_data::CASTLING_QUEENSIDE_ROOK_TO[us]);
            }
            //IF KINGSIDE
            else
            {
                self.move_piece(us, piece::KING, chess_data::CASTLING_KING_FROM[us], chess_data::CASTLING_KINGSIDE_KING_TO[us]);
                self.move_piece(us, piece::ROOK, chess_data::CASTLING_KINGSIDE_ROOK_FROM[us], chess_data::CASTLING_KINGSIDE_ROOK_TO[us]);
            }
        }
        else
        {
            if m.captured != piece::NO_PIECE
            {
                self.remove_piece(enemy, m.captured, chess_data::BIT_AT_INDEX[m.to]);
            }
            if m.promoted == piece::NO_PIECE
            {
                self.move_piece(us, m.moved, chess_data::BIT_AT_INDEX[m.from], chess_data::BIT_AT_INDEX[m.to]);
            }
            else
            {
                self.remove_piece(us, m.moved, chess_data::BIT_AT_INDEX[m.from]);
                self.add_piece(us, m.promoted, chess_data::BIT_AT_INDEX[m.to]);
            }
        }
        if self.whose_move == player::BLACK
        {
            self.fullmoves_played += 1;
        }
        self.whose_move = player::switch_player(self.whose_move);
        self.zobrist_key = m.zobrist_key;

        backup_en_passant_castling
    }
    pub fn undo_move(&mut self, m: &mov::Move, backup_en_passant_castling: u64, us: player::Player, enemy: player::Player)
    {
        self.en_passant_castling = backup_en_passant_castling;
        //en passant
        if m.captured_en_passant
        {
            self.add_piece(enemy, piece::PAWN, chess_data::PAWN_QUIET_ATTACK_TABLE[enemy][m.to]);
            self.move_piece(us, piece::PAWN, chess_data::BIT_AT_INDEX[m.to], chess_data::BIT_AT_INDEX[m.from]);
        }
        //castling
        else if m.castled
        {
            //IF QUEENSIDE
            if m.to == chess_data::CASTLING_QUEENSIDE_KING_TO_INDEX[us]
            {
                self.move_piece(us, piece::KING, chess_data::CASTLING_QUEENSIDE_KING_TO[us], chess_data::CASTLING_KING_FROM[us]);
                self.move_piece(us, piece::ROOK, chess_data::CASTLING_QUEENSIDE_ROOK_TO[us], chess_data::CASTLING_QUEENSIDE_ROOK_FROM[us]);
            }
            //IF KINGSIDE
            else
            {
                self.move_piece(us, piece::KING, chess_data::CASTLING_KINGSIDE_KING_TO[us], chess_data::CASTLING_KING_FROM[us]);
                self.move_piece(us, piece::ROOK, chess_data::CASTLING_KINGSIDE_ROOK_TO[us], chess_data::CASTLING_KINGSIDE_ROOK_FROM[us]);
            }
        }
        else
        {
            if m.promoted == piece::NO_PIECE
            {
                self.move_piece(us, m.moved, chess_data::BIT_AT_INDEX[m.to], chess_data::BIT_AT_INDEX[m.from]);
            }
            else
            {
                self.remove_piece(us, m.promoted, chess_data::BIT_AT_INDEX[m.to]);
                self.add_piece(us, m.moved, chess_data::BIT_AT_INDEX[m.from]);
            }
            if m.captured != piece::NO_PIECE
            {
                self.add_piece(enemy, m.captured, chess_data::BIT_AT_INDEX[m.to]);
            }
        }
        if self.whose_move == player::BLACK
        {
            self.fullmoves_played -= 1;
        }
        self.whose_move = player::switch_player(self.whose_move);
        self.update_zobristkey(m, backup_en_passant_castling, us, enemy);
    }
    pub fn get_all_pseudo_legal_mov_string(&mut self) -> String
    {
        let mut ret = "".to_string();
        let enemy = player::switch_player(self.whose_move);
        let us = self.whose_move;
        let ml = self.generate_move_list(us, enemy);
        for i in 0..ml.len
        {
            let mut next_p = self.clone();
            next_p.make_move(&ml[i], us, enemy);
            ret += "------------------------------------------------\n";
            ret += &next_p.get_chess_board_string()[..];
            ret += "\n";
        }
        ret+= &ml.len.to_string()[..];
        ret += " pseudo-legal mov.\n";
        ret
    }
}
