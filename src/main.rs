use chrono::prelude::*;
use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Widget, BorderType, Block, Borders, ListState, Table, Paragraph, Row, Cell},
};
use tui::layout::{Layout, Constraint, Direction};
use crossterm::{
    event::KeyCode,
    terminal::{disable_raw_mode, enable_raw_mode},
};

mod controls;
mod menu;
mod home;

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
                menu::MenuItem::Home => rect.render_widget(home::render(), chunks[1]),
                menu::MenuItem::Duel => {
                    let combat_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(80), Constraint::Percentage(20)].as_ref(),
                        )
                        .split(chunks[1]);
                    let left = render_combat_log();
                    let right = render_mini_stats(&stats_state);
                    rect.render_widget(left, combat_chunks[0]);
                    rect.render_widget(right, combat_chunks[1]);
                }
                menu::MenuItem::Stats => {}
                // menu::MenuItem::Stats => rect.render_stateful_widget(render_stats(), chunks[1], &mut stats_state),
            }
        }) {
            Ok(_) => {},
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


fn render_combat_log<'a>() -> Paragraph<'a> {
    Paragraph::new(vec![
        Spans::from(vec![Span::raw("A kobold charges you with a spear!")]),
        Spans::from(vec![Span::styled("You were hit for 12", Style::default().fg(Color::Red))]),
        Spans::from(vec![Span::styled(
            "You attacked for 55 damage",
            Style::default().fg(Color::LightBlue),
        )]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title(menu::MenuItem::Duel.to_string())
            .border_type(BorderType::Plain),
    )
}

fn render_mini_stats<'a>(stats_state: &ListState) -> Table<'a> {
    Table::new(vec![Row::new(vec![
        Cell::from(Span::raw("88/100")),
        Cell::from(Span::raw("57654")),
        Cell::from(Span::raw("7")),
    ])])
    // create the table headers
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "HP",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Damage Done",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Level",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Stats")
            .border_type(BorderType::Plain),
    )
    .widths(&[
        Constraint::Percentage(35),
        Constraint::Percentage(35),
        Constraint::Percentage(30),
    ])
}