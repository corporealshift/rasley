use tui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, BorderType, Paragraph},
};

use crate::menu;

pub fn render<'a>() -> Paragraph<'a> {
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