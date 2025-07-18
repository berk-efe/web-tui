use ratatui::{layout::Rect, text::Line, widgets::Paragraph, Frame};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame, area: Rect) {
    let par = Paragraph::new(vec![
        Line::from("\n"),
        Line::from("\n"),
        Line::from("   Hello World!"),
    ]);

    frame.render_widget(par, area);
}

