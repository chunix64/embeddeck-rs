use heapless::String;

use crate::models::units::Temperature;

pub struct Weather {
    pub geo_location: String<32>,
    pub temperature: Temperature,
    pub apparent_temperature: Temperature,
    pub relative_humidity: u8,
}

impl Default for Weather {
    fn default() -> Self {
        Self {
            geo_location: heapless::String::new(),
            temperature: Temperature::Celsius(f32::NAN),
            apparent_temperature: Temperature::Celsius(f32::NAN),
            relative_humidity: 255,
        }
    }
}
