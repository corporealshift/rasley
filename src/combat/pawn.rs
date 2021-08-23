use tui::style::Color;

#[derive(Clone)]
pub enum Orientation {
    North,
    East,
    West,
    South,
}

#[derive(Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone)]
pub struct Pawn {
    pub orientation: Orientation,
    pub pos: Position,
    pub glyph: char,
    pub color: Color,
}

pub fn player_start() -> Pawn {
    Pawn {
        orientation: Orientation::North,
        pos: Position{x: 0.0, y: 0.0},
        glyph: 'P',
        color: Color::Yellow,
    }
}