#![cfg(feature = "desktop")]
extern crate std;
extern crate alloc;

mod models;
mod ui;

use crate::models::clock::Clock;
use crate::ui::actor_desktop::ui_task;

fn main() -> std::io::Result<()> {
    let clock = Clock::default();
    ratatui::run(|terminal| ui_task(terminal, &clock))
}
