use crate::app::{App, CurrentScreen};
use ratatui::Frame;

mod demo_screen;
mod main_screen;
mod start_screen;

pub fn render(app: &mut App, frame: &mut Frame) {
    match app.current_screen {
        CurrentScreen::Start => start_screen::render(app, frame),
        CurrentScreen::Main => main_screen::render(app, frame),
        CurrentScreen::Demo => demo_screen::render(app, frame),
        _ => {}
    }
}

