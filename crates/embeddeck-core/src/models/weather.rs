use crate::models::units::Temperature;

#[derive(Default)]
pub struct Weather {
    pub temperature: Option<Temperature>,
    pub apparent_temperature: Option<Temperature>,
    pub relative_humidity: Option<u8>,
}
