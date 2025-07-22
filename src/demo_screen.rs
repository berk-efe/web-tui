use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame, Terminal,
};

use crate::App;

pub fn render(app: &mut App, frame: &mut Frame, colors: Vec<Color>) {
    let block = Block::bordered()
        .title("bruh")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    let text = format!(
        "This is a Ratzilla template.\n\
             Press left and right to increment and decrement the counter respectively.\n\
             Counter: {}", app.counter
    );

    let paragraph = Paragraph::new(text)
        .block(block)
        .fg(Color::White)
        .bg(Color::Black)
        .centered();

    frame.render_widget(paragraph, frame.area());
}
