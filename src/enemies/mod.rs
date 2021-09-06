use tui::style::Color;
use crate::combat::{
    pawn::{Orientation, Pawn, enemy_start},
    combatant::{
        CombatFrame,
        enemy_damage,
        Combatant,
    }
};

pub struct TrainingDummy {
    pub name: String,
    health: f32,
    pawn: Pawn,
}

pub fn new_dummy() -> TrainingDummy { TrainingDummy {
    name: String::from("Training Dummy"),
    health: 50.0,
    pawn: enemy_start(
            [(Orientation::North, 'T'), (Orientation::South, 'T'), (Orientation::West, 'T'), (Orientation::East, 'T')].iter().cloned().collect(),
            Color::LightRed
        )
    }
}

impl Combatant for TrainingDummy {
    fn get_pawn(&self) -> &Pawn {
        &self.pawn
    }
    fn get_mut_pawn(&mut self) -> &mut Pawn {
        &mut self.pawn
    }

    fn take_damage(&mut self, damage: i64) -> CombatFrame {
        self.health = self.health - damage as f32;
        enemy_damage(format!("Training Dummy takes {} damage", damage))
    }
}