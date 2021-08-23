use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::{
    widgets::{ListState},
};
use tui::layout::{Layout, Constraint, Direction};
use crossterm::{
    event::KeyCode,
    terminal::{disable_raw_mode, enable_raw_mode},
};

mod controls;
mod menu;
mod screens;
mod skills;
mod stats;
mod player;
mod combat;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");

    let key_receiver = controls::input::listen_for_keys();

    // Setup the terminal
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Setup the menu
    let mut active_menu_item = menu::MenuItem::Home;

    // Setup player
    let player = player::new(String::from("Skwared"), skills::Warrior());
    
    // Create stateful list for stats
    let mut stats_state = ListState::default();
    stats_state.select(Some(0));

    loop {
        match terminal.draw(|rect| {
            let size = rect.size();
            // Define top level layout with three sections
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3), // Length is the number of lines tall the box is
                        Constraint::Min(2), // Minimum number of lines of height, fills in between other two
                    ]
                    .as_ref(),
                )
                .split(size);

            // Render the menu, use the menu items defined above the loop
            let tabs = menu::render(active_menu_item);

            // Render the tabs into the window
            rect.render_widget(tabs, chunks[0]);

            // Handle menu selection changed
            match active_menu_item {
                menu::MenuItem::Home => rect.render_widget(screens::home::render(), chunks[1]),
                menu::MenuItem::Duel => {
                    let combat_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(80), Constraint::Percentage(20)].as_ref(),
                        )
                        .split(chunks[1]);
                    screens::combat::render(rect, combat_chunks[0], vec![&player.pawn]);
                    let right = screens::player::render_mini(&stats_state);
                    // rect.render_widget(left, combat_chunks[0]);
                    rect.render_widget(right, combat_chunks[1]);
                }
                menu::MenuItem::Stats => screens::player::render(rect, &player, chunks[1]),
            }
        }) {
            // If draw succeeds great! do nothing because...we drew to the screen
            Ok(_) => {},
            // Otherwise make sure we clean up the terminal before stopping the program
            Err(error) => {
                disable_raw_mode()?;
                terminal.show_cursor()?;
                terminal.clear()?;
                println!("Encountered an error during draw, stopping: {}", error);
                break;
            }
        }

        // Event handling - User keyboard events
        match key_receiver.recv()? {
            controls::input::Event::Input(event) => match event.code {
                // On "q" unlock the terminal that we have control over
                // Breaks us out of the loop
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    terminal.clear()?;
                    break;
                }
                KeyCode::Char('h') => active_menu_item = menu::MenuItem::Home,
                KeyCode::Char('d') => active_menu_item = menu::MenuItem::Duel,
                KeyCode::Char('s') => active_menu_item = menu::MenuItem::Stats,
                _ => {}
            },
            controls::input::Event::Tick => {}
        }
    } // End of render loop
    Ok(())
}
