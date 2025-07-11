// ANCHOR: all
use std::collections::HashMap;

// ANCHOR: screen_modes
pub enum CurrentScreen {
    Main,
}
// ANCHOR_END: screen_modes

// ANCHOR: currently_editing

// ANCHOR_END: currently_editing

// ANCHOR: app_fields
pub struct App {
    current_screen: CurrentScreen,
}
// ANCHOR_END: app_fields

// ANCHOR: impl_new
impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
        }
    }
    // ANCHOR_END: impl_new
}
// ANCHOR_END: all
