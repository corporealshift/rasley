use tui::style::Color;
use std::collections::HashMap;
use std::convert::TryFrom;

use crate::controls::{
    movement::Movement,
    input::PlayerAction,
};

#[derive(Hash, Clone, Eq, PartialEq)]
pub enum Orientation {
    North,
    East,
    West,
    South,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
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


impl Movement for Pawn {
    fn move_in_dir(&mut self, direction: Option<PlayerAction>) -> bool {
        match direction {
            Some(dir) => 
            match &self.orientation {
                Orientation::North => {
                    match dir {
                        PlayerAction::MoveForward => move_y(self, -1),
                        PlayerAction::MoveBackward => move_y(self, 1),
                        PlayerAction::MoveLeft => move_x(self, -1),
                        PlayerAction::MoveRight => move_x(self, 1),
                        _ => false
                    }
                }
                Orientation::South => {
                    match dir {
                        PlayerAction::MoveForward => move_y(self, 1),
                        PlayerAction::MoveBackward => move_y(self, -1),
                        PlayerAction::MoveLeft => move_x(self, 1),
                        PlayerAction::MoveRight => move_x(self, -1),
                        _ => false
                    }
                }
                Orientation::West => {
                    match dir {
                        PlayerAction::MoveForward => move_x(self, -1),
                        PlayerAction::MoveBackward => move_x(self, 1),
                        PlayerAction::MoveLeft => move_y(self, 1),
                        PlayerAction::MoveRight => move_y(self, -1),
                        _ => false
                    }
                }
                Orientation::East => {
                    match dir {
                        PlayerAction::MoveForward => move_x(self, 1),
                        PlayerAction::MoveBackward => move_x(self, -1),
                        PlayerAction::MoveLeft => move_y(self, -1),
                        PlayerAction::MoveRight => move_y(self, 1),
                        _ => false
                    }
                }
            }
            None => false
        }
    }

    fn turn_in_direction(&mut self, direction: Option<PlayerAction>) -> bool {
        match direction {
            Some(dir) => match dir {
                PlayerAction::LookLeft => turn_to(self, Orientation::West),
                PlayerAction::LookRight => turn_to(self, Orientation::East),
                PlayerAction::LookDown => turn_to(self, Orientation::South),
                PlayerAction::LookUp => turn_to(self, Orientation::North),
                _ => false
            }

            // This code changes direction based on the orientation of the pawn, not the gameboard.
            // I played with it for a second and it seemed really confusing that way
            // match &self.orientation {
            //     Orientation::North => {
            //         match dir {
            //             PlayerAction::TurnLeft => turn_to(self, Orientation::West),
            //             PlayerAction::TurnRight => turn_to(self, Orientation::East),
            //             PlayerAction::TurnAround => turn_to(self, Orientation::South),
            //             _ => false
            //         }
            //     }
            //     Orientation::South => {
            //         match dir {
            //             PlayerAction::TurnLeft => turn_to(self, Orientation::East),
            //             PlayerAction::TurnRight => turn_to(self, Orientation::West),
            //             PlayerAction::TurnAround => turn_to(self, Orientation::North),
            //             _ => false
            //         }
            //     }
            //     Orientation::West => {
            //         match dir {
            //             PlayerAction::TurnLeft => turn_to(self, Orientation::South),
            //             PlayerAction::TurnRight => turn_to(self, Orientation::North),
            //             PlayerAction::TurnAround => turn_to(self, Orientation::East),
            //             _ => false
            //         }
            //     }
            //     Orientation::East => {
            //         match dir {
            //             PlayerAction::TurnLeft => turn_to(self, Orientation::North),
            //             PlayerAction::TurnRight => turn_to(self, Orientation::South),
            //             PlayerAction::TurnAround => turn_to(self, Orientation::West),
            //             _ => false
            //         }
            //     }
            // }
            None => false
        }
    }
}

fn move_y(pawn: &mut Pawn, spaces: i8) -> bool {
    // This could give a negative number, which is invalid for a usize
    let new_pos = pawn.pos.y as i8 + spaces;
    // Make sure we have a valid usize before we use it
    match usize::try_from(new_pos).ok() {
        Some(compat_pos) if compat_pos <= 10 => {
            pawn.pos.y = compat_pos;
            return true;
        }
        _ => { return false }
    }
}

fn move_x(pawn: &mut Pawn, spaces: i8) -> bool {
    // This could give a negative number, which is invalid for a usize
    let new_pos = pawn.pos.x as i8 + spaces;
    // Make sure we have a valid usize before we use it
    match usize::try_from(new_pos).ok() {
        Some(compat_pos) if compat_pos <= 10 => {
            pawn.pos.x = compat_pos;
            return true;
        }
        _ => { return false }
    }
}

fn turn_to(pawn: &mut Pawn, new_dir: Orientation) -> bool {
    pawn.orientation = new_dir;
    true
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

pub fn enemy_start(glyph_set: HashMap<Orientation, char>, color: Color) -> Pawn {
    Pawn {
        orientation: Orientation::South,
        pos: Position{ x: 5, y: 0},
        glyph_set: glyph_set,
        color: color
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