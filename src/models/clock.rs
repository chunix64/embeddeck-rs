use chrono::{DateTime, TimeZone, Timelike, Utc};
use chrono_tz::Tz;
use esp_hal::rtc_cntl::Rtc;

pub struct Clock {
    time_zone: Tz,
    rtc: &'static Rtc<'static>,
}

impl Clock {
    pub fn new(time_zone: Tz, rtc: &'static Rtc<'static>) -> Self {
        Self { time_zone, rtc }
    }

    pub fn default(rtc: &'static Rtc<'static>) -> Self {
        // Suggest: use release date for each version as default_date
        let default_us = Tz::UTC
            .with_ymd_and_hms(2026, 5, 10, 12, 0, 0)
            .unwrap()
            .timestamp_micros() as u64;
        let time_zone = chrono_tz::UTC;

        rtc.set_current_time_us(default_us);

        Self::new(time_zone, rtc)
    }

    fn now(&self) -> DateTime<Tz> {
        let us = self.rtc.current_time_us() as i64;
        Utc.timestamp_micros(us)
            .single()
            .expect("RTC returned invalid timestamp")
            .with_timezone(&self.time_zone)
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
