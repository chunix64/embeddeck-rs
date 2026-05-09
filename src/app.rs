use embassy_time::Delay;
use embedded_graphics::{
    Drawable,
    image::{Image, ImageRawLE},
    pixelcolor::Rgb565,
};
use embedded_hal::digital::OutputPin;
use embedded_hal_async::delay::DelayNs;
use log::info;
use mipidsi::{
    interface::{Interface, InterfacePixelFormat},
    models::Model,
    options::{Orientation, Rotation},
};

use crate::backlight::ledc::BacklightController;

pub struct App<'a, DI, MODEL, RST>
where
    DI: Interface,
    MODEL: Model,
    MODEL::ColorFormat: InterfacePixelFormat<DI::Word>,
    RST: OutputPin,
{
    display: mipidsi::Display<DI, MODEL, RST>,
    backlight_controller: Option<BacklightController<'a>>,
    delay: Delay,
}

impl<'a, DI, MODEL, RST> App<'a, DI, MODEL, RST>
where
    DI: Interface,
    MODEL: Model,
    MODEL::ColorFormat: InterfacePixelFormat<DI::Word>,
    MODEL: Model<ColorFormat = embedded_graphics::pixelcolor::Rgb565>,
    RST: OutputPin,
{
    pub fn new(
        display: mipidsi::Display<DI, MODEL, RST>,
        backlight_controller: Option<BacklightController<'a>>,
        delay: Delay,
    ) -> Self {
        Self {
            display,
            backlight_controller,
            delay,
        }
    }

    pub async fn run(&mut self) -> ! {
        if let Some(backlight_controller) = &mut self.backlight_controller {
            backlight_controller.set_min_brightness(1);
            backlight_controller.set_brightness(100);
        }

        let raw_images: [(ImageRawLE<Rgb565>, Orientation); 2] = [
            (
                ImageRawLE::new(include_bytes!("./assets/images/jesus.raw"), 320),
                Orientation::new().rotate(Rotation::Deg90),
            ),
            (
                ImageRawLE::new(include_bytes!("./assets/images/ai.raw"), 240),
                Orientation::new().rotate(Rotation::Deg0),
            ),
        ];

        let mut current_index: usize = raw_images.len() - 1;

        info!("App started!");

        loop {
            current_index = (current_index + 1) % raw_images.len();
            let (raw_image, orientation) = &raw_images[current_index];
            self.display.set_orientation(*orientation).unwrap();
            let image = Image::new(raw_image, embedded_graphics::prelude::Point::new(0, 0));
            image.draw(&mut self.display).unwrap();
            self.delay.delay_ms(1000).await;
        }
    }
}
