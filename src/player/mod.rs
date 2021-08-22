use crate::{skills, stats};
use crate::skills::VocationSkills;
use crate::combat::pawn;

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
