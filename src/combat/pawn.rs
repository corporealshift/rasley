use tui::style::Color;
use std::collections::HashMap;

#[derive(Hash, Clone, Eq, PartialEq)]
pub enum Orientation {
    North,
    East,
    West,
    South,
}

#[derive(Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone)]
pub struct Pawn {
    pub orientation: Orientation,
    pub pos: Position,
    glyph_set: HashMap<Orientation, char>,
    pub color: Color,
}

const DEFAULT_GLYPHS: [(Orientation, char); 4] = [(Orientation::North, '▲'), (Orientation::South, '▼'), (Orientation::West, '◀'), (Orientation::East, '▶')];

pub fn player_start() -> Pawn {
    Pawn {
        orientation: Orientation::North,
        pos: Position{x: 5, y: 10},
        glyph_set: DEFAULT_GLYPHS.iter().cloned().collect(),
        color: Color::LightGreen,
    }
}

pub trait Display {
    fn glyph(&self) -> char;
}

impl Display for Pawn {
    fn glyph(&self) -> char {
        return *self.glyph_set.get(&self.orientation).unwrap_or(&'P');
    }
}