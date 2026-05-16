pub enum Temperature {
    Celsius(f32),
    // Fahrenheit(f32),
}

impl Temperature {
    pub fn celsius(value: f64) -> Self {
        Temperature::Celsius(value as f32)
    }
}
