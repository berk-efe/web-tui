// ANCHOR: all
use std::{error::Error, io, time::Duration};

use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    style::{Style, Stylize},
    text::{Line, Span, Text},
    widgets::Paragraph,
};

mod app;
mod helper;
mod ui;

use crate::{
    app::{App, Component, CurrentScreen},
    ui::ui,
};

// ANCHOR: main_all
// ANCHOR: setup_boilerplate
fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    // ANCHOR_END: setup_boilerplate
    // ANCHOR: application_startup
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);
    // ANCHOR_END: application_startup

    // ANCHOR: ending_boilerplate
    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    // ANCHOR_END: ending_boilerplate

    // ANCHOR: final_print

    Ok(())
}
// ANCHOR_END: final_print
// ANCHOR_END: main_all

// ANCHOR: run_app_all
// ANCHOR: run_method_signature
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    // ANCHOR_END: run_method_signature
    // ANCHOR: ui_loop

    app.components = vec![
        // HOME
        //
        Component::new(
            "  Home",
            Text::from(vec![
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
        // ABOUT
        //
        Component::new("  About", Text::from("  Hello")),
        // ETC
        //
        Component::new("  ETC", Text::from("  Hi!")),
    ];

    app.current_component = Some(0);
    app.sidebar_state.select(Some(0));

    loop {
        terminal.draw(|f| ui(f, app))?;
        // ANCHOR_END: ui_loop

        // ANCHOR: event_poll
        // ANCHOR: main_screen
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    // Skip events that are not KeyEventKind::Press
                    continue;
                }
                match app.current_screen {
                    CurrentScreen::Start => match key.code {
                        KeyCode::Char('q') => return Ok(true),
                        KeyCode::Enter => app.current_screen = CurrentScreen::Main,

                        _ => {}
                    },

                    CurrentScreen::Main => match key.code {
                        KeyCode::Char('e') => {
                            println!("Pressed 'e'");
                        }
                        KeyCode::Char('q') => return Ok(true),
                        KeyCode::Tab => {
                            app.sidebar_state.select_next();
                            app.current_component = app.sidebar_state.selected();
                        }
                        KeyCode::BackTab => {
                            app.sidebar_state.select_previous();
                            app.current_component = app.sidebar_state.selected();
                        }

                        _ => {}
                    },

                    _ => {}
                }
            }
        }
        // ANCHOR_END: event_poll
    }
}
// ANCHOR: run_app_all

// ANCHOR_END: all
