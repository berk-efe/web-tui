use crate::app::App;
use ratatui::{
    layout::Alignment,
    prelude::Stylize,
    style::Color,
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

pub fn render(app: &mut App, frame: &mut Frame) {
    let block = Block::bordered()
        .title("web0")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    let text = format!(
        "This is a Ratzilla template.\n\
         Press left and right to increment and decrement the counter respectively.\n\
         Counter: {}\n\
         Press 'm' to go back to Main screen.",
        app.counter
    );

    let paragraph = Paragraph::new(text)
        .block(block)
        .fg(Color::White)
        .bg(Color::Black)
        .centered();

    frame.render_widget(paragraph, frame.area());
}

