use position;
use search;

fn perft(depth: search::Depth, orig_position: &position::Position) -> u64
{
    if depth == 0
    {
        return 1;
    }

    let mut nodes = 0;
    let move_list = orig_position.generate_move_list();
    for i in 0..move_list.len
    {
        let mut new_position = orig_position.clone();
        new_position.make_move(&move_list[i]);
        if new_position.calculate_zobristkey() !=new_position.zobrist_key
        {
            println!("zobrist key generation faulty.");
            panic!();
        }
        if !new_position.is_check_unkown_kings_index(orig_position.us, orig_position.enemy)
        {
            nodes += perft(depth - 1, &new_position);
        }
    }
    nodes
}
pub fn start_perft(mut position: position::Position, depth: search::Depth) -> u64
{
    perft(depth, &mut position)
}

pub fn test_perft() -> bool
{
    let mut p = position::Position::empty_position();
    p.set_from_fen(&"r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0".to_string());
    if start_perft(p, 4) != 4085603
    {
        println!("\nFailed Perft-Test");
        println!("{}", start_perft(p, 4));
        return false;
    }
    true
}
