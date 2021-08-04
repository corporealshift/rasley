pub struct Vocation {
    name: String,
    description: String,
    skills: Vec<Skill>,
}

pub struct Skill {
    name: String,
    description: String,
    range: SkillRange,
    skill_type: SkillType,
    affliction: Affliction,
}

pub enum SkillRange {
    OnSelf,
    Melee,
    Close,
    Medium,
    Long
}

pub enum SkillType {
    ImmediateDamage,
    DamageOverTime,
    Heal,
    Buff,
}

// Duration is a mechanism of comparing player level to enemy level
pub enum Affliction {
    KnockDown,
    Disorient,
    Disarm,
}