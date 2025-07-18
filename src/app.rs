use ratatui::{
    prelude::Stylize,
    style::Style,
    text::{Line, Span},
    widgets::{ListState, Paragraph, Scrollbar, ScrollbarState},
};
use ratzilla::event::{KeyCode, KeyEvent};

#[derive(PartialEq)]
pub enum CurrentScreen {
    Start,
    Main,
    Demo,
    Exiting,
}

#[derive(PartialEq)]
pub enum CurrentPage {
    Home,
    Projects,
    AboutMe,
}

pub struct Page {
    pub title: String,
    pub page_type: CurrentPage,
}

impl Page {
    pub fn new(title: String, page_type: CurrentPage) -> Self {
        Self {
            title: title,
            page_type: page_type,
        }
    }
}

pub struct App {
    pub counter: u8,
    pub current_screen: CurrentScreen,
    pub sidebar_state: ListState,

    pub current_page: CurrentPage,
    pub pages: Vec<Page>,

    pub vertical_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,

    pub boot_text_id: usize,
    pub boot_index: usize,
    pub frame_count: usize,

    pub title: String,
}

impl Default for App {
    fn default() -> Self {
        let mut sidebar_state = ListState::default();
        sidebar_state.select(Some(0));

        let pages = vec![
            Page::new("  Home".to_string(), CurrentPage::Home),
            Page::new("  Projects".to_string(), CurrentPage::Projects),
            Page::new("  About Me".to_string(), CurrentPage::AboutMe),
        ];

        App {
            counter: 0,
            current_screen: CurrentScreen::Main,
            sidebar_state,

            current_page: CurrentPage::Home,
            pages: pages,

            vertical_scroll_state: ScrollbarState::default(),
            vertical_scroll: usize::default(),

            boot_text_id: usize::default(),
            boot_index: usize::default(),
            frame_count: usize::default(),

            title: String::from("\n  Berk Efe Keskin v1.0"),
        }
    }
}

impl App {
    pub fn handle_events(&mut self, key_event: KeyEvent) {
        match self.current_screen {
            CurrentScreen::Main => match key_event.code {
                KeyCode::Up => {
                    self.sidebar_state.select_previous();
                }
                KeyCode::Down => {
                    self.sidebar_state.select_next();
                }
                KeyCode::Left => {}
                KeyCode::Right => {}
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
