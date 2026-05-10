use chrono::{DateTime, TimeZone, Timelike};
use chrono_tz::Tz;
use esp_hal::time::Instant;

pub struct Clock {
    base_time: DateTime<Tz>,
    base_instant: Instant,
    time_zone: Tz,
}

impl Clock {
    pub fn new(base_time: DateTime<Tz>, time_zone: Tz) -> Self {
        Self {
            base_time,
            base_instant: Instant::now(),
            time_zone,
        }
    }

    pub fn default() -> Self {
        // Suggest: use release date for each version as default_date
        let default_date = Tz::UTC.with_ymd_and_hms(2026, 5, 10, 12, 0, 0).unwrap();
        let time_zone = chrono_tz::UTC;

        Self::new(default_date, time_zone)
    }

    pub fn now(&self) -> DateTime<Tz> {
        let elapsed = Instant::now() - self.base_instant;
        let elapsed_chrono = chrono::TimeDelta::microseconds(elapsed.as_micros() as i64);
        (self.base_time + elapsed_chrono).with_timezone(&self.time_zone)
    }

    pub fn hour(&self) -> u32 {
        self.now().hour()
    }

    pub fn minute(&self) -> u32 {
        self.now().minute()
    }

    pub fn second(&self) -> u32 {
        self.now().second()
    }
}
