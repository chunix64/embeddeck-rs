use embassy_executor::Spawner;
use embassy_time::Delay;
use embedded_hal::digital::OutputPin;
use embedded_hal_async::delay::DelayNs;
use mipidsi::{
    interface::{Interface, InterfacePixelFormat},
    models::Model,
};
use mousefood::{EmbeddedBackend, EmbeddedBackendConfig};
use ratatui::Terminal;

use crate::{display::display_controller::DisplayController, ui::actor::UIActor};

pub struct App<'a, DI, MODEL, RST>
where
    DI: Interface + 'static,
    MODEL: Model + 'static,
    MODEL::ColorFormat: InterfacePixelFormat<DI::Word>,
    RST: OutputPin + 'static,
{
    display: DisplayController<'a, DI, MODEL, RST>,
}

#[allow(clippy::large_stack_frames)]
impl<'a, DI, MODEL, RST> App<'a, DI, MODEL, RST>
where
    DI: Interface + 'static,
    MODEL: Model + 'static,
    MODEL::ColorFormat: InterfacePixelFormat<DI::Word>,
    MODEL: Model<ColorFormat = embedded_graphics::pixelcolor::Rgb565>,
    RST: OutputPin + 'static,
{
    pub fn new(display: DisplayController<'a, DI, MODEL, RST>) -> Self {
        Self { display }
    }

    pub async fn run(&mut self, spawner: Spawner) -> ! {
        // TODO: Refactor with embassy_executor later
        let _ = spawner;
        self.display.init();
        let backend =
            EmbeddedBackend::new(self.display.raw_display(), EmbeddedBackendConfig::default());
        let terminal = Terminal::new(backend).unwrap();

        let mut ui_actor = UIActor::new(terminal);

        loop {
            ui_actor.run().await;
            Delay.delay_ms(500);
        }
    }
}
