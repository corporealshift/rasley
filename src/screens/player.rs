
use tui::{
    backend::Backend,
    Frame,
    layout::{Layout, Constraint, Direction, Rect, Alignment},

    text::{Span, Spans},
    style::{Color, Modifier, Style},
    widgets::{Block, Cell, ListState, Paragraph, Table, Row, BorderType, Borders, Wrap},
};

use crate::player;
use crate::stats::StatReadouts;

fn row<'a>(title: &str, value: String) -> Row<'a> {
    Row::new(vec![
        Cell::from(Span::raw(String::from(title))),
        Cell::from(Span::raw(value)),
    ])
}

pub fn render<B>(rect: &mut Frame<B>, player: &player::Player, area: Rect) where
    B: Backend,
{
    let player_screen = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(70),
                        Constraint::Percentage(30),
                    ]
                    .as_ref(),
                )
                .split(area);
    

    let title = format!("{}", player.name);
    let player_screen_box = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title(title)
        .border_type(BorderType::Plain);

    rect.render_widget(player_screen_box, area);

    // Render player description + skills
    let vocation = format!("Vocation: {}", player.vocation.name);
    let v_description = format!("{}", player.vocation.description);
    let description = Paragraph::new(vec![
        Spans::from(vec![Span::styled(vocation, Style::default().fg(Color::LightBlue))]),
        Spans::from(vec![Span::raw(v_description)]),
    ])
    .alignment(Alignment::Left)
    .block(
        Block::default(),
    ).wrap(Wrap { trim: true });
    rect.render_widget(description, player_screen[0]);

    // Render stats + attrs area
    let stats_and_attrs = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ]
                    .as_ref(),
                )
                .split(player_screen[1]);
    let stats = render_stats(player);
    let attrs = render_attrs(player);
    rect.render_widget(stats, stats_and_attrs[0]);
    rect.render_widget(attrs, stats_and_attrs[1]);
}

fn render_stats<'a>(player: &player::Player) -> Table<'a> {
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
            Style::default().add_modifier(Modifier::BOLD).fg(Color::LightBlue),
        )),
        Cell::from(Span::styled(
            "Value",
            Style::default().add_modifier(Modifier::BOLD).fg(Color::LightBlue),
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
        Constraint::Min(12),
        Constraint::Min(4),
    ])
}

fn render_attrs<'a>(player: &player::Player) -> Table<'a> {
    Table::new(vec![
        row("HP", player.hp()),
        row("Endurance", player.endurance()),
        row("Focus", player.focus()),
        row("----------", String::default()).style(Style::default().fg(Color::DarkGray)),
        row("Level", player.level()),
        row("Experience", player.exp()),
    ])
    // create the table headers
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "Attr",
            Style::default().add_modifier(Modifier::BOLD).fg(Color::LightBlue),
        )),
        Cell::from(Span::styled(
            "Value",
            Style::default().add_modifier(Modifier::BOLD).fg(Color::LightBlue),
        )),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Attributes")
            .border_type(BorderType::Plain),
    )
    .widths(&[
        Constraint::Min(10),
        Constraint::Min(14),
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