use chrono::{DateTime, TimeZone, Timelike, Utc};
use chrono_tz::Tz;

pub struct Clock {
    now: DateTime<Utc>,
    time_zone: Tz,
}

impl Clock {
    pub fn new(now: DateTime<Utc>, time_zone: Tz) -> Self {
        Self { now, time_zone }
    }

    pub fn default() -> Self {
        // Suggest: use release date for each version as default_date
        let default_date = Utc.with_ymd_and_hms(2026, 5, 10, 12, 0, 0).unwrap();
        let time_zone = chrono_tz::UTC;

        Self::new(default_date, time_zone)
    }

    pub fn now(&self) -> DateTime<Tz> {
        self.now.with_timezone(&self.time_zone)
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
