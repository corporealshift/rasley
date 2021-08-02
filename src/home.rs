use tui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Paragraph,
    },
};
use crate::menu;

pub fn render<'a>() -> Paragraph<'a> {
    Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Welcome")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("to")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "Seyral",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Press 'd' to duel, 's' to view your stats, and 'h' to return to this screen.")]),
        Spans::from(vec![Span::raw("Press 'q' to quit!")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title(menu::MenuItem::Home.to_string())
            .border_type(BorderType::Plain),
    )
}