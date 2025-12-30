use std::time::Duration;

const SECONDS_IN_ONE_HOUR: u64 = 3600;
const SECONDS_IN_ONE_MINUTE: u64 = 60;

pub struct HourAndMinute {
    pub hour: i32,
    pub minute: i32,
}

impl From<Duration> for HourAndMinute {
    fn from(duration: Duration) -> Self {
        let seconds = duration.as_secs();
        let hour = (seconds / SECONDS_IN_ONE_HOUR) as i32;
        let minute = ((seconds % SECONDS_IN_ONE_MINUTE) / 60) as i32;
        Self { hour, minute }
    }
}

impl From<HourAndMinute> for Duration {
    fn from(value: HourAndMinute) -> Self {
        Duration::from_secs(
            value.hour as u64 * SECONDS_IN_ONE_HOUR + value.minute as u64 * SECONDS_IN_ONE_MINUTE,
        )
    }
}
