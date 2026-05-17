#[derive(Default)]
pub struct GeoLocation<const N: usize = 32> {
    pub latitude: f32,
    pub longitude: f32,
    pub city_name: Option<heapless::String<N>>,
}
