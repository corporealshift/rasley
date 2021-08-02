#[derive(Copy, Clone, Debug)]
pub enum MenuItem {
    Home,
    Duel,
    Stats
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Duel => 1,
            MenuItem::Stats => 2,
        }
    }
}

impl ToString for MenuItem {
    fn to_string(&self) -> String {
        match self {
            MenuItem::Home => String::from("Home"),
            MenuItem::Duel => String::from("Duel"),
            MenuItem::Stats => String::from("Stats"),
        }
    }
}
