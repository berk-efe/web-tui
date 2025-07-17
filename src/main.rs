use ratatui::{Frame, Terminal};
use ratzilla::{DomBackend, WebRenderer};
use std::{cell::RefCell, io, rc::Rc};

mod app;
mod ui;

use app::App;

fn main() -> io::Result<()> {
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    let state = Rc::new(RefCell::new(App::default()));

    let event_state = Rc::clone(&state);
    terminal.on_key_event(move |key_event| {
        event_state.borrow_mut().handle_events(key_event);
    });

    let render_state = Rc::clone(&state);
    terminal.draw_web(move |frame| {
        render_state.borrow_mut().render(frame);
    });

    Ok(())
}

impl<'a> App<'a> {
    fn render(&mut self, frame: &mut Frame) {
        ui::render(self, frame);
    }
}
