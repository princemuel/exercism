use core::fmt;

const ONE_DAY: i64 = 24 * 60;
const ONE_HOUR: i64 = 60;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i64,
}
impl Clock {
    pub fn new(hours: i64, minutes: i64) -> Self {
        Self { minutes: (hours * ONE_HOUR + minutes).rem_euclid(ONE_DAY) }
    }

    pub fn add_minutes(&self, minutes: i64) -> Self {
        Self::new(0, (self.minutes + minutes).rem_euclid(ONE_DAY))
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            self.minutes / ONE_HOUR,
            self.minutes % ONE_HOUR
        )
    }
}
