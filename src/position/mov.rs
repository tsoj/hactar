
use chess_data::*;
use evaluation::score::{SCORE, Score};
use search::transposition_table::TranspositionTable;
use position::piece::{NO_PIECE, PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING, Piece};
use position::piece::get_unicode;
use position::piece::get_notation;
use position::get_bitboard_string;
use position::Position;
use position::player::{WHITE, BLACK};

use std::ops::{Index,IndexMut};
use std::cmp::Ordering;

#[derive(Copy, Clone)]
pub struct Move
{
    pub from: usize,
    pub to: usize,
    pub moved: Piece,
    pub captured: Piece,
    pub promoted: Piece,
    pub en_passant_castling: u64,
    pub zobrist_key: u64,
    pub castled: bool,
    pub captured_en_passant: bool,
    pub score: Score
}
impl PartialOrd for Move
{
    #[inline(always)]
    fn partial_cmp(&self, other: &Move) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Move
{
    #[inline(always)]
    fn eq(&self, other: &Move) -> bool {
        self.score == other.score
    }
}
impl Eq for Move {}
impl Ord for Move
{
    #[inline(always)]
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
            moved: NO_PIECE,
            captured: NO_PIECE,
            promoted: NO_PIECE,
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
        ret += get_field_notation(self.from);
        ret += "\n\tTO: ";
        ret += get_field_notation(self.to);
        ret += "\n\tMOVED: ";
        ret += get_unicode(self.moved);
        ret += "\n\tCAPTURED: ";
        ret += get_unicode(self.captured);
        ret += "\n\tPROMOTED: ";
        ret += get_unicode(self.promoted);
        ret += "\n\tCASTLED: ";
        ret += &self.castled.to_string()[..];
        ret += "\n\tCAPURED EN PASSANT: ";
        ret += &self.captured_en_passant.to_string()[..];
        ret += "\n\tZOBRIST KEY: ";
        ret += &format!("{:x}",self.zobrist_key)[..];
        ret += "\n\tMOVE: CASTLING / EN PASSANT:\n";
        ret += &get_bitboard_string(self.en_passant_castling)[..];
        ret += "--------------------------------------------------\n";
        ret
    }
    pub fn get_move_notation(&self) -> String
    {
        let mut ret = "".to_string();
        ret += get_field_notation(self.from);
        ret += get_field_notation(self.to);
        if self.promoted != NO_PIECE
        {
            ret+= get_notation(self.promoted);
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
    #[inline(always)]
    fn index<'a>(&'a self, index: usize) -> &'a  Move {
        &self.a[index]
    }
}
impl IndexMut<usize> for MoveList
{
    #[inline(always)]
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Move {
        &mut self.a[index]
    }
}
impl MoveList
{
    pub fn empty_move_list() -> MoveList
    {
        MoveList{len: 0, a: unsafe{::std::mem::uninitialized()}}
    }
    pub fn add_move(
        &mut self,
        from: usize,
        to: usize,
        moved: Piece,
        captured: Piece,
        promoted: Piece,
        en_passant_castling: u64,
        castled: bool,
        captured_en_passant: bool,
        orig_position: &Position
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
            zobrist_key: 0,
            castled: castled,
            captured_en_passant: captured_en_passant,
            score: 0
        };
        self[move_list_length].zobrist_key = orig_position.get_updated_zobristkey(&self[move_list_length]);
        self.len+=1;
    }
    pub fn generate_pawn_moves(&mut self, orig_position: &Position, new_en_passant_castling: u64, only_captures: bool)
    {
        let enemy = orig_position.enemy;
        let us = orig_position.us;
        let mut pawn_occupancy = orig_position.pieces[PAWN] & orig_position.players[us];
        let occupancy = orig_position.players[WHITE] | orig_position.players[BLACK];
        if pawn_occupancy != 0
        {
            loop
            {
                let from = find_and_clear_trailing_one(&mut pawn_occupancy) as usize;

                if PAWN_QUIET_ATTACK_TABLE[us][from] & occupancy == 0
                {
                    let to = PAWN_QUIET_ATTACK_TABLE[us][from].trailing_zeros() as usize;
                    if BIT_AT_INDEX[to] & HOME_RANK[enemy] != 0
                    {
                        self.add_move(from, to, PAWN, NO_PIECE, KNIGHT, new_en_passant_castling, false, false, &orig_position);
                        self.add_move(from, to, PAWN, NO_PIECE, BISHOP, new_en_passant_castling, false, false, &orig_position);
                        self.add_move(from, to, PAWN, NO_PIECE, ROOK, new_en_passant_castling, false, false, &orig_position);
                        self.add_move(from, to, PAWN, NO_PIECE, QUEEN, new_en_passant_castling, false, false, &orig_position);
                    }
                    else if !only_captures
                    {
                        self.add_move(from, to, PAWN, NO_PIECE, NO_PIECE, new_en_passant_castling, false, false, &orig_position);
                        if BIT_AT_INDEX[from] & PAWN_HOME_RANK[us] != 0
                        {
                            let double_push_to = PAWN_QUIET_ATTACK_TABLE[us][to].trailing_zeros() as usize;
                            if BIT_AT_INDEX[double_push_to] & occupancy == 0
                            {
                                self.add_move(
                                    from,
                                    double_push_to,
                                    PAWN,
                                    NO_PIECE,
                                    NO_PIECE,
                                    new_en_passant_castling | BIT_AT_INDEX[to],
                                    false,
                                    false,
                                    &orig_position
                                );
                            }
                        }
                    }
                }
                let mut capture_attack_mask = PAWN_CAPTURE_ATTACK_TABLE[us][from] & orig_position.players[enemy];
                if capture_attack_mask != 0
                {
                    loop
                    {
                        let to = find_and_clear_trailing_one(&mut capture_attack_mask);
                        for i in 0..NO_PIECE
                        {
                            if (orig_position.pieces[i] & BIT_AT_INDEX[to]) != 0
                            {
                                if BIT_AT_INDEX[to] & HOME_RANK[enemy] != 0
                                {
                                    let n_new_en_passant_castling = new_en_passant_castling & !BIT_AT_INDEX[to];
                                    self.add_move(from, to, PAWN, i as Piece, KNIGHT, n_new_en_passant_castling, false, false, &orig_position);
                                    self.add_move(from, to, PAWN, i as Piece, BISHOP, n_new_en_passant_castling, false, false, &orig_position);
                                    self.add_move(from, to, PAWN, i as Piece, ROOK, n_new_en_passant_castling, false, false, &orig_position);
                                    self.add_move(from, to, PAWN, i as Piece, QUEEN, n_new_en_passant_castling, false, false, &orig_position);
                                }
                                else
                                {
                                    self.add_move(from, to, PAWN, i as Piece, NO_PIECE, new_en_passant_castling, false, false, &orig_position);
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
                capture_attack_mask = PAWN_CAPTURE_ATTACK_TABLE[us][from] & orig_position.en_passant_castling & (RANKS[2] | RANKS[5]);
                if capture_attack_mask != 0
                {
                    let to = capture_attack_mask.trailing_zeros() as usize;
                    self.add_move(from, to, PAWN, PAWN, NO_PIECE, new_en_passant_castling, false, true, &orig_position);
                }

                if pawn_occupancy == 0
                {
                    break;
                }
            }
        }
    }
    pub fn generate_castling_moves(&mut self, orig_position: &Position, new_en_passant_castling: u64)
    {
        let enemy = orig_position.enemy;
        let us = orig_position.us;
        if orig_position.en_passant_castling & CASTLING_KING_FROM[us] != 0
        {
            let occupancy = orig_position.players[WHITE] | orig_position.players[BLACK];
            //QUEENSIDE CASTLING
            if
            orig_position.en_passant_castling & CASTLING_QUEENSIDE_ROOK_FROM[us] != 0 &&
            CASTLING_QUEENSIDE_BLOCK_RELEVANT_AREA[us] & occupancy == 0 &&
            !orig_position.is_check(us, enemy, CASTLING_QUEENSIDE_CHECK_RELEVANT_FIELDS[us][0]) &&
            !orig_position.is_check(us, enemy, CASTLING_QUEENSIDE_CHECK_RELEVANT_FIELDS[us][1])
            {
                self.add_move(
                    CASTLING_KING_FROM_INDEX[us],
                    CASTLING_QUEENSIDE_KING_TO_INDEX[us],
                    KING,
                    NO_PIECE,
                    NO_PIECE,
                    new_en_passant_castling & !(CASTLING_KING_FROM[us] | CASTLING_QUEENSIDE_ROOK_FROM[us]),
                    true,
                    false,
                    &orig_position
                );
            }
            //KINGSIDE CASTLING
            if
            orig_position.en_passant_castling & CASTLING_KINGSIDE_ROOK_FROM[us] != 0 &&
            CASTLING_KINGSIDE_BLOCK_RELEVANT_AREA[us] & occupancy == 0 &&
            !orig_position.is_check(us, enemy, CASTLING_KINGSIDE_CHECK_RELEVANT_FIELDS[us][0]) &&
            !orig_position.is_check(us, enemy, CASTLING_KINGSIDE_CHECK_RELEVANT_FIELDS[us][1])
            {
                self.add_move(
                    CASTLING_KING_FROM_INDEX[us],
                    CASTLING_KINGSIDE_KING_TO_INDEX[us],
                    KING,
                    NO_PIECE,
                    NO_PIECE,
                    new_en_passant_castling & !(CASTLING_KING_FROM[us] | CASTLING_KINGSIDE_ROOK_FROM[us]),
                    true,
                    false,
                    &orig_position
                );
            }

        }
    }
    pub fn generate_piece_moves<F>(
        &mut self,
        orig_position: &Position,
        piece: Piece,
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
                let from = find_and_clear_trailing_one(&mut piece_occupancy);

                let occupancy = orig_position.players[WHITE] | orig_position.players[BLACK];
                let mut quiet_attack_mask = get_attack_mask(from, occupancy);
                let mut capture_attack_mask = quiet_attack_mask & orig_position.players[enemy];
                quiet_attack_mask &= !capture_attack_mask;
                quiet_attack_mask &= !orig_position.players[us];
                if quiet_attack_mask != 0 && !only_captures
                {
                    loop
                    {
                        let to = find_and_clear_trailing_one(&mut quiet_attack_mask);
                        let n_new_en_passant_castling = new_en_passant_castling & !BIT_AT_INDEX[from];
                        self.add_move(from, to, piece, NO_PIECE, NO_PIECE, n_new_en_passant_castling, false, false, &orig_position);

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
                        for i in 0..NO_PIECE
                        {
                            if (orig_position.pieces[i] & BIT_AT_INDEX[to]) != 0
                            {
                                let n_new_en_passant_castling = new_en_passant_castling & !(BIT_AT_INDEX[to] | BIT_AT_INDEX[from]);
                                self.add_move(from, to, piece, i as Piece, NO_PIECE, n_new_en_passant_castling, false, false, &orig_position);
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
    pub fn sort_moves(&mut self, transposition_table: &TranspositionTable, pv_move: &Move)
    {
        for i in 0..self.len
        {
            if pv_move.to == self[i].to && pv_move.from == self[i].from
            {
                self[i].score = 10000;
                continue;
            }
            self[i].score = 0;
            /*MVV-LVA*/
            self[i].score += SCORE[self[i].promoted];
            if self[i].captured != NO_PIECE
            {
                self[i].score += SCORE[self[i].captured];
                self[i].score -= SCORE[self[i].moved]/8;
            }
            /*Transposition-Table, fail-high first*/
            if transposition_table.failed_high(self[i].zobrist_key)
            {
                self[i].score = 1000;
            }
        }
        &self.a[0..self.len].sort_unstable_by(|a, b| b.cmp(&a));
    }
    pub fn sort_moves_quiesce(&mut self)
    {
        for i in 0..self.len
        {
            self[i].score = 0;
            /*MVV-LVA*/
            self[i].score += SCORE[self[i].promoted];
            if self[i].captured != NO_PIECE
            {
                self[i].score += SCORE[self[i].captured];
                self[i].score -= SCORE[self[i].moved]/8;
            }
        }
        &self.a[0..self.len].sort_unstable_by(|a ,b| b.cmp(&a));
    }
}
