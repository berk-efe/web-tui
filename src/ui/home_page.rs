use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::Line,
    widgets::{block, Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use crate::helper;

pub fn render(app: &mut App, frame: &mut Frame, area: Rect) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Fill(1)])
        .split(area);

    let par = Paragraph::new(vec![
        Line::from("\n"), // gotta leave some lines
        Line::from("\n"), // so we dont override the title

        Line::from("Hello World!").alignment(Alignment::Center),

        Line::from("\n"),
        Line::from("\n"),

        Line::from("So... I like computers.").alignment(Alignment::Center),
        Line::from("I have been learning programing for a while now. I often enjoy learning new languages, libraries, technologies and more!").alignment(Alignment::Center),
        Line::from("If you enhoy tinkering and learning. And believe that it's not about the destination but the journey...").alignment(Alignment::Center),
        Line::from("Please feel free to contact me.").alignment(Alignment::Center),

        Line::from("\n"),
        Line::from("These are my latest pinned repos:").alignment(Alignment::Center),
        Line::from("\n"),
        Line::from("\n"),
        Line::from("\n"),

    ]).wrap(Wrap { trim: true });

    frame.render_widget(par, layout[0]);

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .vertical_margin(3)
        .horizontal_margin(10)
        .constraints([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .split(layout[1]);

    let mut cur_block = 0;

    for i in &app.github_repos {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        frame.render_widget(block, layout[cur_block]);

        cur_block += 1;
    }
}
