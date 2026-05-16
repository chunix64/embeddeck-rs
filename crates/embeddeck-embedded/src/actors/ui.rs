use embassy_time::Delay;
use embeddeck_ui::screens::default::DefaultScreen;
use embedded_hal_async::delay::DelayNs;
use mousefood::{EmbeddedBackend, EmbeddedBackendConfig};
use ratatui::{Frame, Terminal};

use crate::{hardwares::display::display_controller::DisplayController, models::state::AppState};

#[embassy_executor::task]
pub async fn ui_actor(display: &'static mut DisplayController, app_state: &'static AppState) {
    display.init();
    display.rotate_landscape();

    let backend = EmbeddedBackend::new(display.raw_display(), EmbeddedBackendConfig::default());
    let mut terminal = Terminal::new(backend).unwrap();

    loop {
        let _ = terminal.draw(|frame| render_ui(frame, app_state));
        Delay.delay_ms(1000).await;
    }
}

// Add choosing layout/ui/theme logic later
fn render_ui(frame: &mut Frame, app_state: &AppState) {
    let area = frame.area();

    frame.render_widget(DefaultScreen::new(app_state.clock), area);
}
