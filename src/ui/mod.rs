use ratatui::Frame;
use crate::app::{App, CurrentScreen};

mod main_screen;
mod demo_screen;

pub fn render(app: &mut App, frame: &mut Frame) {
    match app.current_screen {
        CurrentScreen::Main => main_screen::render(app, frame),
        CurrentScreen::Demo => demo_screen::render(app, frame),
        _ => {}
    }
}