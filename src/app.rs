use embassy_executor::Spawner;
use embassy_time::Delay;
use embedded_hal_async::delay::DelayNs;

use crate::{
    hardware::display::display_controller::DisplayController, models::clock::Clock,
    ui::actor::ui_task,
};

#[embassy_executor::task]
pub async fn app_task(
    spawner: Spawner,
    display: &'static mut DisplayController,
    clock: &'static Clock,
) {
    spawner.spawn(ui_task(display, clock).unwrap());

    loop {
        Delay.delay_ms(1000).await;
    }
}
