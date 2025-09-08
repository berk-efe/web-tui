use crate::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, List, ListItem, Padding, Paragraph, Wrap},
    Frame, Terminal,
};

pub fn render(app: &mut App, frame: &mut Frame, rect: Rect, colors: Vec<Color>) {
    let par = Paragraph::new(vec![Line::from("What I'm working on lately:")]);

    frame.render_widget(par, rect);

    let block = Block::default().borders(Borders::ALL).padding(Padding {
        left: 2,
        right: 2,
        top: 3,
        bottom: 3,
    });

    let block_in_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .split(block.inner(rect));

    let cur_block = 0;
    for i in &app.latest_github_repos {
        let repo_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .padding(Padding {
                left: 2,
                right: 2,
                top: 1,
                bottom: 1,
            });

        let repo_block_in_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1),
            ])
            .split(repo_block.inner(block_in_layout[cur_block]));

        let repo_title = Paragraph::new(i.name.clone());
        let repo_desc = Paragraph::new(i.description.clone()).wrap(Wrap { trim: true });
        let repo_lang = Paragraph::new(i.language.clone());

        frame.render_widget(repo_title, repo_block_in_layout[0]);
        frame.render_widget(repo_desc, repo_block_in_layout[1]);
        frame.render_widget(repo_lang, repo_block_in_layout[2]);
    }
}
