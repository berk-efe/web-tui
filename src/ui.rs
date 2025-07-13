// ANCHOR: all
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Padding, Paragraph, Wrap, block::Title},
};

use std::thread;
use std::time::Duration;

use crate::app::{App, CurrentScreen};
use crate::helper::FIRST_BOOT_TEXT_LIST;

// ANCHOR: method_sig
pub fn ui(frame: &mut Frame, app: &mut App) {
    if let CurrentScreen::Start = &app.current_screen {
        thread::sleep(Duration::from_millis(rand::random_range(50..250)));

        if app.boot_index >= FIRST_BOOT_TEXT_LIST.len() {
            app.current_screen = CurrentScreen::Main;
            return;
        }

        let par = Paragraph::new(
            FIRST_BOOT_TEXT_LIST
                .iter()
                .enumerate()
                .filter(|(i, _)| *i <= app.boot_index)
                .map(|(_, e)| Line::from(*e))
                .collect::<Vec<_>>(),
        );
        frame.render_widget(par, frame.area());

        app.boot_index += 1;
        return;
    }

    const BG_COLOR_0: Color = Color::Rgb(0, 19, 45);
    const BG_COLOR_1: Color = Color::Rgb(0, 38, 87);
    const BG_COLOR_2: Color = Color::Rgb(0, 55, 126);

    let main_border = Block::default().borders(Borders::NONE).bg(BG_COLOR_0);

    frame.render_widget(main_border, frame.area());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(5)
        .horizontal_margin(35)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());
    let (header, bottom, footer) = (chunks[0], chunks[1], chunks[2]);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(15), Constraint::Percentage(85)])
        .split(bottom);

    let (left_sidebar, main_content) = (main_chunks[0], main_chunks[1]);

    // HEADER
    //
    let header_p = Paragraph::new("\n Berk Efe Keskin v1.0")
        .alignment(Alignment::Left)
        .block(Block::default().bg(BG_COLOR_1));

    frame.render_widget(
        header_p,
        header.inner(Margin {
            horizontal: 1,
            vertical: 0,
        }),
    );

    let sidebar_items: Vec<ListItem> = app
        .components
        .iter()
        .map(|p| ListItem::new(p.title))
        .collect();

    let space = Margin {
        vertical: 1,
        horizontal: 1,
    };

    let hor_space = Margin {
        vertical: 0,
        horizontal: 1,
    };

    let sidebar = List::new(sidebar_items)
        .highlight_style(Style::default().bg(BG_COLOR_2))
        .block(
            Block::default()
                .borders(Borders::TOP)
                .title(" components ")
                .title_alignment(Alignment::Left)
                .bg(BG_COLOR_1),
        );

    frame.render_stateful_widget(sidebar, left_sidebar.inner(space), &mut app.sidebar_state);

    // MAIN
    //

    let mut _content: Text = Text::from("");

    if let Some(curr_comp_id) = app.current_component {
        let comp = &app.components[curr_comp_id];
        _content = comp.content.clone();
    } else {
    }

    let main_paragraph = Paragraph::new(_content).block(
        Block::default()
            .borders(Borders::TOP)
            .title(" main ")
            .title_alignment(Alignment::Left)
            .bg(BG_COLOR_1),
    );

    frame.render_widget(main_paragraph, main_content.inner(space));

    // FOOTER
    //

    let key_hints = match app.current_screen {
        CurrentScreen::Main => {
            Span::styled(" (q)uit / Tab: next / BackTab: prev", Style::default())
        }
        _ => Span::from(""),
    };

    let key_block = Block::default().borders(Borders::NONE).bg(BG_COLOR_1);
    frame.render_widget(key_block, footer.inner(hor_space));

    let key_p = Paragraph::new(Line::from(key_hints));

    let centered_key_area = center_vertical(footer, 1);

    frame.render_widget(key_p, centered_key_area.inner(hor_space));
}

fn center_vertical(area: Rect, height: u16) -> Rect {
    let [area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    area
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
// ANCHOR_END: centered_rect

// ANCHOR_END: all
