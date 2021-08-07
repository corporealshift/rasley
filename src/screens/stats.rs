
use tui::{
    layout::Constraint,
    text::Span,
    style::{Color, Modifier, Style},
    widgets::{Block, Cell, ListState, Table, Row, BorderType, Borders}
};

use crate::player;
use crate::stats::StatReadouts;

pub fn render_stats<'a>(player: &player::Player) -> Table<'a> {
    Table::new(vec![Row::new(vec![
        Cell::from(Span::raw(player.stats.strength.to_string())),
        Cell::from(Span::raw(player.stats.constitution.to_string())),
        Cell::from(Span::raw(player.stats.insight.to_string())),
        Cell::from(Span::raw(player.stats.stamina.to_string())),
        Cell::from(Span::raw(player.stats.knowledge.to_string())),
        Cell::from(Span::raw(player.stats.presence.to_string())),
    ])])
    // create the table headers
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "Strength",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Constitution",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Insight",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Stamina",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Knowledge",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Presence",
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
        Constraint::Percentage(16),
        Constraint::Percentage(17),
        Constraint::Percentage(17),
        Constraint::Percentage(17),
        Constraint::Percentage(17),
        Constraint::Percentage(16),
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