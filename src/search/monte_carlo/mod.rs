#![allow(dead_code)]

use position::Position;

struct MctNode
{
    childs: Vec<MctNode>,
    wins: usize,
    simulations: usize,
    position: Position
}
