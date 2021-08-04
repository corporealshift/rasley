pub struct Stats {
    // Strength is increased by defeating foes that are stronger or similar in strength to you
    // Soft cap is that there are no more things as strong/stronger than you
    strength: u16, // How far can you throw the rock?
    // Constitution is increased by getting hurt or blocking hits
    // Soft cap is by constitution itself - the more you have, the less incoming damage counts
    constitution: u16, // How much can you take?
    // Insight is increased by successfully blocking / preventing damage
    // Soft cap: Required chain of successful blocks gets longer
    insight: u16, // How well can you guess what is going to happen
    // Stamina is increased by keeping your 'exertion' high over time
    // Soft cap is how long you have to keep exertion in the 'ideal' range
    stamina: u16, // How long can you do <thing> ? Could be physical or mental
    // Not sure what triggers knowledge increases yet
    knowledge: u16, // Magic users - How much do you know about the world and how it works?
    // Not sure what triggers presence increases yet
    presence: u16, // Can be portrayed in many ways but is a gauge of how "imposing" or "intimidating" you are
}

pub struct Attributes {
    health: u32,
    exertion: u8, // Default is "zero" but you use it up the more you do - max is always 100 - kind of like overheating
    focus: u32, // Starts high but is expended by certain tasks (primarily magic)
}