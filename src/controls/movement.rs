use crate::controls::input;

pub trait Movement {
    fn move_in_dir(&mut self, direction: Option<input::PlayerAction>) -> bool;
    fn turn_in_direction(&mut self, direction: Option<input::PlayerAction>) -> bool;
}