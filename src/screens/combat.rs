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
use itertools::Itertools;

use crate::menu;
use crate::combat::pawn;
use crate::combat::pawn::Display;
use crate::combat::pawn::Position;

#[derive(Clone)]
struct MapSquare {
    pub glyph: char,
    pub color: Color,
}

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
            .title(menu::MenuItem::Game.to_string())
            .border_type(BorderType::Plain),
    );
    let player_pawn = pawns.first();
    // may want to switch from tuples to a struct for MapSquares or something
    // may also want to have a struct for the Map
    let mut raw_map = vec![vec![MapSquare {glyph: '•', color: Color::DarkGray}; 11]; 11];

    // Assign player position into the raw map
    // I think these map/unwrap statements could be made generic, or put behind a fn
    let pawn_position = player_pawn.map(|p| p.pos).unwrap_or(Position{x: 0, y: 0});
    let pawn_glyph = player_pawn.map(|p| p.glyph()).unwrap_or('X');
    let pawn_color = player_pawn.map(|p| p.color).unwrap_or(Color::White);
    raw_map[pawn_position.y][pawn_position.x] = MapSquare {glyph: pawn_glyph, color: pawn_color};

    // I can make this more efficient by creating a for loop and creating a new Span
    // each time the glyph or color changes. That might get a bit messy looking so I
    // am leaving this as is for now, but if it seems to run slowly the above idea
    // seems to be the best approach for improving performance
    let spans_vec: Vec<Spans> = raw_map.iter().map(|row| {
        let spaces = vec![MapSquare {glyph: ' ', color: Color::DarkGray}; 10];
        let spaced_row = row.into_iter().interleave(spaces.iter());
        let row_render: Vec<Span> = spaced_row.map(|square| {
            Span::styled(square.glyph.to_string(), Style::default().fg(square.color))
        }).collect();

        Spans::from(row_render)
    }).collect();

    let map = Paragraph::new(spans_vec).alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title(String::from("Arena"))
            .border_type(BorderType::Plain),
    );

    rect.render_widget(combat_log, combat_area[0]);
    rect.render_widget(map, combat_area[1]);
}