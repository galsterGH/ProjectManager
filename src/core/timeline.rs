use chrono::{DateTime, TimeDelta, Utc};
use serde::{Deserialize, Serialize};

// TODO: Define the Timeline struct here

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Duration {
    Hours(i64),
    Days(i64),
    Weeks(i64),
}

type DT = DateTime<Utc>;

trait ToTimeDelta {
    fn to_time_delta(&self) -> TimeDelta;
}

impl From<TimeDelta> for Duration {
    fn from(d: TimeDelta) -> Self {
        if d == TimeDelta::zero() {
            return Duration::Hours(0);
        }

        let total_secs = d.num_seconds();
        let sign = if total_secs < 0 { -1 } else { 1 };
        let secs = total_secs.abs();

        const SECS_PER_HOUR: i64 = 3600;
        const SECS_PER_DAY: i64 = 24 * SECS_PER_HOUR;
        const SECS_PER_WEEK: i64 = 7 * SECS_PER_DAY;

        if secs > SECS_PER_WEEK {
            let mut weeks = secs / SECS_PER_WEEK;

            if secs % SECS_PER_WEEK != 0 {
                weeks += 1;
            }

            Duration::Weeks(sign * weeks)
        } else if secs > SECS_PER_DAY {
            let mut days = secs / SECS_PER_DAY;

            if secs % SECS_PER_DAY != 0 {
                days += 1;
            }

            Duration::Days(sign * days)
        } else {
            let mut hours = secs / SECS_PER_HOUR;

            if secs % SECS_PER_HOUR != 0 {
                hours += 1;
            }

            Duration::Hours(sign * hours)
        }
    }
}

impl ToTimeDelta for Duration {
    fn to_time_delta(&self) -> TimeDelta {
        match self {
            Duration::Hours(h) => chrono::Duration::hours(*h),
            Duration::Days(d) => chrono::Duration::days(*d),
            Duration::Weeks(w) => chrono::Duration::weeks(*w),
        }
    }
}

impl Duration {
    pub fn get_duration(start: &DT, end: &DT) -> Self {
        let delta = *end - *start;
        Duration::from(delta)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize,Default)]
pub struct Timeline {
    pub start: DT,
    pub end: Option<DT>,
    pub duration: Option<Duration>,
}

impl Timeline {
    pub fn from_start_end(st: DT, en: DT) -> Self {
        let duration = Duration::get_duration(&st, &en);
        Timeline {
            start: st,
            end: Some(en),
            duration: Some(duration),
        }
    }

    pub fn from_start_duration(st: DT, duration: Duration) -> Self {
        let td: TimeDelta = duration.to_time_delta();
        let end = st + td;
        Timeline {
            start: st,
            end: Some(end),
            duration: Some(duration),
        }
    }
}
