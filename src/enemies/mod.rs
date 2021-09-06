use tui::style::Color;
use crate::combat::{
    pawn::{Orientation, Pawn, enemy_start},
    combatant::{
        CombatFrame,
        enemy_damage,
        enemy_defeated,
        Combatant,
    }
};

pub struct TrainingDummy {
    pub name: String,
    health: f32,
    pawn: Pawn,
    id: u8,
}

pub fn new_dummy(id: u8) -> TrainingDummy {
    TrainingDummy {
        name: String::from("Training Dummy"),
        health: 50.0,
        pawn: enemy_start(
            [(Orientation::North, 'T'), (Orientation::South, 'T'), (Orientation::West, 'T'), (Orientation::East, 'T')].iter().cloned().collect(),
            Color::LightRed
        ),
        id: id,
    }
}

impl Combatant for TrainingDummy {
    fn get_id(&self) -> u8 {
        self.id
    }
    fn get_pawn(&self) -> &Pawn {
        &self.pawn
    }
    fn get_mut_pawn(&mut self) -> &mut Pawn {
        &mut self.pawn
    }

    fn take_damage(&mut self, damage: i64) -> CombatFrame {
        self.health = self.health - damage as f32;
        if self.health <= 0.0 {
            enemy_defeated(String::from("Training Dummy has been destroyed"), self.id)
        } else {
            enemy_damage(format!("Training Dummy takes {} damage", damage), self.id)
        }
    }

    fn is_dead(&self) -> bool {
        self.health <= 0.0
    }
}