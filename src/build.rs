#![allow(dead_code)]

use std::fs::File;
use std::io::Write;
mod chess_data;

const UPPER_LEFT_SIDE_ZERO: u64 =  0b1000000011000000111000001111000011111000111111001111111011111111;
const LOWER_RIGHT_SIDE_ZERO: u64 = 0b1111111101111111001111110001111100001111000001110000001100000001;
const LOWER_LEFT_SIDE_ZERO: u64 = 0b1111111011111100111110001111000011100000110000001000000000000000;
const UPPER_RIGHT_SIDE_ZERO: u64 = 0b0000000000000001000000110000011100001111000111110011111101111111;
const MAIN_DIAGONAL: u64 = 0b1000000001000000001000000001000000001000000001000000001000000001; //A1 to H8
const ANTI_DIAGONAL: u64 = 0b0000000100000010000001000000100000010000001000000100000010000000; //H1 to A8

const NORTH: isize = 8;
const SOUTH: isize = -8;
const EAST: isize = 1;
const WEST: isize = -1;
const NORTH_EAST: isize = 9;
const NORTH_WEST: isize = 7;
const SOUTH_WEST: isize = -9;
const SOUTH_EAST: isize = -7;

fn write_string_to_file(path: &str, s: &String)
{
    let mut f = File::create(path).expect("Unable to create file");
    f.write_all(s.as_bytes()).expect("Unable to write data");
}
fn write_64_array_to_string(a: &[u64; 64]) -> String
{
    let mut data = String::from("[");
    for i in 0..64
    {
        data.push_str(&(a[i]).to_string());
        data.push_str("u64");
        if i == 63
        {
            data.push_str("]");
        }
        else
        {
            data.push(',');
        }
    }
    data
}
fn write_8_array_to_string(a: &[u64; 8]) -> String
{
    let mut data = String::from("[");
    for i in 0..8
    {
        data.push_str(&(a[i]).to_string());
        data.push_str("u64");
        if i == 7
        {
            data.push_str("]");
        }
        else
        {
            data.push(',');
        }
    }
    data
}
fn write_64_64_array_to_string(a: &[[u64; 64]; 64]) -> String
{

    let mut data = String::from("[");
    for i in 0..64
    {
        data.push_str(&write_64_array_to_string(&a[i]));
        if i == 63
        {
            data.push_str("]");
        }
        else
        {
            data.push(',');
        }
    }
    data
}
fn write_2_64_array_to_string(a: &[[u64; 64]; 2]) -> String
{

    let mut data = String::from("[");
    for i in 0..2
    {
        data.push_str(&write_64_array_to_string(&a[i]));
        if i == 1
        {
            data.push_str("]");
        }
        else
        {
            data.push(',');
        }
    }
    data
}
fn write_2_64_64_array_to_string(a: &[[[u64; 64]; 64]; 2]) -> String
{
    let mut data = String::from("[");
    for i in 0..2
    {
        data.push_str(&write_64_64_array_to_string(&a[i]));
        if i == 1
        {
            data.push_str("]");
        }
        else
        {
            data.push(',');
        }
    }
    data
}

fn main() {
    let mut bit_at_index: [u64; 64] = [0; 64];
    for i in 0..bit_at_index.len()
    {
      bit_at_index[i] = 0b1 << i;
    }
    write_string_to_file("./src/chess_data_in/bit_at_index.in", &write_64_array_to_string(&bit_at_index));

    let mut ranks: [u64; 8] = [0; 8];
    for i in 0..ranks.len()
    {
      ranks[i] = 0b11111111 << (i*8);
    }
    write_string_to_file("./src/chess_data_in/ranks.in", &write_8_array_to_string(&ranks));

    let mut files: [u64; 8] = [0; 8];
    for i in 0..files.len()
    {
      files[i] = 0b0000000100000001000000010000000100000001000000010000000100000001 << i;
    }
    write_string_to_file("./src/chess_data_in/files.in", &write_8_array_to_string(&files));

    let mut ranks_64: [u64; 64] = [0; 64];
    for i in 0..ranks_64.len()
    {
      ranks_64[i] = ranks[i/8];
    }
    write_string_to_file("./src/chess_data_in/ranks_64.in", &write_64_array_to_string(&ranks_64));

    let mut files_64: [u64; 64] = [0; 64];
    for i in 0..files_64.len()
    {
        files_64[i] = files[i%8];
    }
    write_string_to_file("./src/chess_data_in/files_64.in", &write_64_array_to_string(&files_64));

    let mut diagonals_64: [u64; 64] = [0; 64];
    for i in 0..8
    {
        let current_diagonal: u64 = (MAIN_DIAGONAL << i) & UPPER_LEFT_SIDE_ZERO;
        let mut temp: u64 = current_diagonal;
        loop
        {
            let index: usize = temp.trailing_zeros() as usize;
            diagonals_64[index] = current_diagonal;
            temp = temp & !(bit_at_index[index]);
            if temp == 0
            {
                break;
            }
        }
    }
    for i in 1..8
    {
        let current_diagonal: u64 = (MAIN_DIAGONAL >> i) & LOWER_RIGHT_SIDE_ZERO;
        let mut temp: u64 = current_diagonal;
        loop
        {
            let index: usize = temp.trailing_zeros() as usize;
            diagonals_64[index] = current_diagonal;
            temp = temp & !(bit_at_index[index]);
            if temp == 0
            {
                break;
            }
        }
    }
    write_string_to_file("./src/chess_data_in/diagonals_64.in", &write_64_array_to_string(&diagonals_64));


    let mut anti_diagonals_64: [u64; 64] = [0; 64];
    for i in 1..8
    {
        let current_anti_diagonal: u64 = (ANTI_DIAGONAL << i) & LOWER_LEFT_SIDE_ZERO;
        let mut temp: u64 = current_anti_diagonal;
        loop
        {
            let index: usize = temp.trailing_zeros() as usize;
            anti_diagonals_64[index] = current_anti_diagonal;
            temp = temp & !(bit_at_index[index]);
            if temp == 0
            {
                break;
            }
        }
    }
    let current_anti_diagonal: u64 = ANTI_DIAGONAL;
    let mut temp: u64 = current_anti_diagonal;
    loop
    {
        let index: usize = temp.trailing_zeros() as usize;
        anti_diagonals_64[index] = current_anti_diagonal;
        temp = temp & !(bit_at_index[index]);
        if temp == 0
        {
            break;
        }
    }
    for i in 1..8
    {
        let current_anti_diagonal: u64 = (ANTI_DIAGONAL >> i) & UPPER_RIGHT_SIDE_ZERO;
        let mut temp: u64 = current_anti_diagonal;
        loop
        {
            let index: usize = temp.trailing_zeros() as usize;
            anti_diagonals_64[index] = current_anti_diagonal;
            temp = temp & !(bit_at_index[index]);
            if temp == 0
            {
                break;
            }
        }
    }
    write_string_to_file("./src/chess_data_in/anti_diagonals_64.in", &write_64_array_to_string(&anti_diagonals_64));

    let mut possible_ranks: [u64; 64] = [0; 64];
    for i in 0..possible_ranks.len()
    {
        let temp: u64 = 0b10000001 | ((i as u64) << 1);
        for j in 0..8
        {
            possible_ranks[i] |= temp << (j*8);
        }
    }

    let mut possible_files: [u64; 64] = [0; 64];
    for i in 0..possible_files.len()
    {
        let temp: u64 = rank_to_file(0b10000001 | ((i as u64) << 1), &files);
        for j in 0..8
        {
            possible_files[i] |= temp << j;
        }
    }

    let mut anti_diagonal_attack_table: [[u64; 64]; 64] = [[0; 64]; 64];
    for index in 0..anti_diagonal_attack_table.len()
    {
        for possible_ranks_index in 0..possible_ranks.len()
        {
            let mut temp_attackmask: u64 = 0;
            let occupancy: u64 = possible_ranks[possible_ranks_index];
            let mut i = index as isize;
            loop
            {
                i += NORTH_WEST;
                if i >= 64
                {
                    break;
                }
                if index%8 == 0
                {
                    break;
                }
                if occupancy & bit_at_index[i as usize] != 0
                {
                    temp_attackmask |= bit_at_index[i as usize];
                    break;
                }
                else
                {
                    temp_attackmask |= bit_at_index[i as usize];
                }
            }
            i = index as isize;
            loop
            {
                i += SOUTH_EAST;
                if i < 0
                {
                    break;
                }
                if index%8 == 7
                {
                    break;
                }
                if occupancy & bit_at_index[i as usize] != 0
                {
                    temp_attackmask |= bit_at_index[i as usize];
                    break;
                }
                else
                {
                    temp_attackmask |= bit_at_index[i as usize];
                }
            }
            anti_diagonal_attack_table[index][get_hashkey_anti_diagonal(index, occupancy,&files, &anti_diagonals_64)] = temp_attackmask;
            /*if(index == 2 && possible_ranks_index == 20)
            {
                print_bitboard(possible_ranks[possible_ranks_index], &bit_at_index);
                print_bitboard(temp_attackmask, &bit_at_index);
                panic!();
            }*/
        }
    }
    write_string_to_file("./src/chess_data_in/anti_diagonal_attack_table.in", &write_64_64_array_to_string(&anti_diagonal_attack_table));

    let mut diagonal_attack_table: [[u64; 64]; 64] = [[0; 64]; 64];
    for index in 0..diagonal_attack_table.len()
    {
        for possible_ranks_index in 0..possible_ranks.len()
        {
            let mut temp_attackmask: u64 = 0;
            let occupancy: u64 = possible_ranks[possible_ranks_index];
            let mut i = index as isize;
            loop
            {
                i += NORTH_EAST;
                if i >= 64
                {
                    break;
                }
                if index%8 == 7
                {
                    break;
                }
                if occupancy & bit_at_index[i as usize] != 0
                {
                    temp_attackmask |= bit_at_index[i as usize];
                    break;
                }
                else
                {
                    temp_attackmask |= bit_at_index[i as usize];
                }
            }
            i = index as isize;
            loop
            {
                i += SOUTH_WEST;
                if i < 0
                {
                    break;
                }
                if index%8 == 0
                {
                    break;
                }
                if occupancy & bit_at_index[i as usize] != 0
                {
                    temp_attackmask |= bit_at_index[i as usize];
                    break;
                }
                else
                {
                    temp_attackmask |= bit_at_index[i as usize];
                }
            }
            diagonal_attack_table[index][get_hashkey_diagonal(index, occupancy, &files, &diagonals_64)] = temp_attackmask;
            /*if(index == 22 && possible_ranks_index == 20)
            {
                print_bitboard(possible_ranks[possible_ranks_index], &bit_at_index);
                print_bitboard(temp_attackmask, &bit_at_index);
            }*/
        }
    }
    write_string_to_file("./src/chess_data_in/diagonal_attack_table.in", &write_64_64_array_to_string(&diagonal_attack_table));

    let mut file_attack_table: [[u64; 64]; 64] = [[0; 64]; 64];
    for index in 0..file_attack_table.len()
    {
        for possible_files_index in 0..possible_files.len()
        {
            let mut temp_attackmask: u64 = 0;
            let occupancy: u64 = possible_files[possible_files_index];
            let mut i = index as isize;
            loop
            {
                i += NORTH;
                if i >= 64
                {
                    break;
                }
                if occupancy & bit_at_index[i as usize] != 0
                {
                    temp_attackmask |= bit_at_index[i as usize];
                    break;
                }
                else
                {
                    temp_attackmask |= bit_at_index[i as usize];
                }
            }
            i = index as isize;
            loop
            {
                i += SOUTH;
                if i < 0
                {
                    break;
                }
                if occupancy & bit_at_index[i as usize] != 0
                {
                    temp_attackmask |= bit_at_index[i as usize];
                    break;
                }
                else
                {
                    temp_attackmask |= bit_at_index[i as usize];
                }
            }
            file_attack_table[index][get_hashkey_file(index, occupancy, &files)] = temp_attackmask;
            /*if(index == 35 && possible_files_index == 20)
            {
                print_bitboard(possible_files[possible_files_index], &bit_at_index);
                print_bitboard(temp_attackmask, &bit_at_index);
            }*/
        }
    }
    write_string_to_file("./src/chess_data_in/file_attack_table.in", &write_64_64_array_to_string(&file_attack_table));

    let mut rank_attack_table: [[u64; 64]; 64] = [[0; 64]; 64];
    for index in 0..rank_attack_table.len()
    {
        for possible_ranks_index in 0..possible_ranks.len()
        {
            let mut temp_attackmask: u64 = 0;
            let occupancy: u64 = possible_ranks[possible_ranks_index];
            let mut i = index as isize;
            loop
            {
                i += EAST;
                if i >= 64
                {
                    break;
                }
                if index%8 == 7
                {
                    break;
                }
                if occupancy & bit_at_index[i as usize] != 0
                {
                    temp_attackmask |= bit_at_index[i as usize];
                    break;
                }
                else
                {
                    temp_attackmask |= bit_at_index[i as usize];
                }
            }
            i = index as isize;
            loop
            {
                i += WEST;
                if i < 0
                {
                    break;
                }
                if index%8 == 0
                {
                    break;
                }
                if occupancy & bit_at_index[i as usize] != 0
                {
                    temp_attackmask |= bit_at_index[i as usize];
                    break;
                }
                else
                {
                    temp_attackmask |= bit_at_index[i as usize];
                }
            }
            rank_attack_table[index][get_hashkey_rank(index, occupancy)] = temp_attackmask;
            /*if(index == 35 && possible_ranks_index == 20)
            {
                print_bitboard(possible_ranks[possible_ranks_index], &bit_at_index);
                print_bitboard(temp_attackmask, &bit_at_index);
            }*/
        }
    }
    write_string_to_file("./src/chess_data_in/rank_attack_table.in", &write_64_64_array_to_string(&rank_attack_table));

    let mut knight_attack_table: [u64; 64] = [0; 64];
    for k in 0..knight_attack_table.len()
    {
        let i = k as isize;
        knight_attack_table[i as usize] = 0;
        if i + NORTH + NORTH_WEST <= 63 && i % 8 != 0
        {
            knight_attack_table[i as usize] |= bit_at_index[(i + NORTH + NORTH_WEST) as usize];
        }
        if i + NORTH + NORTH_EAST <= 63 && (i + 1) % 8 != 0
        {
            knight_attack_table[i as usize] |= bit_at_index[(i + NORTH + NORTH_EAST) as usize];
        }
        if i + NORTH_WEST + WEST <= 63 && i % 8 > 1
        {
            knight_attack_table[i as usize] |= bit_at_index[(i + NORTH_WEST + WEST) as usize];
        }
        if i + NORTH_EAST + EAST <= 63 && (i + 1) % 8 < 7 && (i + 1) % 8 != 0
        {
            knight_attack_table[i as usize] |= bit_at_index[(i + NORTH_EAST + EAST) as usize];
        }
        if i + SOUTH + SOUTH_WEST >= 0 && i % 8 != 0
        {
            knight_attack_table[i as usize] |= bit_at_index[(i + SOUTH + SOUTH_WEST) as usize];
        }
        if i + SOUTH + SOUTH_EAST >= 0 && (i + 1) % 8 != 0
        {
            knight_attack_table[i as usize] |= bit_at_index[(i + SOUTH + SOUTH_EAST) as usize];
        }
        if i + SOUTH_WEST + WEST >= 0 && i % 8 > 1
        {
            knight_attack_table[i as usize] |= bit_at_index[(i + SOUTH_WEST + WEST) as usize];
        }
        if i + SOUTH_EAST + EAST >= 0 && (i + 1) % 8 < 7 && (i + 1) % 8 != 0
        {
            knight_attack_table[i as usize] |= bit_at_index[(i + SOUTH_EAST + EAST) as usize];
        }
        //print_bitboard(knight_attack_table[i as usize], &bit_at_index);
    }
    write_string_to_file("./src/chess_data_in/knight_attack_table.in", &write_64_array_to_string(&knight_attack_table));

    let mut king_attack_table: [u64; 64] = [0; 64];
    for k in 0..king_attack_table.len()
    {
        let i = k as isize;
        king_attack_table[i as usize] = 0;
        if i + NORTH <= 63
        {
            king_attack_table[i as usize] |= bit_at_index[(i + NORTH) as usize];
            if i % 8 != 0
            {
                king_attack_table[i as usize] |= bit_at_index[(i + NORTH_WEST) as usize];
            }
            if (i + EAST) % 8 != 0
            {
                king_attack_table[i as usize] |= bit_at_index[(i + NORTH_EAST) as usize];
            }
        }
        if i + SOUTH >= 0
        {
            king_attack_table[i as usize] |= bit_at_index[(i + SOUTH) as usize];
            if i % 8 != 0
            {
                king_attack_table[i as usize] |= bit_at_index[(i + SOUTH_WEST) as usize];
            }
            if (i + EAST) % 8 != 0
            {
                king_attack_table[i as usize] |= bit_at_index[(i + SOUTH_EAST) as usize];
            }
        }
        if i % 8 != 0
        {
            king_attack_table[i as usize] |= bit_at_index[(i + WEST) as usize];
        }
        if (i + EAST) % 8 != 0
        {
            king_attack_table[i as usize] |= bit_at_index[(i + EAST) as usize];
        }
        //print_bitboard(king_attack_table[i as usize], &bit_at_index);
    }
    write_string_to_file("./src/chess_data_in/king_attack_table.in", &write_64_array_to_string(&king_attack_table));

    let mut pawn_capture_attack_table: [[u64; 64]; 2] = [[0; 64]; 2];
    for i in 0..64
    {
        //white
        if i + NORTH < 64
        {
            if i%8!=0
            {
                pawn_capture_attack_table[0][i as usize] |= bit_at_index[(i +NORTH_WEST)as usize];
            }
            if i%8!=7
            {
                pawn_capture_attack_table[0][i as usize] |= bit_at_index[(i +NORTH_EAST)as usize];
            }
        }
        //black
        if i + SOUTH >= 0
        {
            if i%8!=0
            {
                pawn_capture_attack_table[1][i as usize] |= bit_at_index[(i +SOUTH_WEST)as usize];
            }
            if i%8!=7
            {
                pawn_capture_attack_table[1][i as usize] |= bit_at_index[(i +SOUTH_EAST)as usize];
            }
        }
    }
    write_string_to_file("./src/chess_data_in/pawn_capture_attack_table.in", &write_2_64_array_to_string(&pawn_capture_attack_table));

    let mut pawn_quiet_attack_table: [[u64; 64]; 2] = [[0; 64]; 2];
    for i in 0..64
    {
        //white
        if i + NORTH < 64
        {
            pawn_quiet_attack_table[0][i as usize] |= bit_at_index[(i +NORTH)as usize];
        }
        //black
        if i + SOUTH >= 0
        {
            pawn_quiet_attack_table[1][i as usize] |= bit_at_index[(i +SOUTH)as usize];
        }
    }
    write_string_to_file("./src/chess_data_in/pawn_quiet_attack_table.in", &write_2_64_array_to_string(&pawn_quiet_attack_table));

    let mut is_passed: [[u64; 64]; 2] = [[0; 64]; 2];
    for i in 0..64
    {
        is_passed[0][i] |= files_64[i];
        if i % 8 != 0
        {
            is_passed[0][i] |= files_64[i - 1];
        }
        if i % 8 != 7
        {
            is_passed[0][i] |= files_64[i + 1];
        }
        is_passed[1][i] = is_passed[0][i];

        //WHITE
        for j in 0..((i/8)+1)
        {
            is_passed[0][i] &= !ranks[j];
        }
        //BLACK
        for j in (i/8)..8
        {
            is_passed[1][i] &= !ranks[j];
        }
    }
    write_string_to_file("./src/chess_data_in/is_passed.in", &write_2_64_array_to_string(&is_passed));
    //panic!();
}

fn print64_fields_to_chessboard(field_content: &Vec<String>)
{
    println!("");
    for _ in 0..33
    {
        print!("{}", chess_data::HORIZONTAL_LINE_UNICODE);
    }
    println!("");
    for h in 0..8
    {
        let i = 7 - h;
        for j in 0..8
        {
            print!("{} {} ", chess_data::VERTICAL_LINE_UNICODE, field_content[8*i + j]);
        }
        print!("{} {}", chess_data::VERTICAL_LINE_UNICODE, (i+1) as u32);
        println!("");
        for _ in 0..33
        {
            print!("{}", chess_data::HORIZONTAL_LINE_UNICODE);
        }
        println!("");
    }
  	println!("  A   B   C   D   E   F   G   H");
}

fn print_bitboard(bitboard: u64, bit_at_index: &[u64; 64])
{
  let mut temp: Vec<String> = vec![String::new(); 64];
  for  i in 0..bit_at_index.len()
  {

    temp[i] = format!("{}", chess_data::ZERO_UNICODE);
    if (bitboard & bit_at_index[i]) != 0
    {
        temp[i] = format!("{}", chess_data::ONE_UNICODE);
    }
  }
  print64_fields_to_chessboard(&temp);
}

fn rank_to_file(rank: u64, files: &[u64; 8]) -> u64
{
    (((rank & 0b11111111).wrapping_mul(MAIN_DIAGONAL)) & files[7]) >> 7
}

fn get_hashkey_rank(index: usize, occupancy: u64) -> usize
{
    (((occupancy >> ((index / 8)*8)) >> 1) & 0b111111) as usize
}

fn get_hashkey_file(index: usize , occupancy: u64, files: &[u64; 8]) -> usize
{
    ((((((occupancy >> (index % 8)) & files[0] ).wrapping_mul(MAIN_DIAGONAL)) >> 56) >> 1) & 0b111111) as usize
}

fn get_hashkey_diagonal(index: usize, occupancy: u64, files: &[u64; 8], diagonals_64: &[u64; 64]) -> usize
{
    (((((occupancy & diagonals_64[index]).wrapping_mul(files[0])) >> 56) >> 1) & 0b111111) as usize
    //return ((((occupancy & diagonals64[fieldIndex])* files[0]) >> 56) >> 1) & 0b111111;
}

fn get_hashkey_anti_diagonal(index: usize, occupancy: u64, files: &[u64; 8], anti_diagonals_64: &[u64; 64]) -> usize
{
    (((((occupancy & anti_diagonals_64[index]).wrapping_mul(files[0])) >> 56) >> 1) & 0b111111) as usize
}
