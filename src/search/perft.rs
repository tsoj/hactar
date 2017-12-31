use position;


fn perft(depth: u32, position: &mut position::Position, us: position::player::Player, enemy: position::player::Player) -> u64
{
    if depth == 0
    {
        return 1;
    }

    let mut nodes = 0;
    let move_list = position.generate_move_list(us, enemy);
    for i in 0..move_list.len
    {
        let backup_en_passant_castling = position.make_move(&move_list[i], us, enemy);
        if !position.is_check_unkown_kings_index(us, enemy)
        {
            nodes += perft(depth - 1, position, enemy, us);
        }
        position.undo_move(&move_list[i], backup_en_passant_castling, us, enemy);
    }
    nodes
}
pub fn start_perft(mut position: position::Position, depth: u32) -> u64
{
    let enemy = position::player::switch_player(position.whose_move);
    let us = position.whose_move;
    perft(depth, &mut position, us, enemy)
}