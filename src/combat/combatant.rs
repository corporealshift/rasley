use crate::combat::pawn::Pawn;
use tui::style::{Style, Color};

pub struct CombatFrame {
    pub message: String,
    pub style: Style,
}

pub fn player_damage(msg: String) -> CombatFrame { CombatFrame {message: msg, style: Style::default().fg(Color::Red)} }
pub fn perform_action(msg: String) -> CombatFrame { CombatFrame {message: msg, style: Style::default().fg(Color::Gray)} }
pub fn enemy_damage(msg: String) -> CombatFrame { CombatFrame {message: msg, style: Style::default().fg(Color::LightBlue)}}

pub trait Combatant {
    fn get_pawn(&self) -> &Pawn;
    fn get_mut_pawn(&mut self) -> &mut Pawn;
    fn take_damage(&mut self, damage: i64) -> CombatFrame;
}