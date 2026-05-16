use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embeddeck_core::models::weather::Weather;
use esp_hal::rtc_cntl::Rtc;
use static_cell::StaticCell;

use crate::models::clock::{EmbeddedClock, EmbeddedClockExt};

static CLOCK: StaticCell<EmbeddedClock> = StaticCell::new();
static WEATHER: Mutex<CriticalSectionRawMutex, Option<Weather>> = Mutex::new(None);

pub struct AppState {
    pub clock: &'static EmbeddedClock,
    pub weather: &'static Mutex<CriticalSectionRawMutex, Option<Weather>>,
}

impl AppState {
    pub async fn init(rtc: &'static Rtc<'static>) -> Self {
        *WEATHER.lock().await = Some(Weather::default());

        Self {
            clock: CLOCK.init(EmbeddedClock::default(rtc)),
            weather: &WEATHER,
        }
    }
}
