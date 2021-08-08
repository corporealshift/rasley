
use tui::{
    layout::Constraint,
    text::{Span, Spans},
    style::{Color, Modifier, Style},
    widgets::{Block, Cell, ListState, List, ListItem, Table, Row, BorderType, Borders}
};

use crate::player;
use crate::stats::StatReadouts;

fn row<'a>(title: &str, value: String) -> Row<'a> {
    Row::new(vec![
        Cell::from(Span::raw(String::from(title))),
        Cell::from(Span::raw(value)),
    ])
}

pub fn render_stats<'a>(player: &player::Player) -> Table<'a> {
    Table::new(vec![
        row("Strength", player.stats.strength.to_string()),
        row("Constitution", player.stats.constitution.to_string()),
        row("Insight", player.stats.insight.to_string()),
        row("Knowledge", player.stats.knowledge.to_string()),
        row("Presence", player.stats.presence.to_string()),
    ])
    // create the table headers
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "Stat",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Current Value",
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
        Constraint::Percentage(75),
        Constraint::Percentage(25),
    ])
}

pub fn render_attributes<'a>(player: player::Player) -> Table<'a> {
    Table::new(vec![Row::new(vec![
        Cell::from(Span::raw(player.hp())),
        Cell::from(Span::raw(player.endurance())),
        Cell::from(Span::raw(player.focus())),
        Cell::from(Span::raw(player.exp()))
    ])])
    // create the table headers
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "HP",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Endurance",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Focus",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Experience",
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
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ])
}

pub fn render_mini<'a>(stats_state: &ListState) -> Table<'a> {
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