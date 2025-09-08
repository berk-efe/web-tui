use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Borders, List, ListItem, Padding, Paragraph},
    Frame, Terminal,
};
use ratzilla::widgets::Hyperlink;

use crate::{margin, App, CurrentPage};

use crate::{about_me_page, home_page, projects_page};

pub fn render(app: &mut App, frame: &mut Frame, colors: Vec<Color>) {
    let _background = Block::default().borders(Borders::NONE).bg(colors[0]);

    frame.render_widget(_background, frame.area());

    let layout = Layout::default()
        .vertical_margin(3)
        .horizontal_margin(15)
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let header_block = Block::default()
        .borders(Borders::NONE)
        .bg(colors[1])
        .padding(Padding {
            left: 2,
            right: 2,
            top: 1,
            bottom: 1,
        });

    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(25), Constraint::Fill(1)])
        .split(layout[1]);

    let sidebar_block = Block::default()
        .borders(Borders::TOP)
        .title("- components ")
        .bg(colors[1])
        .padding(Padding {
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
        });

    let content_block = Block::default()
        .borders(Borders::TOP)
        .title("- content ")
        .bg(colors[1]);
    let footer_block = Block::default().borders(Borders::NONE).bg(colors[1]);

    let header_area = layout[0].inner(margin!(2, 0));
    let sidebar_area = main_layout[0].inner(margin!(2, 1));
    let content_area = main_layout[1].inner(margin!(2, 1));

    let footer_area = layout[2].inner(margin!(2, 0));

    let github_link: &str = "https://github.com/berk-efe/";

    let header_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(2),
            Constraint::Length(github_link.len() as u16),
        ])
        .split(header_block.inner(header_area));

    frame.render_widget(header_block, header_area);

    let header_para = Paragraph::new(String::from("Berk Efe Keskin"));
    let header_git_link = Hyperlink::new(github_link);

    let mut sidebar_list_items: Vec<ListItem> = Vec::new();

    for i in &app.pages {
        sidebar_list_items.push(ListItem::from(i.title.to_string()));
    }

    frame.render_widget(sidebar_block, sidebar_area);

    let sidebar_list =
        List::new(sidebar_list_items).highlight_style(Style::default().bg(colors[2]));

    frame.render_widget(header_para, header_layout[0]);
    frame.render_widget(header_git_link, header_layout[1]);
    frame.render_stateful_widget(
        sidebar_list,
        main_layout[0].inner(margin!(0, 2)),
        &mut app.sidebar_state,
    );

    frame.render_widget(content_block, content_area);
    frame.render_widget(footer_block, footer_area);

    app.current_page = app.pages[app.sidebar_state.selected().unwrap()]
        .page
        .clone();

    // RENDER THE CONTENTS OF COMPONENTS
    match app.current_page {
        CurrentPage::Home => {
            home_page::render(app, frame, content_area.inner(margin!(2, 1)), colors)
        }
        CurrentPage::Projects => {
            projects_page::render(app, frame, content_area.inner(margin!(2, 1)), colors)
        }
        CurrentPage::AboutMe => {
            about_me_page::render(app, frame, content_area.inner(margin!(2, 1)), colors)
        }

        _ => {}
    }
}
