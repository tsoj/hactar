pub type Player = usize;

pub const WHITE: Player = 0;
pub const BLACK: Player = 1;
pub const NO_PLAYER: Player = 2;

pub fn switch_player(orig_player: Player) -> Player
{
    if orig_player == WHITE
    {
        return BLACK
    }
    WHITE

}
