use esp_hal::{gpio::AnyPin, peripherals::LEDC, spi::master::AnySpi};

pub struct AppPeripherals {
    pub ledc: LEDC<'static>,
    pub spi: AnySpi<'static>,
}

pub struct DisplayPins {
    pub sck: AnyPin<'static>,
    pub mosi: AnyPin<'static>,
    pub dc: AnyPin<'static>,
    pub cs: AnyPin<'static>,
    pub rst: AnyPin<'static>,
}

pub struct BacklightConfig {
    pub pin: AnyPin<'static>,
}

pub struct DisplayConfig {
    pub display_width: u16,
    pub display_height: u16,
    pub pins: DisplayPins,
}
