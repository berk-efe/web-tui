use crate::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, List, ListItem, Padding, Paragraph},
    Frame, Terminal,
};

pub fn render(app: &mut App, frame: &mut Frame, rect: Rect, colors: Vec<Color>) {
    let para = Paragraph::new(vec![
        Line::from("Here are some cool stuff and resources:"),
        Line::from(""),
        Line::from("Learn stuff:"),
        Line::from("    https://www.hackerrank.com/"),
        Line::from("    https://learnxinyminutes.com/"),
        Line::from("    https://microcorruption.com/"),
        Line::from("    https://cs50.harvard.edu/x/2025/"),
        Line::from("    https://roadmap.sh/"),
        Line::from("    https://vkguide.dev/"),
        Line::from(""),
        Line::from("Cool people:"),
        Line::from("    https://github.com/qewer33"),
        Line::from("    https://github.com/SpaciousCoder78"),
        Line::from("    https://github.com/orhun"),
        Line::from(""),
        Line::from("Cool stuff:"),
        Line::from("    https://processing.org/"),
        Line::from("    https://github.com/markoni985/NesCat"),
        Line::from("    https://p5js.org/"),
        Line::from(""),
        Line::from("Funny bussiness:"),
        Line::from("    https://imgflip.com/memegenerator"),
        Line::from("    https://st.ayaka.one/"),
        Line::from("    https://fmhy.net/"),
    ]);

    frame.render_widget(para, rect);
}
