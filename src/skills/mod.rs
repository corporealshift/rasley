use crate::stats;

#[derive(Clone)]
pub struct Vocation {
    pub name: String,
    pub description: String,
    skills: Vec<Skill>,
    starting_stats: stats::Stats,
}

pub trait VocationSkills {
    fn stats(&self) -> stats::Stats;
    fn skills(&self) -> &Vec<Skill>;
    fn available_skills(&self) -> &Vec<Skill>;
}

impl VocationSkills for Vocation {
    fn stats(&self) -> stats::Stats { self.starting_stats }
    fn skills(&self) -> &Vec<Skill> { &self.skills }

    fn available_skills(&self) -> &Vec<Skill> {
        self.skills()
    }
}

#[derive(Clone)]
pub struct Skill {
    name: String,
    description: String,
    range: SkillRange,
    skill_type: SkillType,
    affliction: Option<Affliction>,
}

#[derive(Clone)]
pub enum SkillRange {
    OnSelf,
    Melee,
    Close,
    Medium,
    Long
}

#[derive(Clone)]
pub enum SkillType {
    ImmediateDamage,
    DamageOverTime,
    Heal,
    Buff,
}

// Duration is a mechanism of comparing player level to enemy level
#[derive(Clone)]
pub enum Affliction {
    KnockDown,
    Disorient,
    Disarm,
}

// Probably need to throw Vocations into their own module
pub fn Warrior() -> Vocation {
    Vocation {
        name: String::from("Warrior"),
        description: String::from("The warrior perfects the art of melee and ranged weapons; Always eager to learn new ways to protect himself and his friends from harm."),
        skills: vec![Skill{
            name: String::from("Empowered Slash"),
            description: String::from("Heavy Melee Attack. Slash with unusual strength across your enemy. Can break standard guard."),
            range: SkillRange::Melee,
            skill_type: SkillType::ImmediateDamage,
            affliction: None,
        }],
        starting_stats: stats::Stats{
            strength: 10,
            constitution: 8,
            insight: 5,
            stamina: 7,
            knowledge: 3,
            presence: 4,
        },
    }
}