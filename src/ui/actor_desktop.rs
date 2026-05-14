#![cfg(feature = "desktop")]

use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;
use ratatui::Frame;
use std::time::Duration;

use crate::{models::clock::Clock, ui::screens::default::DefaultScreen};

pub fn ui_task(terminal: &mut DefaultTerminal, clock: &Clock) -> std::io::Result<()> {
    let tick_rate = Duration::from_millis(500);

    loop {
        terminal.draw(|frame| render_ui(frame, clock))?;

        if event::poll(tick_rate)? {
            if let Event::Key(_) = event::read()? {
                break Ok(());
            }
        }
    }
}

// Add choosing layout/ui/theme logic later
fn render_ui(frame: &mut Frame, clock: &Clock) {
    let area = frame.area();

    frame.render_widget(DefaultScreen::new(clock), area);
}
