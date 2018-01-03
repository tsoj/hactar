
use std::ops::{Index,IndexMut};
use std::cmp::Ordering;
use evaluation;
use position;
use chess_data;
use search::transposition_table;

#[derive(Copy, Clone)]
pub struct Move
{
    pub from: usize,
    pub to: usize,
    pub moved: position::piece::Piece,
    pub captured: position::piece::Piece,
    pub promoted: position::piece::Piece,
    pub en_passant_castling: u64,
    pub zobrist_key: u64,
    pub castled: bool,
    pub captured_en_passant: bool,
    pub score: evaluation::score::Score
}
impl PartialOrd for Move
{
    fn partial_cmp(&self, other: &Move) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Move
{
    fn eq(&self, other: &Move) -> bool {
        self.score == other.score
    }
}
impl Eq for Move {}
impl Ord for Move
{
    fn cmp(&self, other: &Move) -> Ordering {
        self.score.cmp(&other.score)
    }
}
impl Move
{
    pub fn empty_move() -> Move
    {
        Move
        {
            from: 0,
            to: 0,
            moved: position::piece::NO_PIECE,
            captured: position::piece::NO_PIECE,
            promoted: position::piece::NO_PIECE,
            en_passant_castling: 0,
            zobrist_key: 0,
            castled: false,
            captured_en_passant: false,
            score: 0
        }
    }
    pub fn get_data_string(&self) -> String
    {
        let mut ret = "".to_string();
        ret += "Move:\n";
        ret += "--------------------------------------------------";
        ret += "\n\tFROM: ";
        ret += chess_data::get_field_notation(self.from);
        ret += "\n\tTO: ";
        ret += chess_data::get_field_notation(self.to);
        ret += "\n\tMOVED: ";
        ret += position::piece::get_unicode(self.moved);
        ret += "\n\tCAPTURED: ";
        ret += position::piece::get_unicode(self.captured);
        ret += "\n\tPROMOTED: ";
        ret += position::piece::get_unicode(self.promoted);
        ret += "\n\tCASTLED: ";
        ret += &self.castled.to_string()[..];
        ret += "\n\tCAPURED EN PASSANT: ";
        ret += &self.captured_en_passant.to_string()[..];
        ret += "\n\tZOBRIST KEY: ";
        ret += &format!("{:x}",self.zobrist_key)[..];
        ret += "\n\tMOVE: CASTLING / EN PASSANT:\n";
        ret += &position::get_bitboard_string(self.en_passant_castling)[..];
        ret += "--------------------------------------------------\n";
        ret
    }
    pub fn get_move_notation(&self) -> String
    {
        let mut ret = "".to_string();
        ret += chess_data::get_field_notation(self.from);
        ret += chess_data::get_field_notation(self.to);
        if self.promoted != position::piece::NO_PIECE
        {
            ret+= position::piece::get_notation(self.promoted);
        }
        ret
    }
}

const MOVE_LIST_MAXIMUM_LENGTH: usize = 100;//TODO: needs some testing
#[derive(Copy, Clone)]
pub struct MoveList
{
    pub len: usize,
    pub a: [Move; MOVE_LIST_MAXIMUM_LENGTH]
}
impl Index<usize> for MoveList
{
    type Output = Move;
    fn index<'a>(&'a self, index: usize) -> &'a  Move {
        &self.a[index]
    }
}
impl IndexMut<usize> for MoveList
{
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
    pub fn add_move(
        &mut self,
        from: usize,
        to: usize,
        moved: position::piece::Piece,
        captured: position::piece::Piece,
        promoted: position::piece::Piece,
        en_passant_castling: u64,
        zobrist_key: u64,
        castled: bool,
        captured_en_passant: bool,
        orig_position: &position::Position
    )
    {
        let move_list_length = self.len;
        self[move_list_length] = Move
        {
            from: from,
            to: to,
            moved: moved,
            captured: captured,
            promoted: promoted,
            en_passant_castling: en_passant_castling,
            zobrist_key: zobrist_key,
            castled: castled,
            captured_en_passant: captured_en_passant,
            score: 0
        };
        self[move_list_length].zobrist_key = orig_position.get_updated_zobristkey(&self[move_list_length]);
        self.len+=1;
    }
    pub fn generate_pawn_moves(&mut self, orig_position: &position::Position, new_en_passant_castling: u64, only_captures: bool)
    {
        let enemy = orig_position.enemy;
        let us = orig_position.us;
        let mut pawn_occupancy = orig_position.pieces[position::piece::PAWN] & orig_position.players[us];
        let occupancy = orig_position.players[position::player::WHITE] | orig_position.players[position::player::BLACK];
        if pawn_occupancy != 0
        {
            loop
            {
                let from = chess_data::find_and_clear_trailing_one(&mut pawn_occupancy) as usize;

                if chess_data::PAWN_QUIET_ATTACK_TABLE[us][from] & occupancy == 0
                {
                    let to = chess_data::PAWN_QUIET_ATTACK_TABLE[us][from].trailing_zeros() as usize;
                    if chess_data::BIT_AT_INDEX[to] & chess_data::HOME_RANK[enemy] != 0
                    {
                        self.add_move(from, to, position::piece::PAWN, position::piece::NO_PIECE, position::piece::KNIGHT, new_en_passant_castling, 0, false, false, &orig_position);
                        self.add_move(from, to, position::piece::PAWN, position::piece::NO_PIECE, position::piece::BISHOP, new_en_passant_castling, 0, false, false, &orig_position);
                        self.add_move(from, to, position::piece::PAWN, position::piece::NO_PIECE, position::piece::ROOK, new_en_passant_castling, 0, false, false, &orig_position);
                        self.add_move(from, to, position::piece::PAWN, position::piece::NO_PIECE, position::piece::QUEEN, new_en_passant_castling, 0, false, false, &orig_position);
                    }
                    else if !only_captures
                    {
                        self.add_move(from, to, position::piece::PAWN, position::piece::NO_PIECE, position::piece::NO_PIECE, new_en_passant_castling, 0, false, false, &orig_position);
                        if chess_data::BIT_AT_INDEX[from] & chess_data::PAWN_HOME_RANK[us] != 0
                        {
                            let double_push_to = chess_data::PAWN_QUIET_ATTACK_TABLE[us][to].trailing_zeros() as usize;
                            if chess_data::BIT_AT_INDEX[double_push_to] & occupancy == 0
                            {
                                self.add_move(
                                    from,
                                    double_push_to,
                                    position::piece::PAWN,
                                    position::piece::NO_PIECE,
                                    position::piece::NO_PIECE,
                                    new_en_passant_castling | chess_data::BIT_AT_INDEX[to],
                                    0,
                                    false,
                                    false,
                                    &orig_position
                                );
                            }
                        }
                    }
                }
                let mut capture_attack_mask = chess_data::PAWN_CAPTURE_ATTACK_TABLE[us][from] & orig_position.players[enemy];
                if capture_attack_mask != 0
                {
                    loop
                    {
                        let to = chess_data::find_and_clear_trailing_one(&mut capture_attack_mask);
                        for i in 0..position::piece::NO_PIECE
                        {
                            if (orig_position.pieces[i] & chess_data::BIT_AT_INDEX[to]) != 0
                            {
                                if chess_data::BIT_AT_INDEX[to] & chess_data::HOME_RANK[enemy] != 0
                                {
                                    let n_new_en_passant_castling = new_en_passant_castling & !chess_data::BIT_AT_INDEX[to];
                                    self.add_move(from, to, position::piece::PAWN, i as position::piece::Piece, position::piece::KNIGHT, n_new_en_passant_castling, 0, false, false, &orig_position);
                                    self.add_move(from, to, position::piece::PAWN, i as position::piece::Piece, position::piece::BISHOP, n_new_en_passant_castling, 0, false, false, &orig_position);
                                    self.add_move(from, to, position::piece::PAWN, i as position::piece::Piece, position::piece::ROOK, n_new_en_passant_castling, 0, false, false, &orig_position);
                                    self.add_move(from, to, position::piece::PAWN, i as position::piece::Piece, position::piece::QUEEN, n_new_en_passant_castling, 0, false, false, &orig_position);
                                }
                                else
                                {
                                    self.add_move(from, to, position::piece::PAWN, i as position::piece::Piece, position::piece::NO_PIECE, new_en_passant_castling, 0, false, false, &orig_position);
                                }
                                break;
                            }
                        }

                        if capture_attack_mask == 0
                        {
                            break;
                        }
                    }
                }
                capture_attack_mask = chess_data::PAWN_CAPTURE_ATTACK_TABLE[us][from] & orig_position.en_passant_castling & (chess_data::RANKS[2] | chess_data::RANKS[5]);
                if capture_attack_mask != 0
                {
                    let to = capture_attack_mask.trailing_zeros() as usize;
                    self.add_move(from, to, position::piece::PAWN, position::piece::PAWN, position::piece::NO_PIECE, new_en_passant_castling, 0, false, true, &orig_position);
                }

                if pawn_occupancy == 0
                {
                    break;
                }
            }
        }
    }
    pub fn generate_castling_moves(&mut self, orig_position: &position::Position, new_en_passant_castling: u64)
    {
        let enemy = orig_position.enemy;
        let us = orig_position.us;
        if orig_position.en_passant_castling & chess_data::CASTLING_KING_FROM[us] != 0
        {
            let occupancy = orig_position.players[position::player::WHITE] | orig_position.players[position::player::BLACK];
            //QUEENSIDE CASTLING
            if
            orig_position.en_passant_castling & chess_data::CASTLING_QUEENSIDE_ROOK_FROM[us] != 0 &&
            chess_data::CASTLING_QUEENSIDE_BLOCK_RELEVANT_AREA[us] & occupancy == 0 &&
            !orig_position.is_check(us, enemy, chess_data::CASTLING_QUEENSIDE_CHECK_RELEVANT_FIELDS[us][0]) &&
            !orig_position.is_check(us, enemy, chess_data::CASTLING_QUEENSIDE_CHECK_RELEVANT_FIELDS[us][1])
            {
                self.add_move(
                    chess_data::CASTLING_KING_FROM_INDEX[us],
                    chess_data::CASTLING_QUEENSIDE_KING_TO_INDEX[us],
                    position::piece::KING,
                    position::piece::NO_PIECE,
                    position::piece::NO_PIECE,
                    new_en_passant_castling & !(chess_data::CASTLING_KING_FROM[us] | chess_data::CASTLING_QUEENSIDE_ROOK_FROM[us]),
                    0,
                    true,
                    false,
                    &orig_position
                );
            }
            //KINGSIDE CASTLING
            if
            orig_position.en_passant_castling & chess_data::CASTLING_KINGSIDE_ROOK_FROM[us] != 0 &&
            chess_data::CASTLING_KINGSIDE_BLOCK_RELEVANT_AREA[us] & occupancy == 0 &&
            !orig_position.is_check(us, enemy, chess_data::CASTLING_KINGSIDE_CHECK_RELEVANT_FIELDS[us][0]) &&
            !orig_position.is_check(us, enemy, chess_data::CASTLING_KINGSIDE_CHECK_RELEVANT_FIELDS[us][1])
            {
                self.add_move(
                    chess_data::CASTLING_KING_FROM_INDEX[us],
                    chess_data::CASTLING_KINGSIDE_KING_TO_INDEX[us],
                    position::piece::KING,
                    position::piece::NO_PIECE,
                    position::piece::NO_PIECE,
                    new_en_passant_castling & !(chess_data::CASTLING_KING_FROM[us] | chess_data::CASTLING_KINGSIDE_ROOK_FROM[us]),
                    0,
                    true,
                    false,
                    &orig_position
                );
            }

        }
    }
    pub fn generate_piece_moves<F>(
        &mut self,
        orig_position: &position::Position,
        piece: position::piece::Piece,
        get_attack_mask: F,
        new_en_passant_castling: u64,
        only_captures: bool
    )
    where F: Fn(usize, u64) -> u64
    {
        let enemy = orig_position.enemy;
        let us = orig_position.us;
        let mut piece_occupancy = orig_position.pieces[piece] & orig_position.players[us];
        if piece_occupancy != 0
        {
            loop
            {
                let from = chess_data::find_and_clear_trailing_one(&mut piece_occupancy);

                let occupancy = orig_position.players[position::player::WHITE] | orig_position.players[position::player::BLACK];
                let mut quiet_attack_mask = get_attack_mask(from, occupancy);
                let mut capture_attack_mask = quiet_attack_mask & orig_position.players[enemy];
                quiet_attack_mask &= !capture_attack_mask;
                quiet_attack_mask &= !orig_position.players[us];
                if quiet_attack_mask != 0 && !only_captures
                {
                    loop
                    {
                        let to = chess_data::find_and_clear_trailing_one(&mut quiet_attack_mask);
                        let n_new_en_passant_castling = new_en_passant_castling & !chess_data::BIT_AT_INDEX[from];
                        self.add_move(from, to, piece, position::piece::NO_PIECE, position::piece::NO_PIECE, n_new_en_passant_castling, 0, false, false, &orig_position);

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
                        let to = chess_data::find_and_clear_trailing_one(&mut capture_attack_mask);
                        for i in 0..position::piece::NO_PIECE
                        {
                            if (orig_position.pieces[i] & chess_data::BIT_AT_INDEX[to]) != 0
                            {
                                let n_new_en_passant_castling = new_en_passant_castling & !(chess_data::BIT_AT_INDEX[to] | chess_data::BIT_AT_INDEX[from]);
                                self.add_move(from, to, piece, i as position::piece::Piece, position::piece::NO_PIECE, n_new_en_passant_castling, 0, false, false, &orig_position);
                                break;
                            }
                        }

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
    pub fn sort_moves(&mut self, transposition_table: &transposition_table::TranspositionTable, pv_move: &Move)
    {
        for i in 0..self.len
        {
            if pv_move.to == self[i].to && pv_move.from == self[i].from && pv_move.captured == self[i].captured && pv_move.moved == self[i].moved
            {
                self[i].score = 10000;
                continue;
            }
            self[i].score = 0;
            /*MVV-LVA*/
            self[i].score += evaluation::score::SCORE[self[i].promoted];
            if self[i].captured != position::piece::NO_PIECE
            {
                self[i].score += evaluation::score::SCORE[self[i].captured];
                self[i].score -= evaluation::score::SCORE[self[i].moved]/8;
            }
            /*Transposition-Table, fail-high first*/
            if transposition_table.failed_high(self[i].zobrist_key)
            {
                self[i].score = 1000;
            }
        }
        &self.a[0..self.len].sort_unstable_by(|a ,b| b.cmp(&a));
    }
    pub fn sort_moves_quiesce(&mut self)
    {
        for i in 0..self.len
        {
            self[i].score = 0;
            /*MVV-LVA*/
            self[i].score += evaluation::score::SCORE[self[i].promoted];
            if self[i].captured != position::piece::NO_PIECE
            {
                self[i].score += evaluation::score::SCORE[self[i].captured];
                self[i].score -= evaluation::score::SCORE[self[i].moved]/8;
            }
        }
        &self.a[0..self.len].sort_unstable_by(|a ,b| b.cmp(&a));
    }
}
