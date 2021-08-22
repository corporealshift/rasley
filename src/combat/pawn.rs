#[derive(Clone)]
pub enum Orientation {
    North,
    East,
    West,
    South,
}

#[derive(Clone)]
pub struct Position {
    x: u32,
    y: u32,
}

#[derive(Clone)]
pub struct Pawn {
    orientation: Orientation,
    pos: Position,
}

pub fn player_start() -> Pawn {
    Pawn {
        orientation: Orientation::North,
        pos: Position{x: 5, y: 0}
    }
}