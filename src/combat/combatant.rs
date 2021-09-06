use crate::combat::pawn::Pawn;
use tui::style::{Style, Color};
use crate::player::PLAYER_ID;

pub enum Combat {
    None,

    TookDamage,
    Killed,

    EnemyTookDamage,
    EnemyKilled,
}

pub struct CombatResult {
    pub res_type: Combat,
    pub target_id: u8,
}

pub struct CombatFrame {
    pub message: String,
    pub style: Style,
    pub result: Option<CombatResult>,
}

pub fn player_damage(msg: String) -> CombatFrame {
    CombatFrame {
        message: msg,
        style: Style::default().fg(Color::Red),
        result: Some(CombatResult {
            res_type: Combat::TookDamage,
            target_id: PLAYER_ID,
        })
    } 
}
pub fn perform_action(msg: String) -> CombatFrame {
    CombatFrame {
        message: msg,
        style: Style::default().fg(Color::Gray),
        result: None
    }
}
pub fn enemy_damage(msg: String, enemy: u8) -> CombatFrame {
    CombatFrame {
        message: msg,
        style: Style::default().fg(Color::LightBlue),
        result: Some(CombatResult {
            res_type: Combat::TookDamage,
            target_id: enemy
        })
    }
}
pub fn enemy_defeated(msg: String, enemy: u8) -> CombatFrame {
    CombatFrame {
        message: msg,
        style: Style::default().fg(Color::Green),
        result: Some(CombatResult {
            res_type: Combat::Killed,
            target_id: enemy,
        })
    }
}

pub trait Combatant {
    fn get_pawn(&self) -> &Pawn;
    fn get_mut_pawn(&mut self) -> &mut Pawn;
    fn take_damage(&mut self, damage: i64) -> CombatFrame;
    fn get_id(&self) -> u8;
    fn is_dead(&self) -> bool;
}