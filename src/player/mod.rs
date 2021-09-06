use std::collections::HashMap;

use crate::{skills, stats};
use crate::skills::VocationSkills;
use crate::combat::{
    arena::squares_in_direction,
    pawn,
    actions,
    combatant::{ CombatFrame, Combatant, player_damage, perform_action },
};
use crate::controls::input::PlayerAction;
use crate::combat::pawn::Pawn;

#[derive(Clone)]
pub struct Player {
    pub name: String,
    exp: Experience,
    pub vocation: skills::Vocation,
    pub stats: stats::Stats,
    pub attrs: stats::Attributes,
    pub max_attrs: stats::Attributes,
    pub pawn: pawn::Pawn,
}

impl Combatant for Player {
    fn get_pawn(&self) -> &Pawn {
        &self.pawn
    }
    fn get_mut_pawn(&mut self) -> &mut Pawn {
        &mut self.pawn
    }

    fn take_damage(&mut self, damage: i64) -> CombatFrame {
        self.attrs.health = self.attrs.health - damage as f32;
        player_damage(format!("You take {} damage", damage))
    }
}

impl actions::CombatAction for Player {
    fn perform_action(&mut self, action: Option<PlayerAction>, combatants: &mut HashMap<(usize, usize), Box<dyn Combatant>>) -> Vec<CombatFrame> {
        match action {
            Some(PlayerAction::BasicAttack) => {
                let mut frames = vec![perform_action(String::from("You swing your weapon in front of you"))];

                let squares = squares_in_direction(&self.pawn, 1, &self.pawn.orientation);
                // Now I just have to match the squares to the combatants
                frames.push(perform_action(format!("Squares: {:?}", squares)));
                for square in squares {
                    match combatants.get_mut(&(square.x, square.y)) {
                        Some(combatant) => {frames.push(combatant.take_damage(5));},
                        _ => {}
                    }
                }
                frames
            }
            _ => Vec::new()
        }
    }

}

pub fn new(name: String, vocation: skills::Vocation) -> Player {
    let max_hp: f32 = (vocation.stats().constitution * 10) as f32;
    // Insight contributes a little bit to max focus
    let max_focus = (vocation.stats().knowledge as f32) + ((vocation.stats().insight as f32) * 0.1) * 10.0;
    let attrs = stats::Attributes{
        health: max_hp,
        exertion: 100.0,
        focus: max_focus,
    };

    let stats = vocation.stats();
    Player {
        name: name,
        exp: NewExperience(),
        vocation: vocation,
        stats: stats,
        attrs: attrs,
        max_attrs: attrs,
        pawn: pawn::player_start(),
    }
}

impl stats::StatReadouts for Player {
    fn hp(&self) -> String {
        // {:.2} truncates the precision of the floating point number to only two decimal points
        format!("{:.2}/{:.0}", self.attrs.health, self.max_attrs.health)
    }

    fn endurance(&self) -> String {
        format!("{:.2}/{:.0}", self.attrs.exertion, self.max_attrs.exertion)
    }

    fn focus(&self) -> String {
        format!("{:.2}/{:.0}", self.attrs.focus, self.max_attrs.focus)
    }

    fn exp(&self) -> String {
        format!("{}/{}", self.exp.current_exp, self.exp.exp_to_next)
    }

    fn level(&self) -> String {
        self.exp.level.to_string()
    }

    fn level_desc(&self) -> String {
        format!("Level {}: Total EXP gained: {}", self.exp.level, self.exp.total_exp)
    }
}

#[derive(Clone)]
pub struct Experience {
    level: u32,
    total_exp: u64,
    exp_to_next: u32,
    current_exp: u32,
}

fn NewExperience() -> Experience {
    Experience{
        level: 1,
        total_exp: 0,
        exp_to_next: 100,
        current_exp: 0,
    }
}
