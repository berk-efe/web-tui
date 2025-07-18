use crate::app::{App, CurrentScreen};
use ratatui::{
    layout::{Alignment, Constraint, Direction::*, Flex, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    symbols::scrollbar,
    text::{Line, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation},
    Frame,
};

const BG_COLOR_0: Color = Color::Rgb(0, 19, 45);
const BG_COLOR_1: Color = Color::Rgb(0, 38, 87);
const BG_COLOR_2: Color = Color::Rgb(0, 55, 126);

pub fn render(app: &mut App, frame: &mut Frame) {
    let areas = create_layout(frame);

    render_background(frame, &areas);
    render_header(app, frame, areas.header);
    render_sidebar(app, frame, areas.sidebar);
    render_footer(app, frame, areas.footer);
    render_main_content(app, frame, areas.main);
}

struct LayoutAreas {
    header: Rect,
    sidebar: Rect,
    main: Rect,
    footer: Rect,
}

fn create_layout(frame: &mut Frame) -> LayoutAreas {
    // Main background
    let block = Block::default().bg(BG_COLOR_0).borders(Borders::NONE);
    frame.render_widget(block, frame.area());

    // Vertical layout
    let main_chunks = Layout::default()
        .direction(Vertical)
        .vertical_margin(5)
        .horizontal_margin(35)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let (header, body, footer) = (main_chunks[0], main_chunks[1], main_chunks[2]);

    // Horizontal layout for body
    let body_chunks = Layout::default()
        .direction(Horizontal)
        .constraints([Constraint::Length(25), Constraint::Min(1)])
        .split(body);

    let (sidebar, main) = (body_chunks[0], body_chunks[1]);

    LayoutAreas {
        header,
        sidebar,
        main,
        footer,
    }
}

fn render_background(frame: &mut Frame, areas: &LayoutAreas) {
    let header_block = Block::default().bg(BG_COLOR_1).borders(Borders::NONE);
    let footer_block = Block::default().bg(BG_COLOR_1).borders(Borders::NONE);
    let sidebar_block = Block::default().bg(BG_COLOR_1).borders(Borders::NONE);
    let main_block = Block::default().bg(BG_COLOR_1).borders(Borders::NONE);

    frame.render_widget(header_block, areas.header.inner(Margin::new(1, 0)));
    frame.render_widget(footer_block, areas.footer.inner(Margin::new(1, 0)));
    frame.render_widget(sidebar_block, areas.sidebar.inner(Margin::new(1, 1)));
    frame.render_widget(main_block, areas.main.inner(Margin::new(1, 1)));
}

fn render_header(app: &mut App, frame: &mut Frame, area: Rect) {
    let header_title = Paragraph::new(app.title.clone());
    frame.render_widget(header_title, area);
}

fn render_sidebar(app: &mut App, frame: &mut Frame, area: Rect) {
    let mut sidebar_items: Vec<ListItem> = Vec::new();

    for i in &app.pages {
        sidebar_items.push(ListItem::new(i.title.as_str()));
    }

    let sidebar_list = List::new(sidebar_items)
        .block(
            Block::default()
                .borders(Borders::TOP)
                .title(" - components ")
                .title_alignment(Alignment::Left),
        )
        .highlight_style(Style::default().bg(BG_COLOR_2));

    frame.render_stateful_widget(sidebar_list, area, &mut app.sidebar_state);
}

fn render_main_content(app: &mut App, frame: &mut Frame, area: Rect) {
    // Add your main content rendering here
    let block = Block::default()
        .borders(Borders::TOP)
        .title(" - main content ")
        .title_alignment(Alignment::Left);

    frame.render_widget(block, area.inner(Margin::new(2, 0)));

    let cur_page_id = &app.sidebar_state.selected();
    let text = &app.pages[cur_page_id.unwrap()].content;
    let main_par = Paragraph::new(text.clone()).scroll((app.vertical_scroll as u16, 0));
    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .symbols(scrollbar::DOUBLE_VERTICAL)
            .begin_symbol(None)
            .track_symbol(Some("|"))
            .end_symbol(None),
        area.inner(Margin {
            vertical: 1,
            horizontal: 0,
        }),
        &mut app.vertical_scroll_state,
    );

    app.vertical_scroll_state = app.vertical_scroll_state.content_length(text.len() / 2);

    frame.render_widget(main_par, area.inner(Margin::new(2, 2)));
}

fn render_footer(app: &mut App, frame: &mut Frame, area: Rect) {
    if app.current_screen == CurrentScreen::Main {
        let par = Paragraph::new("\n  right-left keys: move around tabs. up-down keys scroll");

        frame.render_widget(par, area);
    }
}
