use crate::{margin, App};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, List, ListItem, Padding, Paragraph, Wrap},
    Frame, Terminal,
};

pub fn render(app: &mut App, frame: &mut Frame, rect: Rect, colors: Vec<Color>) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Fill(1)])
        .split(rect);

    let par = Paragraph::new(vec![
        Line::from("\n"),
        Line::from("Hello World!").alignment(Alignment::Center),

        Line::from("\n"),
        Line::from("\n"),

        Line::from("So... I like computers.").alignment(Alignment::Center),
        Line::from("I have been learning programing for a while now. I often enjoy learning new languages, libraries, technologies and more!").alignment(Alignment::Center),
        Line::from("If you enhoy tinkering, learning, and believe that it's not about the destination but the journey...").alignment(Alignment::Center),
        Line::from("Please feel free to contact me.").alignment(Alignment::Center),

        Line::from("\n"),
        Line::from("These are my latest pinned repos:").alignment(Alignment::Center),
        Line::from("\n"),
        Line::from("\n"),
        Line::from("\n"),

    ]).wrap(Wrap { trim: true });

    frame.render_widget(par, layout[0]);

    let repos_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .split(layout[1].inner(margin!(5, 2)));

    let mut cur_block_id = 0;
    for i in app.github_repos.iter().take(3) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .padding(Padding {
                left: 2,
                right: 2,
                top: 1,
                bottom: 1,
            });

        let inner_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1),
            ])
            .split(repos_layout[cur_block_id]);

        frame.render_widget(block, repos_layout[cur_block_id]);

        let repo_title = Paragraph::new(i.name.clone());
        let repo_desc = Paragraph::new(i.description.clone()).wrap(Wrap { trim: true });
        let repo_lang = Paragraph::new(i.language.clone());

        frame.render_widget(repo_title, inner_layout[0]);
        frame.render_widget(repo_desc, inner_layout[1]);
        frame.render_widget(repo_lang, inner_layout[2]);
        cur_block_id += 1;
    }
}
