use tui::{
    backend::Backend,
    Frame,
    symbols::Marker,
    layout::{Layout, Constraint, Direction, Rect, Alignment},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{
        Block, Borders, BorderType, Paragraph,
        canvas::{Canvas, Rectangle,},
    },
};

use crate::menu;
use crate::combat::pawn;

pub fn render<B>(rect: &mut Frame<B>, area: Rect, pawns: Vec<&pawn::Pawn>) where
    B: Backend,
{
    // Render stats + attrs area
    let combat_area = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ]
                    .as_ref(),
                )
                .split(area);
    let combat_log = Paragraph::new(vec![
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
    );
    let player_pawn = pawns.first();

    let map = Paragraph::new(vec![
        Spans::from(vec![Span::styled("• • • • • • • • • • •", Style::default().fg(Color::DarkGray))]),
        Spans::from(vec![Span::styled("• • • • • • • • • • •", Style::default().fg(Color::DarkGray))]),
        Spans::from(vec![Span::styled("• • • • • • • • • • •", Style::default().fg(Color::DarkGray))]),
        Spans::from(vec![Span::styled("• • • • • • • • • • •", Style::default().fg(Color::DarkGray))]),
        Spans::from(vec![Span::styled("• • • • • • • • • • •", Style::default().fg(Color::DarkGray))]),
        Spans::from(vec![Span::styled("• • • • • ", Style::default().fg(Color::DarkGray)), Span::styled(player_pawn.map(|p| {p.glyph}).unwrap_or('P').to_string(), Style::default().fg(player_pawn.map(|p| {p.color}).unwrap_or(Color::Yellow))), Span::styled(" • • • • •", Style::default().fg(Color::DarkGray))]),
        Spans::from(vec![Span::styled("• • • • • • • • • • •", Style::default().fg(Color::DarkGray))]),
        Spans::from(vec![Span::styled("• • • • • • • • • • •", Style::default().fg(Color::DarkGray))]),
        Spans::from(vec![Span::styled("• • • • • • • • • • •", Style::default().fg(Color::DarkGray))]),
        Spans::from(vec![Span::styled("• • • • • • • • • • •", Style::default().fg(Color::DarkGray))]),
        Spans::from(vec![Span::styled("• • • • • • • • • • •", Style::default().fg(Color::DarkGray))]),
    ]).alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title(menu::MenuItem::Duel.to_string())
            .border_type(BorderType::Plain),
    );
    rect.render_widget(combat_log, combat_area[0]);
    rect.render_widget(map, combat_area[1]);
}