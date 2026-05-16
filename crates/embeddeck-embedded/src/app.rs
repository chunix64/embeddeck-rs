use embassy_executor::Spawner;

use crate::actors::ui::ui_actor;
use crate::hardwares::display::display_controller::DisplayController;
use crate::models::state::AppState;

#[embassy_executor::task]
pub async fn app_task(
    spawner: Spawner,
    display: &'static mut DisplayController,
    app_state: &'static AppState,
) {
    spawner.spawn(ui_actor(display, app_state).unwrap());
}
