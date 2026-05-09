use embedded_hal::digital::OutputPin;
use mipidsi::{
    interface::{Interface, InterfacePixelFormat},
    models::Model,
};

use crate::backlight::ledc::BacklightController;

pub struct DisplayController<'a, DI, MODEL, RST>
where
    DI: Interface + 'static,
    MODEL: Model + 'static,
    MODEL::ColorFormat: InterfacePixelFormat<DI::Word>,
    RST: OutputPin + 'static,
{
    display: mipidsi::Display<DI, MODEL, RST>,
    backlight: Option<BacklightController<'a>>,
}

impl<'a, DI, MODEL, RST> DisplayController<'a, DI, MODEL, RST>
where
    DI: Interface + 'static,
    MODEL: Model + 'static,
    MODEL::ColorFormat: InterfacePixelFormat<DI::Word>,
    RST: OutputPin + 'static,
{
    pub fn new(
        display: mipidsi::Display<DI, MODEL, RST>,
        backlight: Option<BacklightController<'a>>,
    ) -> Self {
        Self { display, backlight }
    }

    pub fn init(&mut self) {
        self.set_min_brightness(1);
        self.set_brightness(100);
    }

    pub fn raw_display(&mut self) -> &mut mipidsi::Display<DI, MODEL, RST> {
        &mut self.display
    }

    pub fn set_brightness(&mut self, brightness: u8) {
        if let Some(backlight) = &mut self.backlight {
            backlight.set_brightness(brightness);
        }
    }

    pub fn set_min_brightness(&mut self, min_brightness: u8) {
        if let Some(backlight) = &mut self.backlight {
            backlight.set_min_brightness(min_brightness);
        }
    }
}
