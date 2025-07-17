use ratatui::{
    prelude::Stylize,
    style::Style,
    text::{Line, Span},
    widgets::{ListState, Paragraph},
};
use ratzilla::event::{KeyCode, KeyEvent};

#[derive(PartialEq)]
pub enum CurrentScreen {
    Start,
    Main,
    Demo,
    Exiting,
}

pub struct Page<'a> {
    pub title: String,
    pub content: Paragraph<'a>,
}

impl<'a> Page<'a> {
    pub fn new(title: String, content: Paragraph<'a>) -> Self {
        Self {
            title,
            content: content,
        }
    }
}

pub struct App<'a> {
    pub counter: u8,
    pub current_screen: CurrentScreen,
    pub sidebar_state: ListState,

    pub title: String,
    pub pages: Vec<Page<'a>>,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        let mut sidebar_state = ListState::default();
        sidebar_state.select(Some(0));

        App {
            counter: 0,
            current_screen: CurrentScreen::Main,
            sidebar_state,
            title: String::from("\n  Berk Efe Keskin v1.0"),

            pages: vec![
                Page::new(
                    String::from("  Home"),
                    Paragraph::new(vec![
                        Line::from(Span::styled("My Awesome App", Style::new().bold())),
                        Line::from(""),
                        Line::from("This application demonstrates how to build a "),
                        Line::from("terminal UI using the ratatui crate.  The text "),
                        Line::from("wraps automatically to the width of the block."),
                        Line::from(""),
                        Line::from(Span::styled("Features", Style::new().bold())),
                        Line::from(""),
                        Line::from("• Sidebar navigation"),
                        Line::from("• Responsive layout"),
                        Line::from(vec![
                            Span::raw("• "),
                            Span::styled("Rich text ", Style::new().bold()),
                            Span::raw("with colours & styles"),
                        ]),
                        Line::from(""),
                        Line::from("Scroll down to read more…"),
                    ]),
                ),
                Page::new(
                    String::from("  Projects"),
                    Paragraph::new(vec![Line::from("Hello")]),
                ),
                Page::new(
                    String::from("  ETC"),
                    Paragraph::new(vec![Line::from("Hello")]),
                ),
            ],
        }
    }
}

impl<'a> App<'a> {
    pub fn handle_events(&mut self, key_event: KeyEvent) {
        match self.current_screen {
            CurrentScreen::Main => match key_event.code {
                KeyCode::Up => self.sidebar_state.select_previous(),
                KeyCode::Down => self.sidebar_state.select_next(),
                KeyCode::Left => self.sidebar_state.select_first(),
                KeyCode::Right => self.sidebar_state.select_last(),
                KeyCode::Char('d') => self.current_screen = CurrentScreen::Demo,
                _ => {}
            },
            CurrentScreen::Demo => match key_event.code {
                KeyCode::Left => self.counter = self.counter.saturating_sub(1),
                KeyCode::Right => self.counter = self.counter.saturating_add(1),
                KeyCode::Char('m') => self.current_screen = CurrentScreen::Main,
                _ => {}
            },
            _ => {}
        }
    }
}

