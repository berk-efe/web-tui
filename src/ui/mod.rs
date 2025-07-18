use crate::app::{App, CurrentScreen};
use ratatui::Frame;

mod about_me_page;
mod demo_screen;
mod home_page;
mod main_screen;
mod projects_page;
mod start_screen;

pub fn render(app: &mut App, frame: &mut Frame) {
    match app.current_screen {
        CurrentScreen::Start => start_screen::render(app, frame),
        CurrentScreen::Main => main_screen::render(app, frame),
        CurrentScreen::Demo => demo_screen::render(app, frame),
        _ => {}
    }
}
