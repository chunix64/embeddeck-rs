use heapless::String;

use crate::models::units::Temperature;

#[derive(Default)]
pub struct Weather {
    pub geo_location: Option<String<32>>,
    pub temperature: Option<Temperature>,
    pub apparent_temperature: Option<Temperature>,
    pub relative_humidity: Option<u8>,
}
