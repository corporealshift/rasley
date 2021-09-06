use std::collections::HashMap;

use crate::CombatFrame;
use crate::controls::input::PlayerAction;
use crate::combat::combatant::Combatant;

pub trait CombatAction {
    fn perform_action(&mut self, action: Option<PlayerAction>, combatants: &mut HashMap<(usize, usize), Box<dyn Combatant>>) -> Option<CombatFrame>;
}