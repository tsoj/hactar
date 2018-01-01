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

pub fn test_perft() -> bool
{
    let mut p = position::Position::empty_position();
    p.set_from_fen(&"r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0".to_string());
    if start_perft(p, 4) != 4085603
    {
        println!("\nFailed Perft-Test");
        return false;
    }
    true
}
