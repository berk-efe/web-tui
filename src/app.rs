use std::collections::HashMap;

use ratzilla::ratatui::{
    text::Text,
    widgets::{ListState, Paragraph},
};

use crate::helper;

// ANCHOR: screen_modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub boot_text_id: usize,
    pub frame_count: usize,
}
// ANCHOR_END: app_fields

// ANCHOR: impl_new
impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            current_component: None,
            components: Vec::new(),

            sidebar_state: ListState::default(),
            boot_index: 0,
            boot_text_id: 0,
            frame_count: 0,
        }
    }
    // ANCHOR_END: impl_new
}
// ANCHOR_END: all
