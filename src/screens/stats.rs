
use tui::{
    layout::Constraint,
    text::Span,
    style::{Color, Modifier, Style},
    widgets::{Block, Cell, ListState, Table, Row, BorderType, Borders}
};

pub fn render<'a>(stats_state: &ListState) -> Table<'a> {
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