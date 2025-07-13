// ANCHOR: all
use std::collections::HashMap;

use ratatui::{
    text::Text,
    widgets::{ListState, Paragraph},
};

// ANCHOR: screen_modes
pub enum CurrentScreen {
    Start,
    Main,
    Exiting,
}

pub struct Component {
    pub title: &'static str,
    pub content: Text<'static>,
}

impl Component {
    pub fn new(title: &'static str, content: Text<'static>) -> Self {
        Self {
            title,
            content: content,
        }
    }
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub current_component: Option<usize>,
    pub components: Vec<Component>,

    pub sidebar_state: ListState,
    pub boot_index: usize,
}
// ANCHOR_END: app_fields

// ANCHOR: impl_new
impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Start,
            current_component: None,
            components: Vec::new(),

            sidebar_state: ListState::default(),
            boot_index: 0,
        }
    }
    // ANCHOR_END: impl_new
}
// ANCHOR_END: all
