pub type Player = usize;

pub const WHITE: Player = 0;
pub const BLACK: Player = 1;
pub const NO_PLAYER: Player = 2;

pub fn get_unicode(player: Player) -> &'static str
{
    match player
    {
        WHITE =>
        {
            return "White";
        },
        BLACK =>
        {
            return "Black";
        },
        NO_PLAYER =>
        {
            return "-";
        },
        _x =>
        {
            return "?";
        }
    }
}
