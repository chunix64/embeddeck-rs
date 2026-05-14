#![cfg(feature = "desktop")]
extern crate alloc;
extern crate std;

mod models;
mod ui;

use crate::models::clock::Clock;
use crate::ui::actor_desktop::ui_task;

fn main() -> std::io::Result<()> {
    let clock = Clock::default();
    ratatui::run(|terminal| ui_task(terminal, &clock))
}
