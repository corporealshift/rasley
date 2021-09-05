use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, Borders, Tabs,
    },
};

#[derive(Copy, Clone, Debug)]
pub enum MenuItem {
    Home,
    Game,
    Player
}

// This implementation makes it possible for us to match against the
// menu element from TUI with a MenuItem.
impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Game => 1,
            MenuItem::Player => 2,
        }
    }
}

impl ToString for MenuItem {
    fn to_string(&self) -> String {
        match self {
            MenuItem::Home => String::from("Home"),
            MenuItem::Game => String::from("Game"),
            MenuItem::Player => String::from("Player"),
        }
    }
}

pub fn render<'a>(active_menu_item: MenuItem) -> Tabs<'a> {
    let menu_titles = vec![MenuItem::Home, MenuItem::Game, MenuItem::Player];
    let menu = menu_titles
        .iter()
        .map(|t| {
            let menu_name_str = t.to_string();
            let (first, rest) = menu_name_str.split_at(1);
            let styled = vec![
                Span::styled(
                    String::from(first),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(
                    String::from(rest),
                    Style::default()
                        .fg(Color::White),
                ),
            ];
            
            Spans::from(styled)
        })
        .collect();
    // Now that we created the menu items themselves, put them into Tabs
    Tabs::new(menu)
        .select(active_menu_item.into())
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("|"))
}