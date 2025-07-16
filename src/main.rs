use std::{cell::RefCell, io, rc::Rc};

use ratatui::{
    layout::{
        Alignment, Constraint,
        Direction::{Horizontal, Vertical},
        Layout, Margin,
    },
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};

use ratzilla::{
    event::{KeyCode, KeyEvent},
    DomBackend, WebRenderer,
};

#[macro_export]
macro_rules! margin {
    ($n:expr) => {
        Margin::new($n, $n)
    };

    ($h:expr, $v:expr) => {
        Margin::new($h, $v)
    };
}

fn main() -> io::Result<()> {
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    let state = Rc::new(App::default());

    let event_state = Rc::clone(&state);
    terminal.on_key_event(move |key_event| {
        event_state.handle_events(key_event);
    });

    let render_state = Rc::clone(&state);
    terminal.draw_web(move |frame| {
        render_state.render(frame);
    });

    Ok(())
}

#[derive(Default, PartialEq)]
enum CurrentScreen {
    Start,
    #[default]
    Main,
    Demo,
    Exiting,
}

struct Page<'a> {
    title: String,
    content: Vec<Line<'a>>,
}

#[derive(Default)]
struct App {
    counter: RefCell<u8>,
    current_screen: CurrentScreen,

    sidebar_state: RefCell<ListState>,
}

impl App {
    fn render(&self, frame: &mut Frame) {
        const BG_COLOR_0: Color = Color::Rgb(0, 19, 45);
        const BG_COLOR_1: Color = Color::Rgb(0, 38, 87);
        const BG_COLOR_2: Color = Color::Rgb(0, 55, 126);

        // MAIN
        if self.current_screen == CurrentScreen::Main {
            let block = Block::default().bg(BG_COLOR_0).borders(Borders::NONE);

            frame.render_widget(block, frame.area());

            let chunks = Layout::default()
                .direction(Vertical)
                .vertical_margin(5)
                .horizontal_margin(35)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(3),
                ])
                .split(frame.area());

            let (header, body, footer) = (chunks[0], chunks[1], chunks[2]);

            let header_block = Block::default().bg(BG_COLOR_1).borders(Borders::NONE);

            let footer_block = Block::default().bg(BG_COLOR_1).borders(Borders::NONE);

            frame.render_widget(header_block, header.inner(margin!(1, 0)));
            frame.render_widget(footer_block, footer.inner(margin!(1, 0)));

            let chunks = Layout::default()
                .direction(Horizontal)
                .constraints([Constraint::Length(25), Constraint::Min(1)])
                .split(body);

            let (sidebar, body) = (chunks[0], chunks[1]);

            let sidebar_block = Block::default().bg(BG_COLOR_1).borders(Borders::NONE);

            let body_block = Block::default().bg(BG_COLOR_1).borders(Borders::NONE);

            frame.render_widget(sidebar_block, sidebar.inner(margin!(1, 1)));
            frame.render_widget(body_block, body.inner(margin!(1, 1)));

            // SIDEBAR
            let mut sidebar_list_items: Vec<ListItem> = Vec::new();

            // ADD MANUALLY FOR NOW
            sidebar_list_items.push(ListItem::from("Item 1"));
            sidebar_list_items.push(ListItem::from("Item 2"));
            sidebar_list_items.push(ListItem::from("Item 3"));

            let sidebar_list = List::new(sidebar_list_items)
                .block(
                    Block::default()
                        .borders(Borders::TOP)
                        .title("- components ")
                        .title_alignment(Alignment::Left),
                )
                .highlight_style(Style::default().bg(BG_COLOR_2));

            frame.render_widget(sidebar_list, sidebar.inner(margin!(2, 0)));

            // MAIN

            // ELSE
        } else if self.current_screen == CurrentScreen::Demo {
            let block = Block::bordered()
                .title("web0")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded);

            let text = format!(
                "This is a Ratzilla template.\n\
             Press left and right to increment and decrement the counter respectively.\n",
            );

            let paragraph = Paragraph::new(text)
                .block(block)
                .fg(Color::White)
                .bg(Color::Black)
                .centered();

            frame.render_widget(paragraph, frame.area());
        }
    }

    fn handle_events(&self, key_event: KeyEvent) {
        if self.current_screen == CurrentScreen::Main {
            let mut sidebar_state = self.sidebar_state.borrow_mut();
            match key_event.code {
                KeyCode::Up => sidebar_state.select_previous(),
                KeyCode::Down => sidebar_state.select_next(),

                KeyCode::Left => sidebar_state.select_first(),
                KeyCode::Right => sidebar_state.select_last(),

                _ => {}
            }
        } else if self.current_screen == CurrentScreen::Demo {
            let mut counter = self.counter.borrow_mut();
            match key_event.code {
                KeyCode::Left => *counter = counter.saturating_sub(1),
                KeyCode::Right => *counter = counter.saturating_add(1),
                _ => {}
            }
        }
    }
}
