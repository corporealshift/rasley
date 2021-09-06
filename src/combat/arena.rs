use crate::combat::pawn::{
    Orientation,
    Pawn,
    Position
};
use std::convert::TryFrom;

pub fn squares_in_direction(pawn: &Pawn, num_squares: i8, direction: &Orientation) -> Vec<Position> {
    match direction {
        Orientation::North => add_y_if_valid(pawn, num_squares, |p_y, delta| { usize::try_from(p_y as isize - delta as isize).ok() }),
        Orientation::South => add_y_if_valid(pawn, num_squares, |p_y, delta| { usize::try_from(p_y + delta).ok() }),
        Orientation::West => add_x_if_valid(pawn, num_squares, |p_x, delta| { usize::try_from(p_x as isize - delta as isize ).ok() }),
        Orientation::East => add_x_if_valid(pawn, num_squares, |p_x, delta| { usize::try_from(p_x + delta ).ok() }),
    }
}

fn add_y_if_valid(pawn: &Pawn, num_squares: i8, f: fn(usize, usize) -> Option<usize>) -> Vec<Position> {
    let mut squares = vec![];
    for i in 1..=num_squares {
        // Make sure i is inside the arena
        if i <= 10 {
            // Make sure we're _still_ in bounds after we do the mathses
            match f(pawn.pos.y, i as usize) {
                Some(safe_pos) if safe_pos <= 10 => {squares.push(Position{x: pawn.pos.x, y: safe_pos})}
                _ => {}
            }
        }
    }
    squares
}

fn add_x_if_valid(pawn: &Pawn, num_squares: i8, f: fn(usize, usize) -> Option<usize>) -> Vec<Position> {
    let mut squares = vec![];
    for i in 1..=num_squares {
        // Make sure i is inside the arena
        if i <= 10 {
            // Make sure we're _still_ in bounds after we do the mathses
            match f(pawn.pos.x, i as usize) {
                Some(safe_pos) if safe_pos <= 10 => {squares.push(Position{x: safe_pos, y: pawn.pos.y})}
                _ => {}
            }
        }
    }
    squares
}