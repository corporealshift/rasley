use crate::{skills, stats};

pub struct Player {
    name: String,
    exp: Experience,
    hp: u32,
    max_hp: u32,
    class: skills::Vocation,
    stats: stats::Stats,
}

pub struct Experience {
    level: u32,
    total_exp: u64,
    exp_to_next: u32,
    current_exp: u32,
}

