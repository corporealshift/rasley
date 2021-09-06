use crate::controls::input;
use std::collections::HashMap;
use crate::combat::combatant::Combatant;

pub trait Movement {
    fn move_in_dir(&mut self, direction: Option<input::PlayerAction>, combatants: &HashMap<(usize, usize), Box<dyn Combatant>>) -> bool;
    fn turn_in_direction(&mut self, direction: Option<input::PlayerAction>) -> bool;
}