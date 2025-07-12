// ANCHOR: all
use std::collections::HashMap;

use ratatui::widgets::{ListState, Paragraph};

// ANCHOR: screen_modes
pub enum CurrentScreen {
    Start,
    Main,
    Sidebar,
}

pub struct Component {
    pub title: &'static str,
    pub content: Paragraph<'static>,
}

impl Component {
    pub fn new(title: &'static str, content: Paragraph<'static>) -> Self {
        Self {
            title,
            content: content,
        }
    }
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub current_component: Option<Component>,
    pub components: Vec<Component>,

    pub sidebar_state: ListState,
}
// ANCHOR_END: app_fields

// ANCHOR: impl_new
impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Sidebar,
            current_component: None,
            components: Vec::new(),

            sidebar_state: ListState::default(),
        }
    }
    // ANCHOR_END: impl_new
}
// ANCHOR_END: all
