use crate::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, List, ListItem, Padding, Paragraph},
    Frame, Terminal,
};

pub fn render(app: &mut App, frame: &mut Frame, rect: Rect, colors: Vec<Color>) {
    let para = Paragraph::new(vec![Line::from("About Me!")]);

    frame.render_widget(para, rect);
}
