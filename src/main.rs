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

use crate::controls::{
    movement::Movement,
    input::PlayerAction,
};

use crate::combat::combatant::{Combatant, CombatFrame};
use crate::combat::actions::CombatAction;

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
    let mut player = player::new(String::from("Skwared"), skills::Warrior());
    let mut latest_player_action: Option<PlayerAction> = None;

    // Setup combat
    let mut combatants: Vec<Box<dyn Combatant>> = vec![];
    let mut combat_log: Vec<CombatFrame> = vec![];
    
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

            // Update the player state, but only when the "Game" menu is active
            match active_menu_item {
                menu::MenuItem::Game => {
                    player.pawn.move_in_dir(latest_player_action.clone());
                    player.pawn.turn_in_direction(latest_player_action.clone());
                    match player.perform_action(latest_player_action.clone(), &mut combatants) {
                        Some(frame) => combat_log.push(frame),
                        None => {}
                    }
                }
                _ => {}
            }

            // Reset player actions
            latest_player_action = None;

            // Handle menu selection changed
            match active_menu_item {
                menu::MenuItem::Home => rect.render_widget(screens::home::render(), chunks[1]),
                menu::MenuItem::Game => {
                    let combat_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(80), Constraint::Percentage(20)].as_ref(),
                        )
                        .split(chunks[1]);
                    screens::combat::render(rect, combat_chunks[0], &player, &mut combatants, &combat_log);
                    let right = screens::player::render_mini(&stats_state);
                    // rect.render_widget(left, combat_chunks[0]);
                    rect.render_widget(right, combat_chunks[1]);
                }
                menu::MenuItem::Player => screens::player::render(rect, &player, chunks[1]),
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
                KeyCode::Char('g') => active_menu_item = menu::MenuItem::Game,
                KeyCode::Char('p') => active_menu_item = menu::MenuItem::Player,
                KeyCode::Char('w') => latest_player_action = Some(controls::input::PlayerAction::MoveForward),
                KeyCode::Char('s') => latest_player_action = Some(controls::input::PlayerAction::MoveBackward),
                KeyCode::Char('a') => latest_player_action = Some(controls::input::PlayerAction::MoveLeft),
                KeyCode::Char('d') => latest_player_action = Some(controls::input::PlayerAction::MoveRight),
                KeyCode::Left => latest_player_action = Some(controls::input::PlayerAction::LookLeft),
                KeyCode::Right => latest_player_action = Some(controls::input::PlayerAction::LookRight),
                KeyCode::Down => latest_player_action = Some(controls::input::PlayerAction::LookDown),
                KeyCode::Up => latest_player_action = Some(controls::input::PlayerAction::LookUp),
                KeyCode::Char(' ') => latest_player_action = Some(controls::input::PlayerAction::BasicAttack),
                _ => {}
            },
            controls::input::Event::Tick => {}
        }
    } // End of render loop
    Ok(())
}
