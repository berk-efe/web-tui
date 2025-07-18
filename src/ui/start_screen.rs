use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::app::{App, CurrentScreen};
use crate::helper::{FIRST_BOOT_TEXT_LIST, SECOND_BOOT_TEXT_LIST};

pub fn render(app: &mut App, frame: &mut Frame) {
    app.frame_count += 1;
    // Different timing for different boot phases
    let advance_every = if app.boot_text_id == 0 {
        // First boot text: slower, more realistic
        match app.boot_index {
            0..=5 => 20,   // Early hooks: slower
            6..=10 => 15,  // Middle phase: medium
            11..=15 => 40, // Mounting: slower (realistic)
            16..=20 => 5,  // Final phase: faster
            _ => 10,
        }
    } else {
        1
    };

    if app.frame_count % advance_every == 0 {
        if app.boot_index >= FIRST_BOOT_TEXT_LIST.len() && app.boot_text_id == 0 {
            app.boot_text_id += 1;
            app.boot_index = 0;
            app.frame_count = 0;
        } else if app.boot_index >= SECOND_BOOT_TEXT_LIST.len() && app.boot_text_id == 1 {
            app.current_screen = CurrentScreen::Main;
        } else {
            if app.boot_text_id == 1 {
                app.boot_index += 4;
            } else {
                app.boot_index += 1;
            }
        }
    }

    let mut curr_boot_text: &[&str] = &FIRST_BOOT_TEXT_LIST;

    if app.boot_text_id == 1 {
        curr_boot_text = &SECOND_BOOT_TEXT_LIST;
    }

    let view_height = frame.size().height as usize;
    let mut y_offset = 0 as u16;

    if app.boot_index >= view_height {
        y_offset += (app.boot_index - view_height) as u16;
    }

    let par = Paragraph::new(
        curr_boot_text
            .iter()
            .take(app.boot_index + 1)
            .map(|e| Line::from(*e))
            .collect::<Vec<_>>(),
    )
    .scroll((y_offset, 0));

    frame.render_widget(par, frame.area());
}
