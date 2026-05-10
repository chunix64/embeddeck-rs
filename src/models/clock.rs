use chrono::{DateTime, TimeZone, Timelike};
use chrono_tz::Tz;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};

struct ClockInner {
    now: DateTime<Tz>,
    time_zone: Tz,
}

pub struct Clock {
    inner: Mutex<CriticalSectionRawMutex, ClockInner>,
}

impl Clock {
    pub fn new(now: DateTime<Tz>, time_zone: Tz) -> Self {
        Self {
            inner: Mutex::new(ClockInner { now, time_zone }),
        }
    }

    pub fn default() -> Self {
        // Suggest: use release date for each version as default_date
        let default_date = Tz::UTC.with_ymd_and_hms(2026, 5, 10, 12, 0, 0).unwrap();
        let time_zone = chrono_tz::UTC;

        Self::new(default_date, time_zone)
    }

    pub async fn now(&self) -> DateTime<Tz> {
        let inner = self.inner.lock().await;
        inner.now.with_timezone(&inner.time_zone)
    }

    pub async fn hour(&self) -> u32 {
        self.now().await.hour()
    }

    pub async fn minute(&self) -> u32 {
        self.now().await.minute()
    }

    pub async fn second(&self) -> u32 {
        self.now().await.second()
    }

    pub async fn set_time(&self, new_time: DateTime<Tz>) {
        let mut inner = self.inner.lock().await;
        inner.now = new_time;
    }
}
